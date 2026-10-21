use command_blocks::{compile, integrations::valid_command, runtime::Runtime};
use std::{
    fs,
    io::{Read, Write},
    net::TcpListener,
    path::PathBuf,
    process::Command,
    sync::atomic::{AtomicUsize, Ordering},
    thread,
};
static NEXT: AtomicUsize = AtomicUsize::new(0);
struct Temp(PathBuf);
impl Temp {
    fn new() -> Self {
        let p = std::env::temp_dir().join(format!(
            "cb-cli-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&p).unwrap();
        Self(p)
    }
    fn cb(&self, args: &[&str]) -> std::process::Output {
        Command::new(env!("CARGO_BIN_EXE_cb"))
            .current_dir(&self.0)
            .args(args)
            .output()
            .unwrap()
    }
}
impl Drop for Temp {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[test]
fn help_and_version_are_localized_non_executing_and_consistent() {
    let tmp = Temp::new();
    let english = tmp.cb(&["help", "run", "--lang", "en"]);
    assert!(english.status.success());
    assert!(english.stderr.is_empty());
    assert_eq!(
        String::from_utf8_lossy(&english.stdout).replace("\r\n", "\n"),
        include_str!("golden/help-run.en.txt").replace("\r\n", "\n")
    );
    assert_eq!(
        english.stdout,
        tmp.cb(&["run", "--help", "--lang=en"]).stdout
    );
    let portuguese = tmp.cb(&["help", "run"]);
    assert!(portuguese.status.success());
    assert!(portuguese.stderr.is_empty());
    assert_eq!(
        String::from_utf8_lossy(&portuguese.stdout).replace("\r\n", "\n"),
        include_str!("golden/help-run.pt-br.txt").replace("\r\n", "\n")
    );
    assert_eq!(
        tmp.cb(&["help", "--lang", "en"]).stdout,
        tmp.cb(&["--help", "--lang", "en"]).stdout
    );
    assert_eq!(
        tmp.cb(&["--version"]).stdout,
        tmp.cb(&["--lang=en", "--version"]).stdout
    );
    for args in [
        vec!["run", "--version"],
        vec!["--help", "--version"],
        vec!["--help", "--help"],
        vec!["help", "unknown"],
        vec!["unknown", "--help"],
        vec!["--help", "--lang", "invalid"],
    ] {
        let result = tmp.cb(&args);
        assert_eq!(result.status.code(), Some(1), "{args:?}");
        assert!(result.stdout.is_empty());
        assert!(String::from_utf8_lossy(&result.stderr).contains("CB000"));
    }
    assert!(!tmp.0.join("main.cb").exists());
    for command in [
        "run", "check", "convert", "build", "format", "test", "create", "install", "list", "info",
        "help",
    ] {
        for lang in ["pt-br", "en"] {
            let result = tmp.cb(&[command, "--help", "--lang", lang]);
            assert!(result.status.success(), "{command} {lang}");
            assert!(result.stderr.is_empty());
            let output = String::from_utf8(result.stdout).unwrap();
            assert!(output.contains(&format!("cb {command}")));
            assert!(output.contains(if lang == "en" { "Example:" } else { "Exemplo:" }));
        }
    }
}

#[test]
fn app_template_runs_in_both_languages_and_preserves_existing_projects() {
    let tmp = Temp::new();
    for (directory, lang, expected) in [
        ("app_pt", "pt-br", "Olá, Command Blocks!"),
        ("app_en", "en", "Hello, Command Blocks!"),
    ] {
        assert!(
            tmp.cb(&["create", "app", directory, "--lang", lang])
                .status
                .success()
        );
        let source = format!("{directory}/main.cb");
        let result = tmp.cb(&["run", &source]);
        assert!(result.status.success());
        assert_eq!(String::from_utf8(result.stdout).unwrap().trim(), expected);
        assert!(!tmp.cb(&["create", "app", directory]).status.success());
    }
}

#[test]
fn cli_runs_checks_builds_and_tests() {
    let tmp = Temp::new();
    fs::write(tmp.0.join("main.cb"), "print(\"native\")").unwrap();
    assert!(tmp.cb(&["check", "main.cb"]).status.success());
    assert_eq!(
        String::from_utf8(tmp.cb(&["run", "main.cb"]).stdout)
            .unwrap()
            .trim(),
        "native"
    );
    let exe = if cfg!(windows) { "app.exe" } else { "app" };
    let result = tmp.cb(&["build", "main.cb", "--output", exe]);
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let result = Command::new(tmp.0.join(exe)).output().unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    assert_eq!(String::from_utf8(result.stdout).unwrap().trim(), "native");
    let help = Command::new(tmp.0.join(exe))
        .args(["--help", "--lang", "en"])
        .output()
        .unwrap();
    assert!(help.status.success());
    assert!(help.stderr.is_empty());
    let help = String::from_utf8(help.stdout).unwrap();
    assert!(help.contains("Usage: cb COMMAND"));
    assert!(!help.lines().any(|line| line == "native"));
    assert!(
        !tmp.cb(&["build", "main.cb", "--output", exe])
            .status
            .success()
    );
    fs::write(tmp.0.join("tests.cb"),"teste \"soma\" { verificar(2 + 2 == 4) }\ntest \"text\" { assert(text.upper(\"a\") == \"A\") }").unwrap();
    let result = tmp.cb(&["test", "tests.cb", "--lang", "en"]);
    assert!(result.status.success());
    assert!(String::from_utf8_lossy(&result.stdout).contains("2 passed"));
    fs::write(tmp.0.join("fail.cb"), "test \"bad\" { assert(false) }").unwrap();
    assert!(!tmp.cb(&["test", "fail.cb"]).status.success());
}
#[test]
fn cli_creates_converts_and_leaves_no_partial_pack_on_error() {
    let tmp = Temp::new();
    assert!(
        tmp.cb(&["create", "minecraft-datapack", "map"])
            .status
            .success()
    );
    assert!(
        tmp.cb(&[
            "convert",
            "map/main.cb",
            "--target",
            "minecraft",
            "--output",
            "pack"
        ])
        .status
        .success()
    );
    assert!(tmp.0.join("pack/pack.mcmeta").exists());
    assert!(tmp.0.join("pack/cb-report.json").exists());
    fs::write(
        tmp.0.join("invalid.cb"),
        "permission file\nfile.read(\"private.txt\")",
    )
    .unwrap();
    let result = tmp.cb(&[
        "convert",
        "invalid.cb",
        "--output",
        "badpack",
        "--lang",
        "en",
    ]);
    assert!(!result.status.success());
    assert!(!tmp.0.join("badpack").exists());
    let error = String::from_utf8(result.stderr).unwrap();
    assert!(error.contains("CB201"));
    assert!(error.contains("invalid.cb:2:"));
    assert!(error.contains("Suggestion"));
    assert!(!error.contains("private.txt"));
}
#[test]
fn formatting_backups_and_optional_registry() {
    let tmp = Temp::new();
    let source = "if true {\nprint(1)\n}\n";
    fs::write(tmp.0.join("main.cb"), source).unwrap();
    assert!(tmp.cb(&["format", "main.cb"]).status.success());
    assert_eq!(
        fs::read_to_string(tmp.0.join("main.cb.bak")).unwrap(),
        source
    );
    assert!(
        fs::read_to_string(tmp.0.join("main.cb"))
            .unwrap()
            .contains("    print")
    );
    assert!(tmp.cb(&["install", "minecraft_bridge"]).status.success());
    assert!(tmp.cb(&["list"]).status.success());
    assert!(!tmp.cb(&["install", "unknown"]).status.success());
    assert!(tmp.cb(&["create", "discord-bot", "bot"]).status.success());
}
#[test]
fn discord_scaffold_fails_honestly_and_never_leaks_token() {
    let tmp = Temp::new();
    fs::write(tmp.0.join(".cb-libraries"), "discord\n").unwrap();
    fs::write(
        tmp.0.join("bot.cb"),
        "import discord\npermission net\ndiscord.start(secret(\"DISCORD_TOKEN\"))",
    )
    .unwrap();
    let result = Command::new(env!("CARGO_BIN_EXE_cb"))
        .current_dir(&tmp.0)
        .args(["run", "bot.cb", "--allow", "net", "--lang", "en"])
        .env("DISCORD_TOKEN", "token-must-not-appear")
        .output()
        .unwrap();
    assert!(!result.status.success());
    let error = String::from_utf8(result.stderr).unwrap();
    assert!(error.contains("CB202"));
    assert!(error.contains("not implemented"));
    assert!(!error.contains("token-must-not-appear"));
    assert!(result.stdout.is_empty());
}
fn bridge_runtime(tmp: &Temp) -> Runtime {
    fs::write(tmp.0.join(".cb-libraries"), "minecraft_bridge\n").unwrap();
    let program = compile("import minecraft_bridge\npermission minecraft_bridge\nminecraft_bridge.send(\"say Hello\")").unwrap();
    let mut rt = Runtime::new(program, &tmp.0);
    rt.allow.insert("minecraft_bridge".into());
    rt.environment.remove("CB_BRIDGE_URL");
    rt.environment.remove("CB_BRIDGE_TOKEN");
    rt
}
#[test]
fn bridge_is_disabled_without_auth_and_blocks_command_injection() {
    let tmp = Temp::new();
    let mut rt = bridge_runtime(&tmp);
    assert_eq!(rt.run().unwrap_err().code, "CB301");
    for command in [
        "stop",
        "op Steve",
        "say hello\nstop",
        "tp @a 1 2 3",
        "tp Steve 999999999 2 3",
    ] {
        assert!(!valid_command(command));
    }
    assert!(valid_command("say hello"));
    assert!(valid_command("tp Steve 0 80 -3"));
    rt.environment.insert(
        "CB_BRIDGE_URL".into(),
        "http://example.com:80/command".into(),
    );
    rt.environment
        .insert("CB_BRIDGE_TOKEN".into(), "a".repeat(24));
    assert_eq!(rt.run().unwrap_err().code, "CB301");
}
#[test]
fn bridge_sends_authenticated_request_and_requires_acknowledgement() {
    for success in [true, false] {
        let tmp = Temp::new();
        let mut rt = bridge_runtime(&tmp);
        let server = TcpListener::bind("127.0.0.1:0").unwrap();
        let port = server.local_addr().unwrap().port();
        rt.environment.insert(
            "CB_BRIDGE_URL".into(),
            format!("http://127.0.0.1:{port}/command"),
        );
        rt.environment
            .insert("CB_BRIDGE_TOKEN".into(), "a".repeat(24));
        let worker = thread::spawn(move || {
            let (mut stream, _) = server.accept().unwrap();
            stream
                .set_read_timeout(Some(std::time::Duration::from_secs(3)))
                .unwrap();
            let mut data = vec![];
            let mut buf = [0; 1024];
            loop {
                let n = stream.read(&mut buf).unwrap();
                assert!(n > 0);
                data.extend_from_slice(&buf[..n]);
                if let Some(end) = data.windows(4).position(|w| w == b"\r\n\r\n") {
                    let headers = String::from_utf8_lossy(&data[..end]);
                    let len: usize = headers
                        .lines()
                        .find_map(|l| l.strip_prefix("Content-Length: "))
                        .unwrap()
                        .parse()
                        .unwrap();
                    if data.len() >= end + 4 + len {
                        break;
                    }
                }
            }
            let request = String::from_utf8(data).unwrap();
            assert!(request.contains(&format!("Authorization: Bearer {}", "a".repeat(24))));
            assert!(request.contains("{\"command\":\"say Hello\"}"));
            let body = if success {
                "{\"ok\":true}"
            } else {
                "{\"ok\":false}"
            };
            write!(
                stream,
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
                body.len()
            )
            .unwrap();
        });
        let result = rt.run();
        worker.join().unwrap();
        if success {
            result.unwrap();
        } else {
            assert_eq!(result.unwrap_err().code, "CB304");
        }
    }
}
