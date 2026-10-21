use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Int,
    Number,
    Text,
    Bool,
    List,
    Map,
    Null,
    Secret,
    Any,
}
impl Type {
    pub fn accepts(self, actual: Self) -> bool {
        self == Self::Any
            || actual == Self::Any
            || self == actual
            || self == Self::Number && actual == Self::Int
    }
}

#[derive(Clone)]
pub enum Value {
    Int(i32),
    Number(f64),
    Text(String),
    Bool(bool),
    Null,
    List(Rc<RefCell<Vec<Value>>>),
    Map(Rc<RefCell<BTreeMap<String, Value>>>),
    Secret(Rc<String>),
}
impl std::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display())
    }
}
impl Value {
    pub fn ty(&self) -> Type {
        match self {
            Self::Int(_) => Type::Int,
            Self::Number(_) => Type::Number,
            Self::Text(_) => Type::Text,
            Self::Bool(_) => Type::Bool,
            Self::Null => Type::Null,
            Self::List(_) => Type::List,
            Self::Map(_) => Type::Map,
            Self::Secret(_) => Type::Secret,
        }
    }
    pub fn list(values: Vec<Value>) -> Self {
        Self::List(Rc::new(RefCell::new(values)))
    }
    pub fn map(values: BTreeMap<String, Value>) -> Self {
        Self::Map(Rc::new(RefCell::new(values)))
    }
    pub fn number(&self) -> Option<f64> {
        match self {
            Self::Int(v) => Some(*v as f64),
            Self::Number(v) => Some(*v),
            _ => None,
        }
    }
    pub fn display(&self) -> String {
        if let Self::Text(s) = self {
            s.clone()
        } else {
            self.json(0)
        }
    }
    pub fn json(&self, depth: usize) -> String {
        if depth > 16 {
            return quote("[depth limit]");
        }
        match self {
            Self::Int(v) => v.to_string(),
            Self::Number(v) => v.to_string(),
            Self::Bool(v) => v.to_string(),
            Self::Null => "null".into(),
            Self::Text(v) => quote(v),
            Self::Secret(_) => quote("[REDACTED]"),
            Self::List(v) => format!(
                "[{}]",
                v.borrow()
                    .iter()
                    .map(|v| v.json(depth + 1))
                    .collect::<Vec<_>>()
                    .join(",")
            ),
            Self::Map(v) => format!(
                "{{{}}}",
                v.borrow()
                    .iter()
                    .map(|(k, v)| format!("{}:{}", quote(k), v.json(depth + 1)))
                    .collect::<Vec<_>>()
                    .join(",")
            ),
        }
    }
    pub fn equals(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Int(a), Self::Int(b)) => a == b,
            (a, b) if a.number().is_some() && b.number().is_some() => a.number() == b.number(),
            (Self::Text(a), Self::Text(b)) => a == b,
            (Self::Bool(a), Self::Bool(b)) => a == b,
            (Self::Null, Self::Null) => true,
            (Self::List(a), Self::List(b)) => Rc::ptr_eq(a, b),
            (Self::Map(a), Self::Map(b)) => Rc::ptr_eq(a, b),
            _ => false,
        }
    }
}
pub fn quote(text: &str) -> String {
    let mut out = String::from("\"");
    for c in text.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if c < ' ' => out.push_str(&format!("\\u{:04x}", c as u32)),
            _ => out.push(c),
        }
    }
    out.push('"');
    out
}
