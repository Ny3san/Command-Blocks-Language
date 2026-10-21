use crate::{
    catalog,
    diagnostic::{Error, Result, Span, type_error},
    parser::{Expr, ExprKind, Stmt, StmtKind},
    value::Type,
};
use std::collections::{BTreeMap, BTreeSet};

/// Typed tree IR. Builtin names are canonical; spans survive lowering.
#[derive(Debug, Clone)]
pub struct Program {
    pub body: Vec<Stmt>,
    pub permissions: BTreeSet<String>,
    pub imports: BTreeSet<String>,
}
type Symbols = BTreeMap<String, (Type, bool)>;
struct Analyzer {
    scopes: Vec<Symbols>,
    functions: BTreeMap<String, usize>,
    permissions: BTreeSet<String>,
    in_function: bool,
}
fn error(span: Span, pt: &str, en: &str) -> Error {
    Error::new("CB101", span, pt, en)
}
pub fn analyze(mut body: Vec<Stmt>) -> Result<Program> {
    let mut a = Analyzer {
        scopes: vec![Symbols::new()],
        functions: BTreeMap::new(),
        permissions: BTreeSet::new(),
        in_function: false,
    };
    let mut imports = BTreeSet::new();
    for s in &mut body {
        match &mut s.kind {
            StmtKind::Function(name, params, _) => {
                if a.functions.insert(name.clone(), params.len()).is_some()
                    || catalog::spec(&catalog::canonical(name)).is_some()
                    || params.iter().collect::<BTreeSet<_>>().len() != params.len()
                {
                    return Err(error(
                        s.span,
                        "Função ou parâmetros duplicados/reservados.",
                        "Duplicate/reserved function or parameters.",
                    ));
                }
            }
            StmtKind::Permission(name) => {
                *name = catalog::canonical(name);
                if !["file", "process", "net", "minecraft_bridge", "window"]
                    .contains(&name.as_str())
                {
                    return Err(error(
                        s.span,
                        "Permissão desconhecida.",
                        "Unknown permission.",
                    ));
                }
                a.permissions.insert(name.clone());
            }
            StmtKind::Import(name) => {
                *name = catalog::canonical(name);
                if !catalog::MODULES.contains(&name.as_str()) {
                    return Err(Error::new(
                        "CB202",
                        s.span,
                        "Biblioteca não implementada.",
                        "Library not implemented.",
                    ));
                }
                imports.insert(name.clone());
            }
            _ => {}
        }
    }
    for s in &mut body {
        if !matches!(
            s.kind,
            StmtKind::Function(..) | StmtKind::Event(..) | StmtKind::Test(..)
        ) {
            a.stmt(s, true)?;
        }
    }
    for s in &mut body {
        if matches!(
            s.kind,
            StmtKind::Function(..) | StmtKind::Event(..) | StmtKind::Test(..)
        ) {
            a.stmt(s, true)?;
        }
    }
    Ok(Program {
        body,
        permissions: a.permissions,
        imports,
    })
}
impl Analyzer {
    fn lookup(&self, name: &str, span: Span) -> Result<(Type, bool)> {
        self.scopes
            .iter()
            .rev()
            .find_map(|s| s.get(name).copied())
            .ok_or_else(|| error(span, "Nome não declarado.", "Undeclared name."))
    }
    fn define(&mut self, name: &str, ty: Type, constant: bool, span: Span) -> Result<()> {
        if self.scopes.last().unwrap().contains_key(name)
            || self.functions.contains_key(name)
            || catalog::spec(&catalog::canonical(name)).is_some()
        {
            return Err(error(
                span,
                "Nome já declarado ou reservado.",
                "Name already declared or reserved.",
            ));
        }
        self.scopes
            .last_mut()
            .unwrap()
            .insert(name.into(), (ty, constant));
        Ok(())
    }
    fn require(&mut self, e: &mut Expr, ty: Type) -> Result<Type> {
        let actual = self.expr(e)?;
        if !ty.accepts(actual) {
            Err(type_error(e.span))
        } else {
            Ok(actual)
        }
    }
    fn block(&mut self, body: &mut [Stmt], initial: Symbols) -> Result<()> {
        self.scopes.push(initial);
        for s in body {
            self.stmt(s, false)?;
        }
        self.scopes.pop();
        Ok(())
    }
    fn stmt(&mut self, s: &mut Stmt, top: bool) -> Result<()> {
        if !top
            && matches!(
                s.kind,
                StmtKind::Function(..)
                    | StmtKind::Event(..)
                    | StmtKind::Test(..)
                    | StmtKind::Import(..)
                    | StmtKind::Permission(..)
            )
        {
            return Err(error(
                s.span,
                "Declaração permitida apenas no nível principal.",
                "Declaration allowed only at top level.",
            ));
        }
        match &mut s.kind {
            StmtKind::Declare {
                name,
                constant,
                value,
            } => {
                let ty = self.expr(value)?;
                self.define(name, ty, *constant, s.span)?;
            }
            StmtKind::Assign(target, value) => match &target.kind {
                ExprKind::Name(name) => {
                    let (ty, constant) = self.lookup(name, target.span)?;
                    if constant {
                        return Err(Error::new(
                            "CB104",
                            s.span,
                            "Não é possível reatribuir uma constante.",
                            "Cannot reassign a constant.",
                        ));
                    }
                    target.ty = ty;
                    self.require(value, ty)?;
                }
                ExprKind::Index(..) => {
                    self.expr(target)?;
                    self.expr(value)?;
                }
                _ => return Err(type_error(s.span)),
            },
            StmtKind::Expression(e) => {
                self.expr(e)?;
            }
            StmtKind::If(e, yes, no) => {
                self.require(e, Type::Bool)?;
                self.block(yes, Symbols::new())?;
                self.block(no, Symbols::new())?;
            }
            StmtKind::While(e, body) => {
                self.require(e, Type::Bool)?;
                self.block(body, Symbols::new())?;
            }
            StmtKind::Repeat(e, body) => {
                self.require(e, Type::Int)?;
                self.block(body, Symbols::new())?;
            }
            StmtKind::For(name, e, body) => {
                self.require(e, Type::List)?;
                self.block(body, BTreeMap::from([(name.clone(), (Type::Any, false))]))?;
            }
            StmtKind::Function(_, params, body) => {
                self.in_function = true;
                self.block(
                    body,
                    params
                        .iter()
                        .map(|p| (p.clone(), (Type::Any, false)))
                        .collect(),
                )?;
                self.in_function = false;
            }
            StmtKind::Return(e) => {
                if !self.in_function {
                    return Err(error(
                        s.span,
                        "retornar exige uma função.",
                        "return requires a function.",
                    ));
                }
                self.expr(e)?;
            }
            StmtKind::Event(name, body) => {
                *name = catalog::event(name).into();
                self.block(body, Symbols::new())?;
            }
            StmtKind::Test(_, body) => self.block(body, Symbols::new())?,
            StmtKind::Permission(_) | StmtKind::Import(_) => {}
        }
        Ok(())
    }
    fn expr(&mut self, e: &mut Expr) -> Result<Type> {
        use Type::*;
        let ty = match &mut e.kind {
            ExprKind::Literal(v) => v.ty(),
            ExprKind::Name(name) => self.lookup(name, e.span)?.0,
            ExprKind::List(values) => {
                for v in values {
                    self.expr(v)?;
                }
                List
            }
            ExprKind::Map(values) => {
                for (_, v) in values {
                    self.expr(v)?;
                }
                Map
            }
            ExprKind::Index(base, index) => {
                let ty = self.expr(base)?;
                match ty {
                    List | Text => {
                        self.require(index, Int)?;
                    }
                    Map => {
                        self.require(index, Text)?;
                    }
                    Any => {
                        self.expr(index)?;
                    }
                    _ => return Err(type_error(e.span)),
                }
                if ty == Text { Text } else { Any }
            }
            ExprKind::Unary(op, value) => {
                self.require(value, if op == "not" { Bool } else { Number })?
            }
            ExprKind::Binary(op, left, right) => {
                let (a, b) = (self.expr(left)?, self.expr(right)?);
                match op.as_str() {
                    "and" | "or" => {
                        self.require(left, Bool)?;
                        self.require(right, Bool)?;
                        Bool
                    }
                    "==" | "!=" => Bool,
                    "+" if a == Text || b == Text => {
                        if ![Text, Int, Number, Bool, Any].contains(&a)
                            || ![Text, Int, Number, Bool, Any].contains(&b)
                        {
                            return Err(type_error(e.span));
                        }
                        Text
                    }
                    _ => {
                        self.require(left, Number)?;
                        self.require(right, Number)?;
                        if ["<", ">", "<=", ">="].contains(&op.as_str()) {
                            Bool
                        } else if op == "/" || a == Number || b == Number {
                            Number
                        } else if a == Any || b == Any {
                            Any
                        } else {
                            Int
                        }
                    }
                }
            }
            ExprKind::Call(name, args) => {
                *name = catalog::canonical(name);
                if let Some(spec) = catalog::spec(name) {
                    if args.len() != spec.args.len() {
                        return Err(Error::new(
                            "CB105",
                            e.span,
                            "Quantidade incorreta de argumentos.",
                            "Incorrect number of arguments.",
                        ));
                    }
                    for (arg, ty) in args.iter_mut().zip(spec.args) {
                        self.require(arg, ty)?;
                    }
                    if let Some(permission) = spec.permission
                        && !self.permissions.contains(permission)
                    {
                        return Err(Error::new("CB301", e.span, "Permissão ausente no programa.", "Missing program permission.").hint(
                                format!("Declare permissao {permission} e autorize com --allow {permission}."),
                                format!("Declare permission {permission} and grant --allow {permission}.")));
                    }
                    spec.result
                } else if let Some(count) = self.functions.get(name) {
                    if args.len() != *count {
                        return Err(Error::new(
                            "CB105",
                            e.span,
                            "Quantidade incorreta de argumentos.",
                            "Incorrect number of arguments.",
                        ));
                    }
                    for arg in args {
                        self.expr(arg)?;
                    }
                    Any
                } else {
                    return Err(Error::new(
                        "CB102",
                        e.span,
                        "Função desconhecida ou não implementada.",
                        "Unknown or unimplemented function.",
                    ));
                }
            }
        };
        e.ty = ty;
        Ok(ty)
    }
}

pub fn expressions(stmt: &Stmt, visit: &mut impl FnMut(&Expr) -> Result<()>) -> Result<()> {
    fn walk(e: &Expr, visit: &mut impl FnMut(&Expr) -> Result<()>) -> Result<()> {
        visit(e)?;
        match &e.kind {
            ExprKind::Unary(_, v) => walk(v, visit)?,
            ExprKind::Binary(_, a, b) | ExprKind::Index(a, b) => {
                walk(a, visit)?;
                walk(b, visit)?;
            }
            ExprKind::Call(_, args) | ExprKind::List(args) => {
                for a in args {
                    walk(a, visit)?;
                }
            }
            ExprKind::Map(args) => {
                for (_, a) in args {
                    walk(a, visit)?;
                }
            }
            _ => {}
        }
        Ok(())
    }
    match &stmt.kind {
        StmtKind::Declare { value, .. } | StmtKind::Expression(value) | StmtKind::Return(value) => {
            walk(value, visit)?
        }
        StmtKind::Assign(a, b) => {
            walk(a, visit)?;
            walk(b, visit)?;
        }
        StmtKind::If(e, a, b) => {
            walk(e, visit)?;
            for s in a.iter().chain(b) {
                expressions(s, visit)?;
            }
        }
        StmtKind::While(e, b) | StmtKind::Repeat(e, b) | StmtKind::For(_, e, b) => {
            walk(e, visit)?;
            for s in b {
                expressions(s, visit)?;
            }
        }
        StmtKind::Function(_, _, b) | StmtKind::Event(_, b) | StmtKind::Test(_, b) => {
            for s in b {
                expressions(s, visit)?;
            }
        }
        _ => {}
    }
    Ok(())
}
pub fn check_external(program: &Program) -> Result<()> {
    for s in &program.body {
        expressions(s, &mut |e| {
            if let ExprKind::Call(name, _) = &e.kind
                && catalog::spec(name).is_some_and(|s| s.environment == "minecraft")
            {
                return Err(crate::diagnostic::incompatible(e.span));
            }
            Ok(())
        })?;
        if matches!(&s.kind, StmtKind::Event(name,_) if name == "tick") {
            return Err(crate::diagnostic::incompatible(s.span));
        }
    }
    Ok(())
}
