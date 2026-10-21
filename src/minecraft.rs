use crate::{
    catalog,
    diagnostic::{Error, Result, Span, incompatible, io_error},
    parser::{Expr, ExprKind, Stmt, StmtKind},
    semantic::{Program, expressions},
    value::{Type, Value, quote},
};
use std::{
    collections::BTreeMap,
    fs,
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

pub type Files = BTreeMap<String, String>;
pub struct Backend {
    program: Program,
    namespace: String,
    version: String,
    objective: String,
    scores: BTreeMap<String, String>,
    strings: BTreeMap<String, String>,
    functions: BTreeMap<String, String>,
    files: BTreeMap<String, Vec<String>>,
    counter: u32,
}
impl Backend {
    pub fn new(program: Program, namespace: &str, version: &str) -> Result<Self> {
        if !["1.21", "1.21.1"].contains(&version) {
            return Err(Error::new(
                "CB203",
                Span::default(),
                "Versão Minecraft não suportada.",
                "Unsupported Minecraft version.",
            )
            .hint(
                "Use --mc-version 1.21 ou 1.21.1.",
                "Use --mc-version 1.21 or 1.21.1.",
            ));
        }
        if namespace.is_empty()
            || namespace.len() > 40
            || namespace == "minecraft"
            || !namespace.as_bytes()[0].is_ascii_lowercase()
            || !namespace
                .bytes()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == b'_')
        {
            return Err(Error::new(
                "CB204",
                Span::default(),
                "Namespace inválido ou reservado.",
                "Invalid or reserved namespace.",
            ));
        }
        let hash = namespace.bytes().fold(0xcbf29ce484222325u64, |a, b| {
            (a ^ b as u64).wrapping_mul(0x100000001b3)
        });
        Ok(Self {
            program,
            namespace: namespace.into(),
            version: version.into(),
            objective: format!("cb{:012x}", hash & 0xffffffffffff),
            scores: BTreeMap::new(),
            strings: BTreeMap::new(),
            functions: BTreeMap::new(),
            files: BTreeMap::new(),
            counter: 0,
        })
    }
    fn temp(&mut self) -> String {
        self.counter += 1;
        format!("#t{}", self.counter)
    }
    fn score(&self, value: &str) -> String {
        format!("{value} {}", self.objective)
    }
    fn set(&self, value: &str, n: i32) -> String {
        format!("scoreboard players set {} {n}", self.score(value))
    }
    fn op(&self, a: &str, op: &str, b: &str) -> String {
        format!(
            "scoreboard players operation {} {op} {}",
            self.score(a),
            self.score(b)
        )
    }
    fn reference(&self, name: &str) -> String {
        format!("{}:{name}", self.namespace)
    }
    fn validate(&mut self) -> Result<()> {
        let mut executable = false;
        for s in &self.program.body {
            match &s.kind {
                StmtKind::Function(name, params, _) => {
                    if !params.is_empty() {
                        return Err(incompatible(s.span));
                    }
                    self.functions
                        .insert(name.clone(), format!("user/f{}", self.functions.len()));
                }
                StmtKind::Declare {
                    name,
                    constant,
                    value,
                } => {
                    if executable {
                        return Err(incompatible(s.span).hint(
                            "Declare globais antes das instruções executáveis.",
                            "Declare globals before executable statements.",
                        ));
                    }
                    match (&value.kind, value.ty) {
                        (ExprKind::Literal(Value::Text(text)), Type::Text) if *constant => {
                            self.strings.insert(name.clone(), text.clone());
                        }
                        (_, Type::Int | Type::Bool) => {
                            self.scores
                                .insert(name.clone(), format!("#v{}", self.scores.len()));
                        }
                        _ => return Err(incompatible(s.span)),
                    }
                }
                StmtKind::Event(..) | StmtKind::Permission(..) | StmtKind::Import(..) => {}
                _ => executable = true,
            }
            self.validate_statement(s, true)?;
            expressions(s, &mut |e| {
                if let ExprKind::Call(name, _) = &e.kind {
                    if let Some(spec) = catalog::spec(name) {
                        if spec.environment == "external"
                            || ![
                                "print",
                                "text.print",
                                "io.print",
                                "minecraft.message",
                                "minecraft.give_item",
                                "minecraft.teleport",
                                "minecraft.weather",
                                "minecraft.time",
                            ]
                            .contains(&name.as_str())
                        {
                            return Err(incompatible(e.span));
                        }
                    }
                }
                Ok(())
            })?;
        }
        let mut total = 0u64;
        for s in &self.program.body {
            total = total.saturating_add(self.cost(s, &[])?);
        }
        if total > 4000 {
            return Err(incompatible(Span::default()).hint(
                "Reduza o programa; limite estático de 4000 unidades por carga/tick.",
                "Reduce the program; static limit is 4000 units per load/tick.",
            ));
        }
        Ok(())
    }
    fn validate_statement(&self, s: &Stmt, top: bool) -> Result<()> {
        match &s.kind {
            StmtKind::While(..) | StmtKind::For(..) | StmtKind::Return(..) | StmtKind::Test(..) => {
                return Err(incompatible(s.span));
            }
            StmtKind::Declare { .. } if !top => return Err(incompatible(s.span)),
            StmtKind::Repeat(e, b) => {
                if !matches!(&e.kind,ExprKind::Literal(Value::Int(n)) if (0..=1000).contains(n)) {
                    return Err(incompatible(e.span));
                }
                for s in b {
                    self.validate_statement(s, false)?;
                }
            }
            StmtKind::If(_, a, b) => {
                for s in a.iter().chain(b) {
                    self.validate_statement(s, false)?;
                }
            }
            StmtKind::Function(_, _, b) => {
                for s in b {
                    self.validate_statement(s, false)?;
                }
            }
            StmtKind::Event(name, b) => {
                if name != "start" && name != "tick" {
                    return Err(incompatible(s.span));
                }
                for s in b {
                    self.validate_statement(s, false)?;
                }
            }
            _ => {}
        }
        Ok(())
    }
    fn cost(&self, s: &Stmt, stack: &[String]) -> Result<u64> {
        if stack.len() > 64 {
            return Err(incompatible(s.span));
        }
        let mut cost = 1u64;
        match &s.kind {
            StmtKind::Expression(Expr {
                kind: ExprKind::Call(name, _),
                ..
            }) if self.functions.contains_key(name) => {
                if stack.contains(name) {
                    return Err(incompatible(s.span).hint(
                        "Remova recursão do datapack.",
                        "Remove recursion from the datapack.",
                    ));
                }
                let mut next = stack.to_vec();
                next.push(name.clone());
                if let Some(Stmt {
                    kind: StmtKind::Function(_, _, b),
                    ..
                }) = self
                    .program
                    .body
                    .iter()
                    .find(|s| matches!(&s.kind,StmtKind::Function(n,_,_) if n == name))
                {
                    for s in b {
                        cost = cost.saturating_add(self.cost(s, &next)?);
                    }
                }
            }
            StmtKind::Repeat(e, b) => {
                for s in b {
                    cost = cost.saturating_add(self.cost(s, stack)?);
                }
                if let ExprKind::Literal(Value::Int(n)) = e.kind {
                    cost = cost.saturating_mul(n as u64);
                }
            }
            StmtKind::If(_, a, b) => {
                for s in a.iter().chain(b) {
                    cost = cost.saturating_add(self.cost(s, stack)?);
                }
            }
            StmtKind::Function(_, _, b) | StmtKind::Event(_, b) => {
                for s in b {
                    cost = cost.saturating_add(self.cost(s, stack)?);
                }
            }
            _ => {
                expressions(s, &mut |_| {
                    cost += 1;
                    Ok(())
                })?;
            }
        }
        if cost > 4000 {
            return Err(incompatible(s.span));
        }
        Ok(cost)
    }
    fn numeric(&mut self, e: &Expr, out: &mut Vec<String>) -> Result<String> {
        if e.ty != Type::Int && e.ty != Type::Bool {
            return Err(incompatible(e.span));
        }
        match &e.kind {
            ExprKind::Literal(Value::Int(n)) => {
                let t = self.temp();
                out.push(self.set(&t, *n));
                Ok(t)
            }
            ExprKind::Literal(Value::Bool(b)) => {
                let t = self.temp();
                out.push(self.set(&t, *b as i32));
                Ok(t)
            }
            ExprKind::Name(n) => self
                .scores
                .get(n)
                .cloned()
                .ok_or_else(|| incompatible(e.span)),
            ExprKind::Unary(op, v) => {
                let value = self.numeric(v, out)?;
                let t = self.temp();
                out.push(self.set(&t, 0));
                if op == "not" {
                    out.push(format!(
                        "execute if score {} matches 0 run {}",
                        self.score(&value),
                        self.set(&t, 1)
                    ));
                } else {
                    out.push(self.op(&t, if op == "-" { "-=" } else { "=" }, &value));
                }
                Ok(t)
            }
            ExprKind::Binary(op, a, b) => {
                let left = self.numeric(a, out)?;
                let right = self.numeric(b, out)?;
                let t = self.temp();
                match op.as_str() {
                    "+" | "-" | "*" => {
                        out.push(self.op(&t, "=", &left));
                        out.push(self.op(&t, &format!("{op}="), &right));
                    }
                    "==" | "!=" | "<" | ">" | "<=" | ">=" => {
                        out.push(self.set(&t, 0));
                        if (op == "==" || op == "!=") && a.ty != b.ty {
                            out.push(self.set(&t, (op == "!=") as i32));
                        } else {
                            out.push(format!(
                                "execute {} score {} {} {} run {}",
                                if op == "!=" { "unless" } else { "if" },
                                self.score(&left),
                                if op == "==" || op == "!=" { "=" } else { op },
                                self.score(&right),
                                self.set(&t, 1)
                            ));
                        }
                    }
                    "and" | "or" => {
                        out.push(self.set(&t, 0));
                        if op == "and" {
                            out.push(format!(
                                "execute if score {} matches 1 if score {} matches 1 run {}",
                                self.score(&left),
                                self.score(&right),
                                self.set(&t, 1)
                            ));
                        } else {
                            for v in [&left, &right] {
                                out.push(format!(
                                    "execute if score {} matches 1 run {}",
                                    self.score(v),
                                    self.set(&t, 1)
                                ));
                            }
                        }
                    }
                    _ => return Err(incompatible(e.span)),
                }
                Ok(t)
            }
            _ => Err(incompatible(e.span)),
        }
    }
    fn components(&mut self, e: &Expr, out: &mut Vec<String>) -> Result<Vec<String>> {
        match &e.kind {
            ExprKind::Literal(Value::Text(text)) => {
                Ok(vec![format!("{{\"text\":{}}}", quote(text))])
            }
            ExprKind::Literal(Value::Bool(b)) => {
                Ok(vec![format!("{{\"text\":{}}}", quote(&b.to_string()))])
            }
            ExprKind::Name(name) if self.strings.contains_key(name) => {
                Ok(vec![format!("{{\"text\":{}}}", quote(&self.strings[name]))])
            }
            ExprKind::Binary(op, a, b) if op == "+" && e.ty == Type::Text => {
                let mut c = self.components(a, out)?;
                c.extend(self.components(b, out)?);
                Ok(c)
            }
            _ if e.ty == Type::Int => {
                let v = self.numeric(e, out)?;
                Ok(vec![format!(
                    "{{\"score\":{{\"name\":{},\"objective\":{}}}}}",
                    quote(&v),
                    quote(&self.objective)
                )])
            }
            _ => Err(incompatible(e.span)),
        }
    }
    fn literal(&self, e: &Expr) -> Result<Value> {
        match &e.kind {
            ExprKind::Literal(v) => Ok(v.clone()),
            ExprKind::Name(n) if self.strings.contains_key(n) => {
                Ok(Value::Text(self.strings[n].clone()))
            }
            ExprKind::Unary(op, v) if op == "-" => match self.literal(v)? {
                Value::Int(n) => Ok(Value::Int(n.wrapping_neg())),
                Value::Number(n) => Ok(Value::Number(-n)),
                _ => Err(incompatible(e.span)),
            },
            _ => Err(incompatible(e.span)),
        }
    }
    fn selector(&self, e: &Expr) -> Result<String> {
        if let Value::Text(s) = self.literal(e)? {
            if ["@a", "@p", "@r", "@s", "@e"].contains(&s.as_str())
                || !s.is_empty()
                    && s.len() <= 16
                    && s.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_')
            {
                return Ok(s);
            }
        }
        Err(incompatible(e.span))
    }
    fn helper(&mut self, body: &[Stmt]) -> Result<String> {
        let name = format!("internal/{}", self.temp().trim_start_matches('#'));
        let mut lines = vec![];
        for s in body {
            self.stmt(s, &mut lines)?;
        }
        self.files.insert(name.clone(), lines);
        Ok(self.reference(&name))
    }
    fn stmt(&mut self, s: &Stmt, out: &mut Vec<String>) -> Result<()> {
        match &s.kind {
            StmtKind::Declare { name, value, .. } => {
                if let Some(score) = self.scores.get(name).cloned() {
                    let v = self.numeric(value, out)?;
                    out.push(self.op(&score, "=", &v));
                }
            }
            StmtKind::Assign(target, value) => {
                let name = match &target.kind {
                    ExprKind::Name(n) => n,
                    _ => return Err(incompatible(s.span)),
                };
                let score = self
                    .scores
                    .get(name)
                    .cloned()
                    .ok_or_else(|| incompatible(s.span))?;
                let v = self.numeric(value, out)?;
                out.push(self.op(&score, "=", &v));
            }
            StmtKind::If(e, yes, no) => {
                let v = self.numeric(e, out)?;
                let frozen = self.temp();
                out.push(self.op(&frozen, "=", &v));
                let yes = self.helper(yes)?;
                out.push(format!(
                    "execute if score {} matches 1 run function {yes}",
                    self.score(&frozen)
                ));
                if !no.is_empty() {
                    let no = self.helper(no)?;
                    out.push(format!(
                        "execute if score {} matches 0 run function {no}",
                        self.score(&frozen)
                    ));
                }
            }
            StmtKind::Repeat(e, body) => {
                let f = self.helper(body)?;
                if let ExprKind::Literal(Value::Int(n)) = e.kind {
                    for _ in 0..n {
                        out.push(format!("function {f}"));
                    }
                }
            }
            StmtKind::Expression(e) => {
                let (name, args) = match &e.kind {
                    ExprKind::Call(n, a) => (n, a),
                    _ => return Err(incompatible(e.span)),
                };
                if let Some(f) = self.functions.get(name) {
                    out.push(format!("function {}", self.reference(f)));
                } else {
                    match name.as_str() {
                        "print" | "text.print" | "io.print" | "minecraft.message" => {
                            let (target, message) = if name == "minecraft.message" {
                                (self.selector(&args[0])?, &args[1])
                            } else {
                                ("@a".into(), &args[0])
                            };
                            let parts = self.components(message, out)?;
                            out.push(format!(
                                "tellraw {target} [{{\"text\":\"\"}},{}]",
                                parts.join(",")
                            ));
                        }
                        "minecraft.give_item" => {
                            let target = self.selector(&args[0])?;
                            let item = match self.literal(&args[1])? {
                                Value::Text(s) => s,
                                _ => return Err(incompatible(e.span)),
                            };
                            let count = match self.literal(&args[2])? {
                                Value::Int(n) if (1..=6400).contains(&n) => n,
                                _ => return Err(incompatible(e.span)),
                            };
                            if ![
                                "minecraft:diamond",
                                "minecraft:stone",
                                "minecraft:dirt",
                                "minecraft:apple",
                                "minecraft:stick",
                                "minecraft:iron_ingot",
                                "minecraft:gold_ingot",
                                "minecraft:emerald",
                                "minecraft:diamond_sword",
                                "minecraft:bow",
                                "minecraft:arrow",
                            ]
                            .contains(&item.as_str())
                            {
                                return Err(incompatible(e.span));
                            }
                            out.push(format!("give {target} {item} {count}"));
                        }
                        "minecraft.teleport" => {
                            let target = self.selector(&args[0])?;
                            let mut coords = vec![];
                            for a in &args[1..] {
                                let v = self.literal(a)?;
                                let n = v.number().ok_or_else(|| incompatible(a.span))?;
                                if !n.is_finite() || n.abs() > 30_000_000.0 {
                                    return Err(incompatible(a.span));
                                }
                                coords.push(v.display());
                            }
                            out.push(format!("tp {target} {}", coords.join(" ")));
                        }
                        "minecraft.weather" => {
                            let weather = match self.literal(&args[0])? {
                                Value::Text(s)
                                    if ["clear", "rain", "thunder"].contains(&s.as_str()) =>
                                {
                                    s
                                }
                                _ => return Err(incompatible(e.span)),
                            };
                            out.push(format!("weather {weather}"));
                        }
                        "minecraft.time" => {
                            let n = match self.literal(&args[0])? {
                                Value::Int(n) if n >= 0 => n,
                                _ => return Err(incompatible(e.span)),
                            };
                            out.push(format!("time set {n}"));
                        }
                        _ => return Err(incompatible(e.span)),
                    }
                }
            }
            StmtKind::Function(..)
            | StmtKind::Event(..)
            | StmtKind::Permission(..)
            | StmtKind::Import(..) => {}
            _ => return Err(incompatible(s.span)),
        }
        Ok(())
    }
    pub fn generate(mut self) -> Result<Files> {
        self.validate()?;
        let mut load = vec![format!(
            "scoreboard objectives add {} dummy",
            self.objective
        )];
        let mut tick = vec![];
        for s in self.program.body.clone() {
            if let StmtKind::Function(name, _, body) = &s.kind {
                let mut lines = vec![];
                for s in body {
                    self.stmt(s, &mut lines)?;
                }
                self.files.insert(self.functions[name].clone(), lines);
            } else {
                self.stmt(&s, &mut load)?;
            }
        }
        for s in self.program.body.clone() {
            if let StmtKind::Event(name, body) = &s.kind {
                let f = self.helper(body)?;
                if name == "start" {
                    load.push(format!("function {f}"));
                } else {
                    tick.push(format!("function {f}"));
                }
            }
        }
        self.files.insert("load".into(), load);
        self.files.insert("tick".into(), tick);
        let mut files: Files = self
            .files
            .iter()
            .map(|(name, lines)| {
                (
                    format!("data/{}/function/{name}.mcfunction", self.namespace),
                    lines.join("\n") + "\n",
                )
            })
            .collect();
        for name in ["load", "tick"] {
            files.insert(
                format!("data/minecraft/tags/function/{name}.json"),
                format!("{{\"values\":[{}]}}\n", quote(&self.reference(name))),
            );
        }
        files.insert(
            "pack.mcmeta".into(),
            "{\"pack\":{\"pack_format\":48,\"description\":\"Command Blocks 0.1\"}}\n".into(),
        );
        let mapping = self
            .scores
            .iter()
            .map(|(k, v)| format!("{}:{}", quote(k), quote(v)))
            .collect::<Vec<_>>()
            .join(",");
        files.insert("cb-report.json".into(),format!("{{\n  \"compiler\":\"0.1.0\",\n  \"target\":\"minecraft-java\",\n  \"version\":{},\n  \"namespace\":{},\n  \"objective\":{},\n  \"functions\":{},\n  \"scores\":{{{mapping}}},\n  \"validation\":\"compiler-only; no live game verification\",\n  \"reload\":\"globals reset; existing objective is retained\"\n}}\n",quote(&self.version),quote(&self.namespace),quote(&self.objective),self.files.len()));
        Ok(files)
    }
}
pub fn publish(files: &Files, output: &Path) -> Result<()> {
    if output.exists() {
        return Err(Error::new(
            "CB205",
            Span::default(),
            "O destino já existe.",
            "Output already exists.",
        )
        .hint("Escolha uma pasta nova.", "Choose a new directory."));
    }
    let parent = output
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or(Path::new("."));
    fs::create_dir_all(parent).map_err(|_| io_error())?;
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let temp = parent.join(format!(".cb-{}-{nonce}", std::process::id()));
    fs::create_dir(&temp).map_err(|_| io_error())?;
    let result = (|| {
        for (name, content) in files {
            let rel = Path::new(name);
            if rel.is_absolute()
                || rel
                    .components()
                    .any(|c| !matches!(c, std::path::Component::Normal(_)))
            {
                return Err(io_error());
            }
            let path = temp.join(rel);
            fs::create_dir_all(path.parent().unwrap()).map_err(|_| io_error())?;
            fs::write(path, content).map_err(|_| io_error())?;
        }
        if output.exists() {
            return Err(io_error());
        }
        fs::rename(&temp, output).map_err(|_| io_error())?;
        Ok(())
    })();
    if temp.exists() {
        let _ = fs::remove_dir_all(&temp);
    }
    result
}
