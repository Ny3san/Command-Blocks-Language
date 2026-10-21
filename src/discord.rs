//! Discord REST client. Gateway event transport is a separate lifecycle.
use crate::{
    diagnostic::{Error, Result, Span},
    json, net,
    value::Value,
};
use std::{
    collections::BTreeMap,
    rc::Rc,
    time::{Duration, Instant},
};

pub struct Client {
    token: Rc<String>,
    next_request: Instant,
    recent_requests: Vec<Instant>,
}
fn failure(span: Span, code: &'static str) -> Error {
    Error::new(
        code,
        span,
        "Discord não confirmou a operação; detalhes privados omitidos.",
        "Discord did not confirm the operation; private details omitted.",
    )
}
fn id(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 20
        && value.bytes().all(|b| b.is_ascii_digit())
        && value.parse::<u64>().is_ok_and(|v| v > 0)
}
impl Client {
    pub fn new(token: Rc<String>, span: Span) -> Result<Self> {
        if token.len() < 24
            || token.len() > 256
            || !token
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b"._-".contains(&b))
        {
            return Err(failure(span, "CB301"));
        }
        Ok(Self {
            token,
            next_request: Instant::now(),
            recent_requests: Vec::new(),
        })
    }
    pub fn authenticate(&mut self, span: Span) -> Result<Value> {
        let user = self.request("GET", "/users/@me", "", span)?;
        let Value::Map(properties) = &user else {
            return Err(failure(span, "CB304"));
        };
        if !matches!(properties.borrow().get("bot"), Some(Value::Bool(true))) {
            return Err(failure(span, "CB301"));
        }
        Ok(user)
    }
    pub fn send(&mut self, channel: &str, content: &str, span: Span) -> Result<Value> {
        if !id(channel) || content.is_empty() || content.encode_utf16().count() > 2000 {
            return Err(failure(span, "CB105"));
        }
        let payload = Value::map(BTreeMap::from([
            ("content".into(), Value::Text(content.into())),
            (
                "allowed_mentions".into(),
                Value::map(BTreeMap::from([("parse".into(), Value::list(vec![]))])),
            ),
        ]));
        self.request(
            "POST",
            &format!("/channels/{channel}/messages"),
            &json::stringify(&payload, span)?,
            span,
        )
    }
    fn request(&mut self, method: &str, path: &str, body: &str, span: Span) -> Result<Value> {
        self.request_with(method, path, body, span, net::request)
    }
    fn request_with<F>(
        &mut self,
        method: &str,
        path: &str,
        body: &str,
        span: Span,
        transport: F,
    ) -> Result<Value>
    where
        F: FnOnce(&str, &str, &str, &BTreeMap<String, String>, i32, Span) -> Result<net::Response>,
    {
        let now = Instant::now();
        self.recent_requests
            .retain(|t| now.duration_since(*t) < Duration::from_secs(1));
        if now < self.next_request || self.recent_requests.len() >= 10 {
            return Err(failure(span, "CB302"));
        }
        self.recent_requests.push(now);
        let headers = BTreeMap::from([
            ("Authorization".into(), format!("Bot {}", self.token)),
            ("Content-Type".into(), "application/json".into()),
        ]);
        let response = transport(
            method,
            &format!("https://discord.com/api/v10{path}"),
            body,
            &headers,
            10000,
            span,
        )?;
        if response.status == 429 {
            let delay = json::parse(&response.body, span)
                .ok()
                .and_then(|v| {
                    if let Value::Map(m) = v {
                        m.borrow().get("retry_after").and_then(Value::number)
                    } else {
                        None
                    }
                })
                .filter(|n| n.is_finite() && *n > 0.0)
                .unwrap_or(60.0)
                .clamp(1.0, 86400.0);
            self.next_request = Instant::now() + Duration::from_secs_f64(delay);
            return Err(failure(span, "CB302"));
        }
        if response.status == 401 || response.status == 403 {
            return Err(failure(span, "CB301"));
        }
        if !(200..300).contains(&response.status) {
            return Err(failure(span, "CB304"));
        }
        if response.body.is_empty() {
            return Ok(Value::Null);
        }
        json::parse(&response.body, span).map_err(|_| failure(span, "CB304"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn consecutive_requests_work_and_bursts_are_bounded() {
        let span = Span::default();
        let mut client =
            Client::new(Rc::new("private_test_token_not_real_123456".into()), span).unwrap();
        for _ in 0..10 {
            client
                .request_with("GET", "/users/@me", "", span, |_, _, _, _, _, _| {
                    Ok(net::Response {
                        status: 200,
                        body: "{}".into(),
                    })
                })
                .unwrap();
        }
        assert_eq!(
            client
                .request_with("GET", "/users/@me", "", span, |_, _, _, _, _, _| panic!(
                    "burst must be blocked"
                ))
                .unwrap_err()
                .code,
            "CB302"
        );
    }
    #[test]
    fn authenticated_request_and_rate_limit_without_secret_errors() {
        let span = Span::default();
        let token = Rc::new("private_test_token_not_real_123456".into());
        let mut client = Client::new(token, span).unwrap();
        let result = client
            .request_with(
                "GET",
                "/users/@me",
                "",
                span,
                |method, url, _, headers, _, _| {
                    assert_eq!(method, "GET");
                    assert_eq!(url, "https://discord.com/api/v10/users/@me");
                    assert!(headers["Authorization"].starts_with("Bot "));
                    Ok(net::Response {
                        status: 429,
                        body: "{\"retry_after\":2.5}".into(),
                    })
                },
            )
            .unwrap_err();
        assert_eq!(result.code, "CB302");
        assert!(!format!("{result:?}").contains("private_test"));
        assert!(client.next_request > Instant::now() + Duration::from_secs(2));
        assert_eq!(
            client
                .request_with("GET", "/users/@me", "", span, |_, _, _, _, _, _| panic!(
                    "rate limited request must not reach transport"
                ))
                .unwrap_err()
                .code,
            "CB302"
        );
    }
    #[test]
    fn rejects_invalid_tokens_and_channel_injection() {
        let span = Span::default();
        assert!(Client::new(Rc::new("bad\r\ntoken".into()), span).is_err());
        let mut client =
            Client::new(Rc::new("private_test_token_not_real_123456".into()), span).unwrap();
        assert_eq!(
            client
                .send("123/../../users", "hello", span)
                .unwrap_err()
                .code,
            "CB105"
        );
        assert_eq!(
            client
                .send("123", &"x".repeat(2001), span)
                .unwrap_err()
                .code,
            "CB105"
        );
    }
}
