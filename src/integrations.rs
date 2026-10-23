//! Optional integration boundary. Discord is deliberately a scaffold in 0.1.
use crate::{
    diagnostic::{Error, Result, Span},
    runtime::Runtime,
    value::{Value, quote},
};
use std::{
    fs,
    io::{Read, Write},
    net::{Ipv4Addr, SocketAddr, TcpStream},
    time::{Duration, Instant},
};

pub fn dispatch(runtime: &mut Runtime, name: &str, args: &[Value], span: Span) -> Result<Value> {
    let module = name.split('.').next().unwrap_or("");
    let enabled = fs::read_to_string(runtime.root.join(".cb-libraries")).unwrap_or_default();
    if !runtime.program.imports.contains(module) || !enabled.lines().any(|line| line == module) {
        return Err(Error::new(
            "CB202",
            span,
            "Biblioteca opcional não habilitada/importada.",
            "Optional library is not enabled/imported.",
        )
        .hint(
            format!("Execute cb install {module} e importe a biblioteca."),
            format!("Run cb install {module} and import the library."),
        ));
    }
    if module == "discord" {
        if name == "discord.connect" {
            let Value::Secret(token) = &args[0] else {
                return Err(crate::diagnostic::type_error(span));
            };
            let mut client = crate::discord::Client::new(token.clone(), span)?;
            let profile = client.authenticate(span)?;
            runtime.discord = Some(client);
            return Ok(profile);
        }
        if name == "discord.send" {
            let (Value::Text(channel), Value::Text(content)) = (&args[0], &args[1]) else {
                return Err(crate::diagnostic::type_error(span));
            };
            let content = runtime.redact(content);
            let client = runtime.discord.as_mut().ok_or_else(|| {
                Error::new(
                    "CB301",
                    span,
                    "Conecte o bot primeiro com discord.conectar().",
                    "Connect the bot first with discord.connect().",
                )
            })?;
            return client.send(channel, &content, span);
        }
        return Err(Error::new(
            "CB202",
            span,
            "Discord possui apenas a interface inicial; transporte ainda não implementado.",
            "Discord provides only the initial interface; transport is not implemented yet.",
        )
        .hint(
            "Consulte docs/integracoes.md. Nenhuma conexão foi iniciada.",
            "See docs/integracoes.md (in Portuguese). No connection was started.",
        ));
    }
    let command = match &args[0] {
        Value::Text(s) => runtime.redact(s),
        _ => return Err(crate::diagnostic::type_error(span)),
    };
    let token = runtime
        .environment
        .get("CB_BRIDGE_TOKEN")
        .cloned()
        .unwrap_or_default();
    let url = runtime
        .environment
        .get("CB_BRIDGE_URL")
        .cloned()
        .unwrap_or_default();
    let port = url
        .strip_prefix("http://127.0.0.1:")
        .and_then(|s| s.strip_suffix("/command"))
        .and_then(|s| s.parse::<u16>().ok())
        .filter(|p| *p > 0);
    if port.is_none()
        || token.len() < 24
        || !token
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"_-".contains(&b))
    {
        return Err(Error::new("CB301",span,"Ponte desativada ou configuração insegura.","Bridge disabled or unsafe configuration.")
            .hint("Defina CB_BRIDGE_URL=http://127.0.0.1:PORT/command e CB_BRIDGE_TOKEN (24+ letras, dígitos, _ ou -).",
                  "Set CB_BRIDGE_URL=http://127.0.0.1:PORT/command and CB_BRIDGE_TOKEN (24+ letters, digits, _ or -)."));
    }
    if !valid_command(&command) {
        return Err(Error::new(
            "CB301",
            span,
            "Comando não permitido na ponte inicial.",
            "Command is not allowed by the initial bridge.",
        )
        .hint(
            "Use say ou tp com jogador e coordenadas inteiras.",
            "Use say or tp with a player name and integer coordinates.",
        ));
    }
    let now = Instant::now();
    runtime
        .bridge_requests
        .retain(|t| now.duration_since(*t) < Duration::from_secs(1));
    if runtime.bridge_requests.len() >= 10 {
        return Err(Error::new(
            "CB302",
            span,
            "Limite de requisições atingido.",
            "Request rate limit reached.",
        ));
    }
    runtime.bridge_requests.push(now);
    let result = send(port.unwrap(), &token, &command);
    if !result {
        return Err(Error::new(
            "CB304",
            span,
            "A ponte não confirmou a execução; detalhes privados omitidos.",
            "Bridge did not acknowledge execution; private details omitted.",
        )
        .hint(
            "Verifique o servidor local, autenticação e protocolo documentado.",
            "Check the local server, authentication and documented protocol.",
        ));
    }
    Ok(Value::Null)
}
pub fn valid_command(command: &str) -> bool {
    if command.len() > 1024 || command.chars().any(|c| c.is_control()) {
        return false;
    }
    if let Some(message) = command.strip_prefix("say ") {
        return !message.is_empty();
    }
    let words: Vec<_> = command.split(' ').collect();
    words.len() == 5
        && words[0] == "tp"
        && !words[1].is_empty()
        && words[1].len() <= 16
        && words[1]
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'_')
        && words[2..].iter().all(|s| {
            s.parse::<i32>()
                .is_ok_and(|n| (-30_000_000..=30_000_000).contains(&n))
        })
}
fn send(port: u16, token: &str, command: &str) -> bool {
    let attempt = || -> std::io::Result<bool> {
        let mut stream = TcpStream::connect_timeout(
            &SocketAddr::from((Ipv4Addr::LOCALHOST, port)),
            Duration::from_secs(3),
        )?;
        stream.set_read_timeout(Some(Duration::from_secs(3)))?;
        stream.set_write_timeout(Some(Duration::from_secs(3)))?;
        let body = format!("{{\"command\":{}}}", quote(command));
        write!(
            stream,
            "POST /command HTTP/1.1\r\nHost: 127.0.0.1:{port}\r\nAuthorization: Bearer {token}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
            body.len()
        )?;
        let mut response = String::new();
        stream.take(65537).read_to_string(&mut response)?;
        if response.len() > 65536 {
            return Ok(false);
        }
        let Some((header, body)) = response.split_once("\r\n\r\n") else {
            return Ok(false);
        };
        let status = header.lines().next().unwrap_or("");
        let body: String = body.chars().filter(|c| !c.is_whitespace()).collect();
        Ok(
            (status.starts_with("HTTP/1.1 200 ") || status.starts_with("HTTP/1.0 200 "))
                && body == "{\"ok\":true}",
        )
    };
    attempt().unwrap_or(false)
}
