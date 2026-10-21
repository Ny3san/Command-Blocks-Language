//! HTTP boundary. Uses the operating system TLS and certificate validation.
use crate::diagnostic::{Error, Result, Span};
use std::collections::BTreeMap;

pub struct Response {
    pub status: i32,
    pub body: String,
}
pub(crate) struct Url {
    pub host: String,
    pub port: u16,
    pub path: String,
    pub secure: bool,
}
fn error(span: Span) -> Error {
    Error::new(
        "CB304",
        span,
        "Requisição recusada, inválida ou sem resposta válida.",
        "Request rejected, invalid, or no valid response received.",
    )
    .hint(
        "Use HTTPS, confira o endereço e o tempo limite. Detalhes privados foram omitidos.",
        "Use HTTPS, check the address and timeout. Private details were omitted.",
    )
}
impl Url {
    pub(crate) fn parse(url: &str, span: Span) -> Result<Self> {
        if url.len() > 8192
            || url.chars().any(|c| c.is_control() || c.is_whitespace())
            || url.contains(['\\', '#', '@'])
        {
            return Err(error(span));
        }
        let (secure, rest) = if let Some(s) = url.strip_prefix("https://") {
            (true, s)
        } else if let Some(s) = url.strip_prefix("http://") {
            (false, s)
        } else {
            return Err(error(span));
        };
        let end = rest.find(['/', '?']).unwrap_or(rest.len());
        let authority = &rest[..end];
        let (host, port) = if let Some((h, p)) = authority.split_once(':') {
            (h, p.parse::<u16>().map_err(|_| error(span))?)
        } else {
            (authority, if secure { 443 } else { 80 })
        };
        if host.is_empty()
            || host.len() > 253
            || !host
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b".-".contains(&b))
            || port == 0
            || (!secure && host != "127.0.0.1" && host != "localhost")
        {
            return Err(error(span));
        }
        let path = match &rest[end..] {
            "" => "/".into(),
            s if s.starts_with('?') => format!("/{s}"),
            s => s.into(),
        };
        Ok(Self {
            host: host.into(),
            port,
            path,
            secure,
        })
    }
}
pub fn request(
    method: &str,
    url: &str,
    body: &str,
    headers: &BTreeMap<String, String>,
    timeout: i32,
    span: Span,
) -> Result<Response> {
    let url = Url::parse(url, span)?;
    if !["GET", "POST", "PUT", "DELETE", "PATCH", "HEAD"].contains(&method)
        || !(100..=60000).contains(&timeout)
        || body.len() > 1_000_000
        || headers.len() > 64
    {
        return Err(error(span));
    }
    let mut header_text = String::new();
    for (key, value) in headers {
        if key.is_empty()
            || !key
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b"-".contains(&b))
            || value.chars().any(|c| c.is_control())
            || [
                "host",
                "content-length",
                "transfer-encoding",
                "connection",
                "proxy-authorization",
            ]
            .contains(&key.to_ascii_lowercase().as_str())
        {
            return Err(error(span));
        }
        header_text.push_str(key);
        header_text.push_str(": ");
        header_text.push_str(value);
        header_text.push_str("\r\n");
        if header_text.len() > 16384 {
            return Err(error(span));
        }
    }
    #[cfg(windows)]
    {
        native::request(method, &url, body, &header_text, timeout, span)
    }
    #[cfg(not(windows))]
    {
        let _ = (url, header_text);
        Err(Error::new(
            "CB201",
            span,
            "Transporte HTTP nativo disponível apenas no Windows nesta versão.",
            "Native HTTP transport is available only on Windows in this version.",
        ))
    }
}

#[cfg(windows)]
mod native {
    use super::*;
    use std::{
        ffi::c_void,
        ptr::{null, null_mut},
        time::Instant,
    };
    type Raw = *mut c_void;
    #[link(name = "winhttp", kind = "raw-dylib")]
    unsafe extern "system" {
        fn WinHttpOpen(
            agent: *const u16,
            access: u32,
            proxy: *const u16,
            bypass: *const u16,
            flags: u32,
        ) -> Raw;
        fn WinHttpConnect(session: Raw, server: *const u16, port: u16, reserved: u32) -> Raw;
        fn WinHttpOpenRequest(
            connect: Raw,
            verb: *const u16,
            path: *const u16,
            version: *const u16,
            referer: *const u16,
            accept: *const *const u16,
            flags: u32,
        ) -> Raw;
        fn WinHttpSetTimeouts(
            handle: Raw,
            resolve: i32,
            connect: i32,
            send: i32,
            receive: i32,
        ) -> i32;
        fn WinHttpSetOption(handle: Raw, option: u32, value: *const c_void, len: u32) -> i32;
        fn WinHttpSendRequest(
            handle: Raw,
            headers: *const u16,
            length: u32,
            data: *const c_void,
            data_len: u32,
            total: u32,
            context: usize,
        ) -> i32;
        fn WinHttpReceiveResponse(handle: Raw, reserved: Raw) -> i32;
        fn WinHttpQueryHeaders(
            handle: Raw,
            info: u32,
            name: *const u16,
            buffer: Raw,
            len: *mut u32,
            index: *mut u32,
        ) -> i32;
        fn WinHttpReadData(handle: Raw, buffer: Raw, size: u32, read: *mut u32) -> i32;
        fn WinHttpCloseHandle(handle: Raw) -> i32;
    }
    fn wide(s: &str) -> Vec<u16> {
        s.encode_utf16().chain(Some(0)).collect()
    }
    struct Handle(Raw);
    impl Handle {
        fn new(raw: Raw, span: Span) -> Result<Self> {
            if raw.is_null() {
                Err(error(span))
            } else {
                Ok(Self(raw))
            }
        }
    }
    impl Drop for Handle {
        fn drop(&mut self) {
            unsafe {
                WinHttpCloseHandle(self.0);
            }
        }
    }
    pub(super) fn request(
        method: &str,
        url: &Url,
        body: &str,
        headers: &str,
        timeout: i32,
        span: Span,
    ) -> Result<Response> {
        unsafe {
            // No implicit proxy credentials, cookies or redirects are carried to another origin.
            let session = Handle::new(
                WinHttpOpen(wide("CommandBlocks/0.1").as_ptr(), 1, null(), null(), 0),
                span,
            )?;
            if WinHttpSetTimeouts(session.0, timeout, timeout, timeout, timeout) == 0 {
                return Err(error(span));
            }
            let connection = Handle::new(
                WinHttpConnect(session.0, wide(&url.host).as_ptr(), url.port, 0),
                span,
            )?;
            let request = Handle::new(
                WinHttpOpenRequest(
                    connection.0,
                    wide(method).as_ptr(),
                    wide(&url.path).as_ptr(),
                    null(),
                    null(),
                    null(),
                    if url.secure { 0x00800000 } else { 0 },
                ),
                span,
            )?;
            let disable = 7u32;
            if WinHttpSetOption(request.0, 63, (&disable as *const u32).cast(), 4) == 0 {
                return Err(error(span));
            }
            let headers = wide(headers);
            let start = Instant::now();
            if WinHttpSendRequest(
                request.0,
                headers.as_ptr(),
                u32::MAX,
                body.as_ptr().cast(),
                body.len() as u32,
                body.len() as u32,
                0,
            ) == 0
                || WinHttpReceiveResponse(request.0, null_mut()) == 0
            {
                return Err(error(span));
            }
            let mut status = 0u32;
            let mut len = 4u32;
            if WinHttpQueryHeaders(
                request.0,
                19 | 0x20000000,
                null(),
                (&mut status as *mut u32).cast(),
                &mut len,
                null_mut(),
            ) == 0
            {
                return Err(error(span));
            }
            let mut body = Vec::new();
            let mut buffer = [0u8; 8192];
            loop {
                let mut read = 0;
                if start.elapsed().as_millis() > timeout as u128 * 4
                    || WinHttpReadData(
                        request.0,
                        buffer.as_mut_ptr().cast(),
                        buffer.len() as u32,
                        &mut read,
                    ) == 0
                {
                    return Err(error(span));
                }
                if read == 0 {
                    break;
                }
                if body.len() + read as usize > 1_000_000 {
                    return Err(error(span));
                }
                body.extend_from_slice(&buffer[..read as usize]);
            }
            Ok(Response {
                status: status as i32,
                body: String::from_utf8(body).map_err(|_| error(span))?,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rejects_unsafe_urls_and_headers_before_network() {
        let span = Span::default();
        for url in [
            "http://example.com/",
            "https://user:pass@example.com/",
            "https://example.com/\r\nx",
            "file:///x",
            "https://example.com/#x",
            "https://[::1]/",
        ] {
            assert!(Url::parse(url, span).is_err());
        }
        assert_eq!(
            Url::parse("https://example.com?x=1", span).unwrap().path,
            "/?x=1"
        );
        assert!(
            request(
                "GET",
                "http://localhost/",
                "",
                &BTreeMap::from([("Authorization".into(), "x\r\nInjected: y".into())]),
                100,
                span
            )
            .is_err()
        );
    }
    #[cfg(windows)]
    #[test]
    fn real_loopback_http_and_redirect_not_followed() {
        use std::{
            io::{Read, Write},
            net::TcpListener,
            thread,
        };
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        let server = thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            stream
                .set_read_timeout(Some(std::time::Duration::from_secs(5)))
                .unwrap();
            let mut buffer = [0u8; 4096];
            let n = stream.read(&mut buffer).unwrap();
            let received = String::from_utf8_lossy(&buffer[..n]);
            assert!(received.starts_with("GET /test "));
            stream.write_all(b"HTTP/1.1 302 Found\r\nLocation: https://example.invalid/secret\r\nContent-Length: 2\r\nConnection: close\r\n\r\nok").unwrap();
        });
        let response = request(
            "GET",
            &format!("http://127.0.0.1:{port}/test"),
            "",
            &BTreeMap::new(),
            2000,
            Span::default(),
        )
        .unwrap();
        server.join().unwrap();
        assert_eq!(response.status, 302);
        assert_eq!(response.body, "ok");
    }
}
