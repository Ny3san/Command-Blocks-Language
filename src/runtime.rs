use crate::{
    catalog,
    diagnostic::{Error, Result, Span, incompatible, io_error, type_error},
    parser::{Expr, ExprKind, Stmt, StmtKind},
    semantic::Program,
    value::{Type, Value},
};
use std::{
    cell::RefCell,
    collections::{BTreeMap, BTreeSet},
    fs,
    io::{self, Read, Write},
    path::{Path, PathBuf},
    rc::Rc,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

type Env = Rc<RefCell<Scope>>;
struct Scope {
    parent: Option<Env>,
    cells: BTreeMap<String, (Value, Type, bool)>,
}
impl Scope {
    fn new(parent: Option<Env>) -> Env {
        Rc::new(RefCell::new(Self {
            parent,
            cells: BTreeMap::new(),
        }))
    }
    fn owner(env: &Env, name: &str, span: Span) -> Result<Env> {
        if env.borrow().cells.contains_key(name) {
            return Ok(env.clone());
        }
        if let Some(parent) = &env.borrow().parent {
            return Self::owner(parent, name, span);
        }
        Err(Error::new(
            "CB101",
            span,
            "Nome ainda não inicializado.",
            "Name is not initialized yet.",
        ))
    }
}
pub struct Runtime {
    pub program: Program,
    pub root: PathBuf,
    pub allow: BTreeSet<String>,
    pub output: Vec<String>,
    pub echo: bool,
    pub environment: BTreeMap<String, String>,
    pub max_steps: u64,
    pub steps: u64,
    pub bridge_requests: Vec<Instant>,
    pub(crate) discord: Option<crate::discord::Client>,
    globals: Env,
    depth: usize,
    secrets: Vec<String>,
    started: Instant,
    random: u64,
    #[cfg(windows)]
    window: Option<crate::window::Window>,
}
impl Runtime {
    pub fn new(program: Program, root: impl AsRef<Path>) -> Self {
        let environment: BTreeMap<String, String> = std::env::vars().collect();
        let secrets = environment
            .iter()
            .filter(|(k, _)| {
                ["TOKEN", "PASSWORD", "SECRET", "API_KEY"]
                    .iter()
                    .any(|s| k.to_uppercase().contains(s))
            })
            .map(|(_, v)| v.clone())
            .collect();
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos() as u64;
        Self {
            program,
            root: root.as_ref().to_path_buf(),
            allow: BTreeSet::new(),
            output: vec![],
            echo: false,
            environment,
            max_steps: 100000,
            steps: 0,
            bridge_requests: vec![],
            discord: None,
            globals: Scope::new(None),
            depth: 0,
            secrets,
            started: Instant::now(),
            random: seed | 1,
            #[cfg(windows)]
            window: None,
        }
    }
    pub fn redact(&self, text: &str) -> String {
        let mut text = text.to_string();
        let mut secrets: Vec<&str> = self
            .secrets
            .iter()
            .map(|s| s.as_str())
            .chain(
                self.environment
                    .iter()
                    .filter(|(k, _)| {
                        ["TOKEN", "PASSWORD", "SECRET", "API_KEY"]
                            .iter()
                            .any(|s| k.to_uppercase().contains(s))
                    })
                    .map(|(_, v)| v.as_str()),
            )
            .filter(|v| !v.is_empty())
            .collect();
        secrets.sort_by_key(|s| std::cmp::Reverse(s.len()));
        for secret in secrets {
            text = text.replace(secret, "[REDACTED]");
        }
        text
    }
    fn budget(&mut self, span: Span) -> Result<()> {
        self.steps += 1;
        if self.steps > self.max_steps || self.depth > 64 {
            Err(Error::new(
                "CB302",
                span,
                "Limite de execução atingido.",
                "Execution limit reached.",
            )
            .hint(
                "Reduza repetições/recursão ou ajuste --max-steps para código confiável.",
                "Reduce loops/recursion or adjust --max-steps for trusted code.",
            ))
        } else {
            Ok(())
        }
    }
    pub fn run(&mut self) -> Result<()> {
        self.initialize()?;
        self.emit("start")
    }
    pub fn initialize(&mut self) -> Result<()> {
        for stmt in self.program.body.clone() {
            if !matches!(
                stmt.kind,
                StmtKind::Function(..)
                    | StmtKind::Event(..)
                    | StmtKind::Test(..)
                    | StmtKind::Permission(..)
                    | StmtKind::Import(..)
            ) {
                self.stmt(&stmt, &self.globals.clone())?;
            }
        }
        Ok(())
    }
    pub fn run_test(&mut self, body: &[Stmt]) -> Result<()> {
        self.initialize()?;
        self.block(body, &self.globals.clone())?;
        Ok(())
    }
    pub fn emit(&mut self, name: &str) -> Result<()> {
        self.depth += 1;
        let events: Vec<_> = self
            .program
            .body
            .iter()
            .filter_map(|s| match &s.kind {
                StmtKind::Event(n, b) if n == catalog::event(name) => Some((s.span, b.clone())),
                _ => None,
            })
            .collect();
        let result = (|| {
            for (span, body) in events {
                self.budget(span)?;
                self.block(&body, &self.globals.clone())?;
            }
            Ok(())
        })();
        self.depth -= 1;
        result
    }
    fn block(&mut self, body: &[Stmt], parent: &Env) -> Result<Option<Value>> {
        let scope = Scope::new(Some(parent.clone()));
        for stmt in body {
            if let Some(v) = self.stmt(stmt, &scope)? {
                return Ok(Some(v));
            }
        }
        Ok(None)
    }
    fn stmt(&mut self, s: &Stmt, env: &Env) -> Result<Option<Value>> {
        self.budget(s.span)?;
        match &s.kind {
            StmtKind::Declare {
                name,
                value,
                constant,
            } => {
                let v = self.expr(value, env)?;
                env.borrow_mut()
                    .cells
                    .insert(name.clone(), (v, value.ty, *constant));
            }
            StmtKind::Assign(target, value) => {
                let v = self.expr(value, env)?;
                match &target.kind {
                    ExprKind::Name(name) => {
                        let owner = Scope::owner(env, name, s.span)?;
                        let mut owner = owner.borrow_mut();
                        let cell = owner.cells.get_mut(name).unwrap();
                        if cell.2 || !cell.1.accepts(v.ty()) {
                            return Err(type_error(s.span));
                        }
                        cell.0 = v;
                    }
                    ExprKind::Index(base, idx) => {
                        let b = self.expr(base, env)?;
                        let i = self.expr(idx, env)?;
                        let v = snapshot(&v, 0, s.span)?;
                        match (b, i) {
                            (Value::List(list), Value::Int(i))
                                if i >= 0 && (i as usize) < list.borrow().len() =>
                            {
                                list.borrow_mut()[i as usize] = v;
                            }
                            (Value::Map(map), Value::Text(key)) => {
                                map.borrow_mut().insert(key, v);
                            }
                            _ => return Err(index_error(s.span)),
                        }
                    }
                    _ => return Err(type_error(s.span)),
                }
            }
            StmtKind::Expression(e) => {
                self.expr(e, env)?;
            }
            StmtKind::Return(e) => return Ok(Some(self.expr(e, env)?)),
            StmtKind::If(e, yes, no) => {
                let v = self.expr(e, env)?;
                return self.block(if boolean(v, e.span)? { yes } else { no }, env);
            }
            StmtKind::While(e, body) => {
                while {
                    let v = self.expr(e, env)?;
                    boolean(v, e.span)?
                } {
                    self.budget(s.span)?;
                    if let Some(v) = self.block(body, env)? {
                        return Ok(Some(v));
                    }
                }
            }
            StmtKind::Repeat(e, body) => {
                let count = match self.expr(e, env)? {
                    Value::Int(n) if n >= 0 => n,
                    _ => return Err(type_error(e.span)),
                };
                for _ in 0..count {
                    self.budget(s.span)?;
                    if let Some(v) = self.block(body, env)? {
                        return Ok(Some(v));
                    }
                }
            }
            StmtKind::For(name, e, body) => {
                let values = match self.expr(e, env)? {
                    Value::List(v) => v.borrow().clone(),
                    _ => return Err(type_error(e.span)),
                };
                for v in values {
                    self.budget(s.span)?;
                    let scope = Scope::new(Some(env.clone()));
                    scope
                        .borrow_mut()
                        .cells
                        .insert(name.clone(), (v, Type::Any, false));
                    if let Some(v) = self.block(body, &scope)? {
                        return Ok(Some(v));
                    }
                }
            }
            _ => {}
        }
        Ok(None)
    }
    fn expr(&mut self, e: &Expr, env: &Env) -> Result<Value> {
        self.budget(e.span)?;
        let result = match &e.kind {
            ExprKind::Literal(v) => v.clone(),
            ExprKind::Name(name) => Scope::owner(env, name, e.span)?.borrow().cells[name]
                .0
                .clone(),
            ExprKind::List(values) => Value::list(
                values
                    .iter()
                    .map(|e| self.expr(e, env))
                    .collect::<Result<_>>()?,
            ),
            ExprKind::Map(values) => Value::map(
                values
                    .iter()
                    .map(|(k, v)| Ok((k.clone(), self.expr(v, env)?)))
                    .collect::<Result<_>>()?,
            ),
            ExprKind::Index(base, idx) => {
                index(self.expr(base, env)?, self.expr(idx, env)?, e.span)?
            }
            ExprKind::Unary(op, v) => {
                let v = self.expr(v, env)?;
                match (op.as_str(), v) {
                    ("not", Value::Bool(b)) => Value::Bool(!b),
                    ("-", Value::Int(i)) => Value::Int(i.wrapping_neg()),
                    ("-", Value::Number(n)) => Value::Number(-n),
                    ("+", v) if v.number().is_some() => v,
                    _ => return Err(type_error(e.span)),
                }
            }
            ExprKind::Binary(op, a, b) => {
                let a = self.expr(a, env)?;
                if op == "and" {
                    Value::Bool(boolean(a, e.span)? && boolean(self.expr(b, env)?, e.span)?)
                } else if op == "or" {
                    Value::Bool(boolean(a, e.span)? || boolean(self.expr(b, env)?, e.span)?)
                } else {
                    binary(op, a, self.expr(b, env)?, e.span)?
                }
            }
            ExprKind::Call(name, args) => {
                let values = args
                    .iter()
                    .map(|e| self.expr(e, env))
                    .collect::<Result<Vec<_>>>()?;
                if let Some(spec) = catalog::spec(name) {
                    if !spec
                        .args
                        .iter()
                        .zip(&values)
                        .all(|(t, v)| t.accepts(v.ty()))
                    {
                        return Err(type_error(e.span));
                    }
                    if spec.environment == "minecraft" {
                        return Err(incompatible(e.span));
                    }
                    if let Some(p) = spec.permission
                        && !self.allow.contains(p)
                    {
                        return Err(Error::new(
                            "CB301",
                            e.span,
                            "Permissão não autorizada na CLI.",
                            "Permission not granted on the CLI.",
                        )
                        .hint(
                            format!("Autorize código confiável com --allow {p}."),
                            format!("Grant trusted code --allow {p}."),
                        ));
                    }
                    self.builtin(name, &values, e.span)?
                } else {
                    let (params, body) = self
                        .program
                        .body
                        .iter()
                        .find_map(|s| match &s.kind {
                            StmtKind::Function(n, p, b) if n == name => {
                                Some((p.clone(), b.clone()))
                            }
                            _ => None,
                        })
                        .ok_or_else(|| type_error(e.span))?;
                    let scope = Scope::new(Some(self.globals.clone()));
                    for (p, v) in params.into_iter().zip(values) {
                        scope.borrow_mut().cells.insert(p, (v, Type::Any, false));
                    }
                    self.depth += 1;
                    let result = self.block(&body, &scope);
                    self.depth -= 1;
                    result?.unwrap_or(Value::Null)
                }
            }
        };
        if matches!(&result, Value::Number(n) if !n.is_finite())
            || matches!(&result, Value::Text(s) if s.len() > 1_000_000)
        {
            return Err(Error::new(
                "CB302",
                e.span,
                "Valor excede o limite permitido.",
                "Value exceeds the allowed limit.",
            ));
        }
        Ok(result)
    }
    fn path(&self, text: &str, span: Span) -> Result<PathBuf> {
        let root = self.root.canonicalize().map_err(|_| io_error())?;
        let requested = Path::new(text);
        if requested.is_absolute()
            || requested.components().any(|c| {
                matches!(
                    c,
                    std::path::Component::ParentDir | std::path::Component::Prefix(_)
                )
            })
        {
            return Err(Error::new(
                "CB301",
                span,
                "Caminho fora da pasta do projeto.",
                "Path outside the project directory.",
            ));
        }
        let path = root.join(requested);
        let mut existing = path.as_path();
        while !existing.exists() {
            existing = existing.parent().ok_or_else(io_error)?;
        }
        if !existing
            .canonicalize()
            .map_err(|_| io_error())?
            .starts_with(&root)
        {
            return Err(Error::new(
                "CB301",
                span,
                "Caminho fora da pasta do projeto.",
                "Path outside the project directory.",
            ));
        }
        Ok(path)
    }
    fn window_call(&mut self, name: &str, a: &[Value], span: Span) -> Result<Value> {
        #[cfg(not(windows))]
        {
            let _ = (name, a);
            Err(Error::new(
                "CB201",
                span,
                "Janelas nativas disponíveis apenas no Windows nesta versão.",
                "Native windows are available only on Windows in this version.",
            ))
        }
        #[cfg(windows)]
        {
            let text = |i: usize| -> Result<String> {
                match &a[i] {
                    Value::Text(s) => Ok(s.clone()),
                    _ => Err(type_error(span)),
                }
            };
            let int = |i: usize| -> Result<i32> {
                match a[i] {
                    Value::Int(n) => Ok(n),
                    _ => Err(type_error(span)),
                }
            };
            if name == "window.open" {
                if self.window.as_ref().is_some_and(|w| w.is_open()) {
                    return Err(type_error(span));
                }
                let title = self.redact(&text(0)?);
                self.window = Some(crate::window::Window::open(
                    &title,
                    int(1)?,
                    int(2)?,
                    true,
                    span,
                )?);
            } else if name == "window.run" {
                while self.window.as_ref().is_some_and(|w| w.is_open()) {
                    let event = self.window.as_mut().and_then(|w| w.poll());
                    if let Some(event) = event {
                        self.emit(&event)?;
                    }
                    std::thread::sleep(Duration::from_millis(8));
                }
            } else {
                let clean = if matches!(name, "window.label" | "window.input" | "window.button") {
                    Some(self.redact(&text(0)?))
                } else if name == "window.set_text" {
                    Some(self.redact(&text(1)?))
                } else {
                    None
                };
                let window = self.window.as_mut().ok_or_else(|| {
                    Error::new(
                        "CB401",
                        span,
                        "Abra uma janela primeiro.",
                        "Open a window first.",
                    )
                })?;
                match name {
                    "window.close" => window.close(),
                    "window.text" => return Ok(Value::Text(window.text(int(0)?, span)?)),
                    "window.set_text" => {
                        window.set_text(int(0)?, clean.as_deref().unwrap(), span)?
                    }
                    "window.label" | "window.input" | "window.button" => {
                        let button = name == "window.button";
                        let offset = if button { 2 } else { 1 };
                        let event = if button { text(1)? } else { String::new() };
                        let rect = [
                            int(offset)?,
                            int(offset + 1)?,
                            int(offset + 2)?,
                            int(offset + 3)?,
                        ];
                        return Ok(Value::Int(window.control(
                            name.split('.').nth(1).unwrap(),
                            clean.as_deref().unwrap(),
                            &event,
                            rect,
                            span,
                        )?));
                    }
                    _ => return Err(type_error(span)),
                }
            }
            Ok(Value::Null)
        }
    }
    fn builtin(&mut self, name: &str, a: &[Value], span: Span) -> Result<Value> {
        if name.starts_with("window.") {
            return self.window_call(name, a, span);
        }
        use Value::*;
        let text = |i: usize| -> Result<&str> {
            if let Text(s) = &a[i] {
                Ok(s)
            } else {
                Err(type_error(span))
            }
        };
        let num = |i: usize| -> Result<f64> { a[i].number().ok_or_else(|| type_error(span)) };
        if name == "net.get" || name == "net.request" {
            let now = Instant::now();
            self.bridge_requests
                .retain(|t| now.duration_since(*t) < Duration::from_secs(1));
            if self.bridge_requests.len() >= 10 {
                return Err(Error::new(
                    "CB302",
                    span,
                    "Limite de requisições atingido.",
                    "Request rate limit reached.",
                ));
            }
            self.bridge_requests.push(now);
            let mut headers = BTreeMap::new();
            let (method, url, body, timeout) = if name == "net.get" {
                ("GET", text(0)?, "", 10000)
            } else {
                let Value::Map(values) = &a[3] else {
                    return Err(type_error(span));
                };
                for (key, value) in values.borrow().iter() {
                    let value = match value {
                        Text(s) => s.clone(),
                        Secret(s) => s.as_ref().clone(),
                        _ => return Err(type_error(span)),
                    };
                    headers.insert(key.clone(), value);
                }
                let Int(timeout) = a[4] else {
                    return Err(type_error(span));
                };
                (text(0)?, text(1)?, text(2)?, timeout)
            };
            let response = crate::net::request(method, url, body, &headers, timeout, span)?;
            return Ok(Value::map(BTreeMap::from([
                ("status".into(), Int(response.status)),
                ("body".into(), Text(response.body)),
            ])));
        }
        let list = |i: usize| {
            if let List(v) = &a[i] {
                Ok(v.clone())
            } else {
                Err(type_error(span))
            }
        };
        let map = |i: usize| {
            if let Map(v) = &a[i] {
                Ok(v.clone())
            } else {
                Err(type_error(span))
            }
        };
        let out = match name {
            "print" | "text.print" | "io.print" => {
                let s = self.redact(&a[0].display());
                if self.echo {
                    println!("{s}");
                }
                self.output.push(s);
                Null
            }
            "assert" => {
                if !boolean(a[0].clone(), span)? {
                    return Err(Error::new(
                        "CB401",
                        span,
                        "Verificação falhou.",
                        "Assertion failed.",
                    ));
                }
                Null
            }
            "secret" => {
                let value = self
                    .environment
                    .get(text(0)?)
                    .filter(|v| !v.is_empty())
                    .cloned()
                    .ok_or_else(|| {
                        Error::new(
                            "CB303",
                            span,
                            "Segredo ausente no ambiente.",
                            "Secret missing from environment.",
                        )
                    })?;
                self.secrets.push(value.clone());
                Secret(Rc::new(value))
            }
            "json.parse" => crate::json::parse(text(0)?, span)?,
            "json.stringify" => Text(self.redact(&crate::json::stringify(&a[0], span)?)),
            "json.valid" => Bool(crate::json::parse(text(0)?, span).is_ok()),
            "text.upper" => Text(text(0)?.to_uppercase()),
            "text.lower" => Text(text(0)?.to_lowercase()),
            "text.trim" => Text(text(0)?.trim().into()),
            "text.length" => Int(text(0)?.chars().count() as i32),
            "text.string" => Text(self.redact(&a[0].display())),
            "text.number" => Number(text(0)?.parse().map_err(|_| type_error(span))?),
            "text.split" => Value::list(text(0)?.split(text(1)?).map(|s| Text(s.into())).collect()),
            "text.join" => {
                let l = list(0)?;
                let l = l.borrow();
                let parts = l
                    .iter()
                    .map(|v| {
                        if let Text(s) = v {
                            Ok(s.as_str())
                        } else {
                            Err(type_error(span))
                        }
                    })
                    .collect::<Result<Vec<_>>>()?;
                if parts.iter().map(|s| s.len()).sum::<usize>()
                    + text(1)?.len().saturating_mul(parts.len())
                    > 1_000_000
                {
                    return Err(type_error(span));
                }
                Text(parts.join(text(1)?))
            }
            "text.replace" => {
                let s = text(0)?;
                let from = text(1)?;
                let to = text(2)?;
                if s.len()
                    .saturating_add(s.matches(from).count().saturating_mul(to.len()))
                    > 1_000_000
                {
                    return Err(type_error(span));
                }
                Text(s.replace(from, to))
            }
            "text.contains" => Bool(text(0)?.contains(text(1)?)),
            "text.starts_with" => Bool(text(0)?.starts_with(text(1)?)),
            "text.ends_with" => Bool(text(0)?.ends_with(text(1)?)),
            "text.format" => {
                let template = text(0)?;
                let values = map(1)?;
                let values = values.borrow();
                let mut out = String::new();
                let mut rest = template;
                while let Some(start) = rest.find('{') {
                    out.push_str(&rest[..start]);
                    let tail = &rest[start + 1..];
                    let end = tail.find('}').ok_or_else(|| type_error(span))?;
                    out.push_str(
                        &values
                            .get(&tail[..end])
                            .ok_or_else(|| index_error(span))?
                            .display(),
                    );
                    rest = &tail[end + 1..];
                    if out.len() > 1_000_000 {
                        return Err(type_error(span));
                    }
                }
                out.push_str(rest);
                Text(self.redact(&out))
            }
            "math.abs" => Number(num(0)?.abs()),
            "math.round" => Number(num(0)?.round()),
            "math.sqrt" => Number(num(0)?.sqrt()),
            "math.sin" => Number(num(0)?.sin()),
            "math.cos" => Number(num(0)?.cos()),
            "math.tan" => Number(num(0)?.tan()),
            "math.degrees" => Number(num(0)?.to_degrees()),
            "math.radians" => Number(num(0)?.to_radians()),
            "math.min" => Number(num(0)?.min(num(1)?)),
            "math.max" => Number(num(0)?.max(num(1)?)),
            "math.pow" => Number(num(0)?.powf(num(1)?)),
            "math.random" => {
                let (lo, hi) = (num(0)? as i64, num(1)? as i64);
                if lo > hi {
                    return Err(type_error(span));
                }
                self.random ^= self.random << 13;
                self.random ^= self.random >> 7;
                self.random ^= self.random << 17;
                Int((lo + (self.random % ((hi - lo + 1) as u64)) as i64) as i32)
            }
            "list.create" => Value::list(vec![]),
            "list.length" => Int(list(0)?.borrow().len() as i32),
            "list.add" => {
                let l = list(0)?;
                if l.borrow().len() >= 100000 {
                    return Err(type_error(span));
                }
                let v = snapshot(&a[1], 0, span)?;
                l.borrow_mut().push(v);
                Null
            }
            "list.remove" => {
                let l = list(0)?;
                let index = l
                    .borrow()
                    .iter()
                    .position(|v| v.equals(&a[1]))
                    .ok_or_else(|| index_error(span))?;
                l.borrow_mut().remove(index);
                Null
            }
            "list.get" => index(a[0].clone(), a[1].clone(), span)?,
            "list.contains" => Bool(list(0)?.borrow().iter().any(|v| v.equals(&a[1]))),
            "list.sort" => {
                let mut l = list(0)?.borrow().clone();
                if l.iter().all(|v| v.number().is_some()) {
                    l.sort_by(|a, b| a.number().unwrap().total_cmp(&b.number().unwrap()));
                } else if l.iter().all(|v| matches!(v, Text(_))) {
                    l.sort_by_key(|v| v.display());
                } else {
                    return Err(type_error(span));
                }
                Value::list(l)
            }
            "map.create" => Value::map(BTreeMap::new()),
            "map.length" => Int(map(0)?.borrow().len() as i32),
            "map.set" => {
                let m = map(0)?;
                let v = snapshot(&a[2], 0, span)?;
                if m.borrow().len() >= 100000 {
                    return Err(type_error(span));
                }
                m.borrow_mut().insert(text(1)?.into(), v);
                Null
            }
            "map.get" => index(a[0].clone(), a[1].clone(), span)?,
            "map.remove" => {
                let m = map(0)?;
                if m.borrow_mut().remove(text(1)?).is_none() {
                    return Err(index_error(span));
                }
                Null
            }
            "map.contains" => Bool(map(0)?.borrow().contains_key(text(1)?)),
            "map.keys" => Value::list(map(0)?.borrow().keys().map(|s| Text(s.clone())).collect()),
            "event.emit" => {
                self.emit(text(0)?)?;
                Null
            }
            "io.environment" => Text("external".into()),
            "io.read_text" | "io.read_number" => {
                print!("{}", self.redact(text(0)?));
                io::stdout().flush().map_err(|_| io_error())?;
                let mut s = String::new();
                io::stdin()
                    .lock()
                    .take(1_000_001)
                    .read_line_limited(&mut s)?;
                let s = s.trim_end_matches(['\r', '\n']);
                if name == "io.read_number" {
                    Number(s.parse().map_err(|_| type_error(span))?)
                } else {
                    Text(s.into())
                }
            }
            "time.now" => Number(self.started.elapsed().as_secs_f64()),
            "time.sleep" => {
                let seconds = num(0)?;
                if !(0.0..=60.0).contains(&seconds) {
                    return Err(type_error(span));
                }
                std::thread::sleep(Duration::from_secs_f64(seconds));
                Null
            }
            "file.read" | "file.write" | "file.append" | "file.exists" | "file.mkdir"
            | "file.list" => {
                let path = self.path(text(0)?, span)?;
                match name {
                    "file.read" => {
                        let mut s = String::new();
                        fs::File::open(path)
                            .map_err(|_| io_error())?
                            .take(1_000_001)
                            .read_to_string(&mut s)
                            .map_err(|_| io_error())?;
                        Text(s)
                    }
                    "file.write" => {
                        fs::write(path, self.redact(text(1)?)).map_err(|_| io_error())?;
                        Null
                    }
                    "file.append" => {
                        fs::OpenOptions::new()
                            .create(true)
                            .append(true)
                            .open(path)
                            .map_err(|_| io_error())?
                            .write_all(self.redact(text(1)?).as_bytes())
                            .map_err(|_| io_error())?;
                        Null
                    }
                    "file.exists" => Bool(path.exists()),
                    "file.mkdir" => {
                        fs::create_dir_all(path).map_err(|_| io_error())?;
                        Null
                    }
                    _ => {
                        let mut names = fs::read_dir(path)
                            .map_err(|_| io_error())?
                            .take(100001)
                            .map(|e| {
                                e.map(|e| Text(e.file_name().to_string_lossy().into_owned()))
                                    .map_err(|_| io_error())
                            })
                            .collect::<Result<Vec<_>>>()?;
                        if names.len() > 100000 {
                            return Err(type_error(span));
                        }
                        names.sort_by_key(|v| v.display());
                        Value::list(names)
                    }
                }
            }
            "minecraft_bridge.send"
            | "discord.start"
            | "discord.reply"
            | "discord.connect"
            | "discord.send" => {
                return crate::integrations::dispatch(self, name, a, span);
            }
            _ => {
                return Err(Error::new(
                    "CB202",
                    span,
                    "Recurso reservado, ainda não implementado.",
                    "Reserved feature, not implemented yet.",
                ));
            }
        };
        Ok(out)
    }
}
fn boolean(v: Value, span: Span) -> Result<bool> {
    if let Value::Bool(b) = v {
        Ok(b)
    } else {
        Err(type_error(span))
    }
}
fn index_error(span: Span) -> Error {
    Error::new(
        "CB107",
        span,
        "Índice ou chave inválida.",
        "Invalid index or key.",
    )
}
fn index(base: Value, idx: Value, span: Span) -> Result<Value> {
    match (base, idx) {
        (Value::List(l), Value::Int(i)) if i >= 0 => l
            .borrow()
            .get(i as usize)
            .cloned()
            .ok_or_else(|| index_error(span)),
        (Value::Text(s), Value::Int(i)) if i >= 0 => s
            .chars()
            .nth(i as usize)
            .map(|c| Value::Text(c.to_string()))
            .ok_or_else(|| index_error(span)),
        (Value::Map(m), Value::Text(k)) => {
            m.borrow().get(&k).cloned().ok_or_else(|| index_error(span))
        }
        _ => Err(index_error(span)),
    }
}
fn snapshot(v: &Value, depth: usize, span: Span) -> Result<Value> {
    if depth > 16 {
        return Err(Error::new(
            "CB302",
            span,
            "Coleção muito profunda.",
            "Collection is too deeply nested.",
        ));
    }
    match v {
        Value::List(l) => Ok(Value::list(
            l.borrow()
                .iter()
                .map(|v| snapshot(v, depth + 1, span))
                .collect::<Result<_>>()?,
        )),
        Value::Map(m) => Ok(Value::map(
            m.borrow()
                .iter()
                .map(|(k, v)| Ok((k.clone(), snapshot(v, depth + 1, span)?)))
                .collect::<Result<_>>()?,
        )),
        _ => Ok(v.clone()),
    }
}
fn binary(op: &str, a: Value, b: Value, span: Span) -> Result<Value> {
    use Value::*;
    if matches!(a, Secret(_)) || matches!(b, Secret(_)) {
        return Err(type_error(span));
    }
    if op == "==" || op == "!=" {
        return Ok(Bool(a.equals(&b) == (op == "==")));
    }
    if op == "+" && (matches!(a, Text(_)) || matches!(b, Text(_))) {
        if ![Type::Text, Type::Int, Type::Number, Type::Bool].contains(&a.ty())
            || ![Type::Text, Type::Int, Type::Number, Type::Bool].contains(&b.ty())
        {
            return Err(type_error(span));
        }
        return Ok(Text(a.display() + &b.display()));
    }
    let x = a.number().ok_or_else(|| type_error(span))?;
    let y = b.number().ok_or_else(|| type_error(span))?;
    if (op == "/" || op == "%") && y == 0.0 {
        return Err(Error::new(
            "CB108",
            span,
            "Divisão por zero.",
            "Division by zero.",
        ));
    }
    if let (Int(a), Int(b)) = (&a, &b) {
        match op {
            "+" => return Ok(Int(a.wrapping_add(*b))),
            "-" => return Ok(Int(a.wrapping_sub(*b))),
            "*" => return Ok(Int(a.wrapping_mul(*b))),
            "%" => return Ok(Int(if *a == i32::MIN && *b == -1 { 0 } else { a % b })),
            _ => {}
        }
    }
    Ok(match op {
        "+" => Number(x + y),
        "-" => Number(x - y),
        "*" => Number(x * y),
        "/" => Number(x / y),
        "%" => Number(x % y),
        "<" => Bool(x < y),
        ">" => Bool(x > y),
        "<=" => Bool(x <= y),
        ">=" => Bool(x >= y),
        _ => return Err(type_error(span)),
    })
}
trait ReadLineLimited {
    fn read_line_limited(&mut self, out: &mut String) -> Result<()>;
}
impl<R: Read> ReadLineLimited for R {
    fn read_line_limited(&mut self, out: &mut String) -> Result<()> {
        let mut bytes = vec![];
        let mut byte = [0];
        while self.read(&mut byte).map_err(|_| io_error())? != 0 {
            bytes.push(byte[0]);
            if byte[0] == b'\n' {
                break;
            }
        }
        if bytes.len() > 1_000_000 {
            return Err(io_error());
        }
        *out = String::from_utf8(bytes).map_err(|_| io_error())?;
        Ok(())
    }
}
