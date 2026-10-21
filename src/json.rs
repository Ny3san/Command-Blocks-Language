//! Bounded JSON codec shared by scripts and integration protocols.
use crate::{
    diagnostic::{Error, Result, Span},
    value::{Value, quote},
};
use std::collections::BTreeMap;

fn invalid(span: Span) -> Error {
    Error::new(
        "CB107",
        span,
        "JSON inválido ou limite excedido.",
        "Invalid JSON or limit exceeded.",
    )
    .hint(
        "Verifique aspas, escapes, números e separadores; limite de 1 MB e 32 níveis.",
        "Check quotes, escapes, numbers and separators; limit is 1 MB and 32 levels.",
    )
}
pub fn parse(source: &str, span: Span) -> Result<Value> {
    if source.len() > 1_000_000 {
        return Err(invalid(span));
    }
    let mut parser = Parser {
        source,
        pos: 0,
        span,
    };
    let value = parser.value(0)?;
    parser.space();
    if parser.pos != source.len() {
        return Err(invalid(span));
    }
    Ok(value)
}
struct Parser<'a> {
    source: &'a str,
    pos: usize,
    span: Span,
}
impl Parser<'_> {
    fn peek(&self) -> Option<u8> {
        self.source.as_bytes().get(self.pos).copied()
    }
    fn space(&mut self) {
        while matches!(self.peek(), Some(b' ' | b'\n' | b'\r' | b'\t')) {
            self.pos += 1;
        }
    }
    fn take(&mut self, b: u8) -> bool {
        self.space();
        if self.peek() == Some(b) {
            self.pos += 1;
            true
        } else {
            false
        }
    }
    fn fail<T>(&self) -> Result<T> {
        Err(invalid(self.span))
    }
    fn value(&mut self, depth: usize) -> Result<Value> {
        if depth > 32 {
            return self.fail();
        }
        self.space();
        match self.peek() {
            Some(b'"') => Ok(Value::Text(self.string()?)),
            Some(b'[') => {
                self.pos += 1;
                let mut values = Vec::new();
                if self.take(b']') {
                    return Ok(Value::list(values));
                }
                loop {
                    values.push(self.value(depth + 1)?);
                    if self.take(b']') {
                        break;
                    }
                    if !self.take(b',') {
                        return self.fail();
                    }
                }
                Ok(Value::list(values))
            }
            Some(b'{') => {
                self.pos += 1;
                let mut values = BTreeMap::new();
                if self.take(b'}') {
                    return Ok(Value::map(values));
                }
                loop {
                    self.space();
                    let key = self.string()?;
                    if !self.take(b':') {
                        return self.fail();
                    }
                    let value = self.value(depth + 1)?;
                    // Reject ambiguous duplicate properties in authentication/protocol data.
                    if values.insert(key, value).is_some() {
                        return self.fail();
                    }
                    if self.take(b'}') {
                        break;
                    }
                    if !self.take(b',') {
                        return self.fail();
                    }
                }
                Ok(Value::map(values))
            }
            Some(b't') => self.literal("true", Value::Bool(true)),
            Some(b'f') => self.literal("false", Value::Bool(false)),
            Some(b'n') => self.literal("null", Value::Null),
            Some(b'-' | b'0'..=b'9') => self.number(),
            _ => self.fail(),
        }
    }
    fn literal(&mut self, s: &str, v: Value) -> Result<Value> {
        if self.source[self.pos..].starts_with(s) {
            self.pos += s.len();
            Ok(v)
        } else {
            self.fail()
        }
    }
    fn number(&mut self) -> Result<Value> {
        let start = self.pos;
        if self.peek() == Some(b'-') {
            self.pos += 1;
        }
        match self.peek() {
            Some(b'0') => self.pos += 1,
            Some(b'1'..=b'9') => {
                while matches!(self.peek(), Some(b'0'..=b'9')) {
                    self.pos += 1;
                }
            }
            _ => return self.fail(),
        }
        if self.peek() == Some(b'.') {
            self.pos += 1;
            let p = self.pos;
            while matches!(self.peek(), Some(b'0'..=b'9')) {
                self.pos += 1;
            }
            if p == self.pos {
                return self.fail();
            }
        }
        if matches!(self.peek(), Some(b'e' | b'E')) {
            self.pos += 1;
            if matches!(self.peek(), Some(b'+' | b'-')) {
                self.pos += 1;
            }
            let p = self.pos;
            while matches!(self.peek(), Some(b'0'..=b'9')) {
                self.pos += 1;
            }
            if p == self.pos {
                return self.fail();
            }
        }
        let token = &self.source[start..self.pos];
        if let Ok(n) = token.parse::<i32>() {
            return Ok(Value::Int(n));
        }
        let n = token.parse::<f64>().map_err(|_| invalid(self.span))?;
        if !n.is_finite() {
            return self.fail();
        }
        Ok(Value::Number(n))
    }
    fn hex(&mut self) -> Result<u32> {
        let mut value = 0;
        for _ in 0..4 {
            let c = self
                .peek()
                .and_then(|c| (c as char).to_digit(16))
                .ok_or_else(|| invalid(self.span))?;
            self.pos += 1;
            value = value * 16 + c;
        }
        Ok(value)
    }
    fn string(&mut self) -> Result<String> {
        if self.peek() != Some(b'"') {
            return self.fail();
        }
        self.pos += 1;
        let mut out = String::new();
        loop {
            match self.peek() {
                Some(b'"') => {
                    self.pos += 1;
                    return Ok(out);
                }
                Some(b'\\') => {
                    self.pos += 1;
                    let escape = self.peek().ok_or_else(|| invalid(self.span))?;
                    self.pos += 1;
                    match escape {
                        b'"' => out.push('"'),
                        b'\\' => out.push('\\'),
                        b'/' => out.push('/'),
                        b'b' => out.push('\u{8}'),
                        b'f' => out.push('\u{c}'),
                        b'n' => out.push('\n'),
                        b'r' => out.push('\r'),
                        b't' => out.push('\t'),
                        b'u' => {
                            let first = self.hex()?;
                            let code = if (0xd800..=0xdbff).contains(&first) {
                                if !self.source[self.pos..].starts_with("\\u") {
                                    return self.fail();
                                }
                                self.pos += 2;
                                let second = self.hex()?;
                                if !(0xdc00..=0xdfff).contains(&second) {
                                    return self.fail();
                                }
                                0x10000 + ((first - 0xd800) << 10) + (second - 0xdc00)
                            } else {
                                first
                            };
                            out.push(char::from_u32(code).ok_or_else(|| invalid(self.span))?);
                        }
                        _ => return self.fail(),
                    }
                }
                Some(0..=31) | None => return self.fail(),
                _ => {
                    let c = self.source[self.pos..].chars().next().unwrap();
                    out.push(c);
                    self.pos += c.len_utf8();
                }
            }
        }
    }
}

pub fn stringify(value: &Value, span: Span) -> Result<String> {
    fn write(value: &Value, out: &mut String, depth: usize, span: Span) -> Result<()> {
        if depth > 32 {
            return Err(invalid(span));
        }
        match value {
            Value::Null => out.push_str("null"),
            Value::Bool(v) => out.push_str(if *v { "true" } else { "false" }),
            Value::Int(v) => out.push_str(&v.to_string()),
            Value::Number(v) if v.is_finite() => out.push_str(&v.to_string()),
            Value::Text(v) => out.push_str(&quote(v)),
            Value::Secret(_) => out.push_str("\"[REDACTED]\""),
            Value::List(v) => {
                out.push('[');
                for (i, v) in v.borrow().iter().enumerate() {
                    if i > 0 {
                        out.push(',');
                    }
                    write(v, out, depth + 1, span)?;
                }
                out.push(']');
            }
            Value::Map(v) => {
                out.push('{');
                for (i, (k, v)) in v.borrow().iter().enumerate() {
                    if i > 0 {
                        out.push(',');
                    }
                    out.push_str(&quote(k));
                    out.push(':');
                    write(v, out, depth + 1, span)?;
                }
                out.push('}');
            }
            _ => return Err(invalid(span)),
        }
        if out.len() > 1_000_000 {
            return Err(invalid(span));
        }
        Ok(())
    }
    let mut out = String::new();
    write(value, &mut out, 0, span)?;
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn unicode_nested_roundtrip_and_strict_errors() {
        let span = Span::default();
        let v = parse(
            r#"{"name":"Alex","emoji":"\ud83d\ude00","data":[true,null,-2.5e2]}"#,
            span,
        )
        .unwrap();
        let text = stringify(&v, span).unwrap();
        assert!(text.contains('😀'));
        assert_eq!(stringify(&parse(&text, span).unwrap(), span).unwrap(), text);
        for bad in [
            "01",
            "1.",
            "1e",
            "+1",
            "1e999",
            "[1,]",
            "{\"x\":1,\"x\":2}",
            r#""\ud800""#,
            r#""\udc00""#,
            "true false",
            "\"line\nfeed\"",
        ] {
            assert!(parse(bad, span).is_err(), "{bad:?}");
        }
        assert!(parse(&format!("{}0{}", "[".repeat(34), "]".repeat(34)), span).is_err());
        assert!(stringify(&Value::Number(f64::NAN), span).is_err());
        assert_eq!(
            stringify(&Value::Secret(std::rc::Rc::new("private".into())), span).unwrap(),
            "\"[REDACTED]\""
        );
    }
}
