use std::{
    fs,
    path::PathBuf,
    process::Command,
    sync::atomic::{AtomicUsize, Ordering},
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
        "run", "check", "convert", "build", "format", "test", "create", "help",
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
fn formatting_creates_backups() {
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
}
