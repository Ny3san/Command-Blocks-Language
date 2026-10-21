pub mod catalog;
pub mod cli;
pub mod diagnostic;
pub mod discord;
pub mod integrations;
pub mod json;
pub mod lexer;
pub mod minecraft;
pub mod net;
pub mod parser;
pub mod runtime;
pub mod semantic;
pub mod value;
pub mod window;

pub fn compile(source: &str) -> diagnostic::Result<semantic::Program> {
    if source.len() > 1_000_000 {
        return Err(diagnostic::Error::new(
            "CB106",
            Default::default(),
            "Fonte muito grande.",
            "Source is too large.",
        ));
    }
    semantic::analyze(parser::parse(lexer::lex(source)?)?)
}
