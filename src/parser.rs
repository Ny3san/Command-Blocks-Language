use crate::{
    diagnostic::{Error, Result, Span},
    lexer::{Token, TokenKind, keyword},
    value::{Type, Value},
};

#[derive(Debug, Clone)]
pub struct Expr {
    pub kind: ExprKind,
    pub span: Span,
    pub ty: Type,
}
#[derive(Debug, Clone)]
pub enum ExprKind {
    Literal(Value),
    Name(String),
    List(Vec<Expr>),
    Map(Vec<(String, Expr)>),
    Unary(String, Box<Expr>),
    Binary(String, Box<Expr>, Box<Expr>),
    Call(String, Vec<Expr>),
    Index(Box<Expr>, Box<Expr>),
}
#[derive(Debug, Clone)]
pub struct Stmt {
    pub kind: StmtKind,
    pub span: Span,
}
#[derive(Debug, Clone)]
pub enum StmtKind {
    Declare {
        name: String,
        constant: bool,
        value: Expr,
    },
    Assign(Expr, Expr),
    Expression(Expr),
    If(Expr, Vec<Stmt>, Vec<Stmt>),
    While(Expr, Vec<Stmt>),
    Repeat(Expr, Vec<Stmt>),
    For(String, Expr, Vec<Stmt>),
    Function(String, Vec<String>, Vec<Stmt>),
    Return(Expr),
    Event(String, Vec<Stmt>),
    Test(String, Vec<Stmt>),
    Permission(String),
    Import(String),
}

pub fn parse(tokens: Vec<Token>) -> Result<Vec<Stmt>> {
    Parser {
        tokens,
        i: 0,
        depth: 0,
    }
    .statements(false)
}
struct Parser {
    tokens: Vec<Token>,
    i: usize,
    depth: usize,
}
impl Parser {
    fn current(&self) -> &Token {
        &self.tokens[self.i]
    }
    fn is(&self, text: &str) -> bool {
        match &self.current().kind {
            TokenKind::Word(w) => keyword(w) == text,
            TokenKind::Symbol(s) => s == text,
            _ => false,
        }
    }
    fn next_is(&self, text: &str) -> bool {
        self.tokens
            .get(self.i + 1)
            .is_some_and(|t| matches!(&t.kind, TokenKind::Symbol(s) if s == text))
    }
    fn take(&mut self) -> Token {
        let t = self.current().clone();
        if t.kind != TokenKind::Eof {
            self.i += 1;
        }
        t
    }
    fn eat(&mut self, text: &str) -> bool {
        if self.is(text) {
            self.take();
            true
        } else {
            false
        }
    }
    fn error(&self) -> Error {
        Error::new(
            "CB002",
            self.current().span,
            "Sintaxe inválida ou expressão incompleta.",
            "Invalid syntax or incomplete expression.",
        )
    }
    fn expect(&mut self, text: &str) -> Result<()> {
        if self.eat(text) {
            Ok(())
        } else {
            Err(Error::new(
                "CB002",
                self.current().span,
                format!("Esperado: {text}."),
                format!("Expected: {text}."),
            ))
        }
    }
    fn word(&mut self) -> Result<String> {
        match self.take().kind {
            TokenKind::Word(w) => Ok(w),
            _ => Err(self.error()),
        }
    }
    fn separators(&mut self) {
        while self.current().kind == TokenKind::Sep {
            self.take();
        }
    }
    fn enter(&mut self) -> Result<()> {
        self.depth += 1;
        if self.depth > 128 {
            Err(Error::new(
                "CB106",
                self.current().span,
                "Aninhamento excessivo.",
                "Excessive nesting.",
            ))
        } else {
            Ok(())
        }
    }
    fn statements(&mut self, block: bool) -> Result<Vec<Stmt>> {
        self.enter()?;
        let mut out = vec![];
        self.separators();
        while self.current().kind != TokenKind::Eof && !(block && self.is("}")) {
            out.push(self.statement()?);
            if self.current().kind != TokenKind::Sep
                && self.current().kind != TokenKind::Eof
                && !(block && self.is("}"))
            {
                return Err(Error::new(
                    "CB002",
                    self.current().span,
                    "Separe instruções com nova linha ou ;.",
                    "Separate statements with a newline or semicolon.",
                ));
            }
            self.separators();
        }
        if block {
            self.expect("}")?;
        }
        self.depth -= 1;
        Ok(out)
    }
    fn block(&mut self) -> Result<Vec<Stmt>> {
        self.expect("{")?;
        self.statements(true)
    }
    fn statement(&mut self) -> Result<Stmt> {
        let span = self.current().span;
        let kind = if self.is("variable") || self.is("constant") {
            let constant = self.is("constant");
            self.take();
            let name = self.word()?;
            self.expect("=")?;
            StmtKind::Declare {
                name,
                constant,
                value: self.expr(0)?,
            }
        } else if self.eat("permission") {
            StmtKind::Permission(self.word()?)
        } else if self.eat("import") {
            StmtKind::Import(self.word()?)
        } else if self.eat("function") {
            let name = self.word()?;
            self.expect("(")?;
            let mut args = vec![];
            if !self.is(")") {
                loop {
                    args.push(self.word()?);
                    if !self.eat(",") {
                        break;
                    }
                }
            }
            self.expect(")")?;
            StmtKind::Function(name, args, self.block()?)
        } else if self.is("event") && !self.next_is(".") {
            self.take();
            let name = self.word()?;
            StmtKind::Event(name, self.block()?)
        } else if self.is("test") && !self.next_is(".") {
            self.take();
            let name = match self.take().kind {
                TokenKind::Text(t) => t,
                _ => return Err(self.error()),
            };
            StmtKind::Test(name, self.block()?)
        } else if self.eat("if") {
            let condition = self.expr(0)?;
            let yes = self.block()?;
            let saved = self.i;
            self.separators();
            let no = if self.eat("else") {
                self.block()?
            } else {
                self.i = saved;
                vec![]
            };
            StmtKind::If(condition, yes, no)
        } else if self.eat("while") {
            let e = self.expr(0)?;
            StmtKind::While(e, self.block()?)
        } else if self.eat("repeat") {
            let e = self.expr(0)?;
            self.eat("times");
            StmtKind::Repeat(e, self.block()?)
        } else if self.eat("for") {
            self.eat("each");
            let name = self.word()?;
            self.expect("in")?;
            let e = self.expr(0)?;
            StmtKind::For(name, e, self.block()?)
        } else if self.eat("return") {
            StmtKind::Return(self.expr(0)?)
        } else {
            let e = self.expr(0)?;
            if self.eat("=") {
                if !matches!(e.kind, ExprKind::Name(_) | ExprKind::Index(_, _)) {
                    return Err(self.error());
                }
                StmtKind::Assign(e, self.expr(0)?)
            } else {
                StmtKind::Expression(e)
            }
        };
        Ok(Stmt { kind, span })
    }
    fn args(&mut self, end: &str) -> Result<Vec<Expr>> {
        let mut out = vec![];
        self.separators();
        if !self.is(end) {
            loop {
                out.push(self.expr(0)?);
                self.separators();
                if !self.eat(",") {
                    break;
                }
                self.separators();
            }
        }
        self.expect(end)?;
        Ok(out)
    }
    fn precedence(&self) -> Option<(String, u8)> {
        let op = match &self.current().kind {
            TokenKind::Symbol(s) => s.as_str(),
            TokenKind::Word(w) => keyword(w),
            _ => return None,
        };
        let level = match op {
            "or" => 1,
            "and" => 2,
            "==" | "!=" => 3,
            "<" | ">" | "<=" | ">=" => 4,
            "+" | "-" => 5,
            "*" | "/" | "%" => 6,
            _ => return None,
        };
        Some((op.into(), level))
    }
    fn expr(&mut self, min: u8) -> Result<Expr> {
        self.enter()?;
        let t = self.take();
        let span = t.span;
        let mut left = match t.kind {
            TokenKind::Text(s) => ExprKind::Literal(Value::Text(s)),
            TokenKind::Number(s) => ExprKind::Literal(if s.contains('.') {
                let v: f64 = s.parse().map_err(|_| self.error())?;
                if !v.is_finite() {
                    return Err(self.error());
                }
                Value::Number(v)
            } else {
                Value::Int(s.parse().map_err(|_| {
                    Error::new(
                        "CB103",
                        span,
                        "Inteiro fora de 32 bits.",
                        "Integer outside 32-bit range.",
                    )
                })?)
            }),
            TokenKind::Word(w) if keyword(&w) == "not" => {
                ExprKind::Unary("not".into(), Box::new(self.expr(7)?))
            }
            TokenKind::Word(w) => match keyword(&w) {
                "true" => ExprKind::Literal(Value::Bool(true)),
                "false" => ExprKind::Literal(Value::Bool(false)),
                "null" => ExprKind::Literal(Value::Null),
                _ => {
                    let mut name = w;
                    while self.eat(".") {
                        name.push('.');
                        name.push_str(&self.word()?);
                    }
                    ExprKind::Name(name)
                }
            },
            TokenKind::Symbol(s) if s == "-" || s == "+" => {
                if s == "-"
                    && matches!(&self.current().kind, TokenKind::Number(n) if n == "2147483648")
                {
                    self.take();
                    ExprKind::Literal(Value::Int(i32::MIN))
                } else {
                    ExprKind::Unary(s, Box::new(self.expr(7)?))
                }
            }
            TokenKind::Symbol(s) if s == "(" => {
                let e = self.expr(0)?;
                self.expect(")")?;
                e.kind
            }
            TokenKind::Symbol(s) if s == "[" => ExprKind::List(self.args("]")?),
            TokenKind::Symbol(s) if s == "{" => {
                let mut pairs = vec![];
                self.separators();
                if !self.is("}") {
                    loop {
                        let key = match self.take().kind {
                            TokenKind::Text(s) => s,
                            _ => return Err(self.error()),
                        };
                        self.expect(":")?;
                        pairs.push((key, self.expr(0)?));
                        self.separators();
                        if !self.eat(",") {
                            break;
                        }
                        self.separators();
                    }
                }
                self.expect("}")?;
                ExprKind::Map(pairs)
            }
            _ => {
                return Err(Error::new(
                    "CB002",
                    span,
                    "Esperada uma expressão.",
                    "Expected an expression.",
                ));
            }
        };
        let mut chain = 0;
        loop {
            chain += 1;
            if chain > 128 {
                return Err(Error::new(
                    "CB106",
                    span,
                    "Expressão muito longa.",
                    "Expression is too long.",
                ));
            }
            if self.eat("(") {
                let name = match left {
                    ExprKind::Name(name) => name,
                    _ => return Err(self.error()),
                };
                left = ExprKind::Call(name, self.args(")")?);
            } else if self.eat("[") {
                let index = self.expr(0)?;
                self.expect("]")?;
                left = ExprKind::Index(
                    Box::new(Expr {
                        kind: left,
                        span,
                        ty: Type::Any,
                    }),
                    Box::new(index),
                );
            } else if let Some((op, p)) = self.precedence() {
                if p < min {
                    break;
                }
                let op_span = self.take().span;
                let right = self.expr(p + 1)?;
                left = ExprKind::Binary(
                    op,
                    Box::new(Expr {
                        kind: left,
                        span: op_span,
                        ty: Type::Any,
                    }),
                    Box::new(right),
                );
            } else {
                break;
            }
        }
        self.depth -= 1;
        Ok(Expr {
            kind: left,
            span,
            ty: Type::Any,
        })
    }
}
