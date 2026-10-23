use crate::diagnostic::{Error, Result, Span};

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Word(String),
    Number(String),
    Text(String),
    Symbol(String),
    Sep,
    Eof,
}
#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

pub fn keyword(word: &str) -> &str {
    match word {
        "variavel" => "variable",
        "constante" => "constant",
        "funcao" => "function",
        "se" => "if",
        "senao" => "else",
        "enquanto" => "while",
        "repetir" => "repeat",
        "vezes" => "times",
        "para" => "for",
        "cada" => "each",
        "em" => "in",
        "evento" => "event",
        "teste" => "test",
        "retornar" => "return",
        "importar" => "import",
        "permissao" => "permission",
        "verdadeiro" => "true",
        "falso" => "false",
        "nulo" => "null",
        "e" => "and",
        "ou" => "or",
        "nao" => "not",
        _ => word,
    }
}
pub fn lex(source: &str) -> Result<Vec<Token>> {
    let chars: Vec<char> = source.chars().collect();
    let (mut i, mut line, mut col) = (0, 1, 1);
    let mut tokens = Vec::new();
    while i < chars.len() {
        let c = chars[i];
        let span = Span { line, column: col };
        let start = i;
        if c == ' ' || c == '\t' || c == '\r' {
            i += 1;
            col += 1;
            continue;
        }
        if c == '#' || c == '/' && chars.get(i + 1) == Some(&'/') {
            while i < chars.len() && chars[i] != '\n' {
                i += 1;
                col += 1;
            }
            continue;
        }
        if c == '\n' {
            tokens.push(Token {
                kind: TokenKind::Sep,
                span,
            });
            i += 1;
            line += 1;
            col = 1;
            continue;
        }
        let kind = if c == '\'' || c == '"' {
            let quote = c;
            i += 1;
            let mut text = String::new();
            while i < chars.len() && chars[i] != quote {
                if chars[i] == '\n' {
                    return Err(Error::new(
                        "CB001",
                        span,
                        "Texto não terminado.",
                        "Unterminated string.",
                    ));
                }
                if chars[i] == '\\' {
                    // Record the column of the backslash for accurate diagnostics.
                    let escape_col = col + (i - start);
                    let escape_span = Span { line, column: escape_col };
                    i += 1;
                    let escaped = match chars.get(i) {
                        Some('n') => '\n',
                        Some('r') => '\r',
                        Some('t') => '\t',
                        Some('\\') => '\\',
                        Some('"') => '"',
                        Some('\'') => '\'',
                        _ => {
                            return Err(Error::new(
                                "CB001",
                                escape_span,
                                "Escape inválido.",
                                "Invalid escape.",
                            ));
                        }
                    };
                    text.push(escaped);
                } else {
                    text.push(chars[i]);
                }
                i += 1;
            }
            if i >= chars.len() {
                return Err(Error::new(
                    "CB001",
                    span,
                    "Texto não terminado.",
                    "Unterminated string.",
                ));
            }
            i += 1;
            TokenKind::Text(text)
        } else if c.is_ascii_digit() {
            i += 1;
            while chars.get(i).is_some_and(|c| c.is_ascii_digit()) {
                i += 1;
            }
            if chars.get(i) == Some(&'.') && chars.get(i + 1).is_some_and(|c| c.is_ascii_digit()) {
                i += 1;
                while chars.get(i).is_some_and(|c| c.is_ascii_digit()) {
                    i += 1;
                }
            }
            TokenKind::Number(chars[start..i].iter().collect())
        } else if c.is_alphabetic() || c == '_' {
            i += 1;
            while chars
                .get(i)
                .is_some_and(|c| c.is_alphanumeric() || *c == '_')
            {
                i += 1;
            }
            TokenKind::Word(chars[start..i].iter().collect())
        } else if "+-*/%<>=!(){}[],:.;".contains(c) {
            i += 1;
            let mut symbol = c.to_string();
            if "=!<>".contains(c) && chars.get(i) == Some(&'=') {
                symbol.push('=');
                i += 1;
            }
            if symbol == ";" {
                TokenKind::Sep
            } else {
                TokenKind::Symbol(symbol)
            }
        } else {
            return Err(Error::new(
                "CB001",
                span,
                "Caractere inválido.",
                "Invalid character.",
            ));
        };
        col += i - start;
        tokens.push(Token { kind, span });
    }
    tokens.push(Token {
        kind: TokenKind::Eof,
        span: Span { line, column: col },
    });
    Ok(tokens)
}
