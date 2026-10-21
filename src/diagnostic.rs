use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub line: usize,
    pub column: usize,
}
impl Default for Span {
    fn default() -> Self {
        Self { line: 1, column: 1 }
    }
}

#[derive(Debug, Clone)]
pub struct Error {
    pub code: &'static str,
    pub span: Span,
    pub pt: String,
    pub en: String,
    pub hint_pt: String,
    pub hint_en: String,
}
pub type Result<T> = std::result::Result<T, Error>;
impl Error {
    pub fn new(
        code: &'static str,
        span: Span,
        pt: impl Into<String>,
        en: impl Into<String>,
    ) -> Self {
        Self {
            code,
            span,
            pt: pt.into(),
            en: en.into(),
            hint_pt: "Revise a instrução e seus argumentos.".into(),
            hint_en: "Check the statement and its arguments.".into(),
        }
    }
    pub fn hint(mut self, pt: impl Into<String>, en: impl Into<String>) -> Self {
        self.hint_pt = pt.into();
        self.hint_en = en.into();
        self
    }
    pub fn render(&self, file: &str, english: bool) -> String {
        format!(
            "{}:{}:{}: {}: {}\n{}: {}",
            file,
            self.span.line,
            self.span.column,
            self.code,
            if english { &self.en } else { &self.pt },
            if english { "Suggestion" } else { "Sugestão" },
            if english {
                &self.hint_en
            } else {
                &self.hint_pt
            }
        )
    }
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.code, self.en)
    }
}
impl std::error::Error for Error {}

pub fn io_error() -> Error {
    Error::new(
        "CB009",
        Span::default(),
        "Falha de entrada/saída; detalhes privados omitidos.",
        "Input/output failure; private details omitted.",
    )
}
pub fn type_error(span: Span) -> Error {
    Error::new("CB103", span, "Tipos incompatíveis.", "Incompatible types.")
}
pub fn incompatible(span: Span) -> Error {
    Error::new(
        "CB201",
        span,
        "Recurso incompatível com este ambiente ou backend.",
        "Feature incompatible with this environment or backend.",
    )
    .hint(
        "Use o subconjunto documentado; mantenha APIs externas no computador com minecraft_bridge.",
        "Use the documented subset; keep external APIs on the computer with minecraft_bridge.",
    )
}
