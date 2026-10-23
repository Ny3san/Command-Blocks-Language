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
    pub pt: Box<str>,
    pub en: Box<str>,
    pub hint_pt: Box<str>,
    pub hint_en: Box<str>,
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
            pt: pt.into().into_boxed_str(),
            en: en.into().into_boxed_str(),
            hint_pt: "Revise a instrução e seus argumentos.".into(),
            hint_en: "Check the statement and its arguments.".into(),
        }
    }
    pub fn hint(mut self, pt: impl Into<String>, en: impl Into<String>) -> Self {
        self.hint_pt = pt.into().into_boxed_str();
        self.hint_en = en.into().into_boxed_str();
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
        "Use o subconjunto documentado do backend Minecraft.",
        "Use the documented subset of the Minecraft backend.",
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn localized_diagnostic_preserves_location_code_and_hint() {
        let error = Error::new(
            "CB103",
            Span {
                line: 4,
                column: 12,
            },
            "Tipos incompatíveis.",
            "Incompatible types.",
        )
        .hint("Use um número.", "Use a number.");
        assert_eq!(
            error.render("programa.cb", false),
            "programa.cb:4:12: CB103: Tipos incompatíveis.\nSugestão: Use um número."
        );
        assert_eq!(
            error.clone().render("program.cb", true),
            "program.cb:4:12: CB103: Incompatible types.\nSuggestion: Use a number."
        );
        assert_eq!(error.to_string(), "CB103: Incompatible types.");
    }
}
