use crate::{
    catalog, compile,
    diagnostic::{Error, Result, Span, io_error},
    lexer::{TokenKind, lex},
    minecraft::{Backend, publish},
    runtime::Runtime,
    semantic::check_external,
};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::{Read, Seek, SeekFrom, Write},
    path::{Path, PathBuf},
};

const MAGIC: &[u8; 8] = b"CBPKG001";
pub const APP_PT: &str = "# Seu aplicativo começa aqui.\nimprimir(\"Olá, Command Blocks!\")\n";
pub const APP_EN: &str = "# Your application starts here.\nprint(\"Hello, Command Blocks!\")\n";
pub const DATAPACK: &str = "variavel pontos = 0\n\nfuncao ganhar_ponto() {\n    pontos = pontos + 1\n    imprimir(\"Pontuação: \" + pontos)\n}\n\nevento inicio {\n    imprimir(\"Sistema iniciado\")\n    repetir 5 vezes {\n        ganhar_ponto()\n    }\n}\n";
struct Args {
    positional: Vec<String>,
    options: BTreeMap<String, Vec<String>>,
}
impl Args {
    fn parse(args: Vec<String>) -> Result<Self> {
        let mut result = Self {
            positional: vec![],
            options: BTreeMap::new(),
        };
        let mut args = args.into_iter();
        while let Some(arg) = args.next() {
            if arg == "--help" || arg == "--version" {
                let key = arg[2..].to_string();
                if result.options.insert(key, vec!["true".into()]).is_some() {
                    return Err(argument_error());
                }
                continue;
            }
            if let Some(flag) = arg.strip_prefix("--") {
                let (key, value) = if let Some((k, v)) = flag.split_once('=') {
                    (k.to_string(), v.to_string())
                } else {
                    (flag.to_string(), args.next().ok_or_else(argument_error)?)
                };
                if ![
                    "lang",
                    "allow",
                    "max-steps",
                    "target",
                    "output",
                    "mc-version",
                    "namespace",
                ]
                .contains(&key.as_str())
                {
                    return Err(argument_error());
                }
                if key != "allow" && result.options.contains_key(&key) {
                    return Err(argument_error());
                }
                result.options.entry(key).or_default().push(value);
            } else {
                result.positional.push(arg);
            }
        }
        if !["pt-br", "en"].contains(&result.option("lang", "pt-br")) {
            return Err(argument_error());
        }
        Ok(result)
    }
    fn option<'a>(&'a self, key: &str, default: &'a str) -> &'a str {
        self.options
            .get(key)
            .and_then(|v| v.first())
            .map(|s| s.as_str())
            .unwrap_or(default)
    }
    fn pos(&self, i: usize) -> Result<&str> {
        self.positional
            .get(i)
            .map(|s| s.as_str())
            .ok_or_else(argument_error)
    }
    fn validate(&self, min: usize, max: usize, allowed: &[&str]) -> Result<()> {
        if self.positional.len() < min
            || self.positional.len() > max
            || self
                .options
                .keys()
                .any(|k| k != "lang" && !allowed.contains(&k.as_str()))
        {
            return Err(argument_error());
        }
        Ok(())
    }
    fn permissions(&self) -> Result<BTreeSet<String>> {
        let permissions: BTreeSet<_> = self
            .options
            .get("allow")
            .cloned()
            .unwrap_or_default()
            .iter()
            .map(|p| catalog::canonical(p))
            .collect();
        if permissions.iter().any(|p| !["file"].contains(&p.as_str())) {
            return Err(argument_error());
        }
        Ok(permissions)
    }
    fn steps(&self) -> Result<u64> {
        self.option("max-steps", "100000")
            .parse::<u64>()
            .ok()
            .filter(|n| *n > 0)
            .ok_or_else(argument_error)
    }
}
fn argument_error() -> Error {
    Error::new(
        "CB000",
        Span::default(),
        "Argumentos inválidos.",
        "Invalid arguments.",
    )
    .hint("Consulte cb help.", "See cb help.")
}
fn read_source(path: &Path) -> Result<String> {
    let mut source = String::new();
    fs::File::open(path)
        .map_err(|_| io_error())?
        .take(1_000_001)
        .read_to_string(&mut source)
        .map_err(|_| io_error())?;
    Ok(source
        .strip_prefix('\u{feff}')
        .unwrap_or(&source)
        .to_string())
}
pub fn main_entry(raw: Vec<String>) -> i32 {
    let english =
        raw.iter().any(|v| v == "--lang=en") || raw.windows(2).any(|w| w == ["--lang", "en"]);
    let mut file = "<cli>".to_string();
    let result = (|| {
        let args = Args::parse(raw)?;
        if args.options.contains_key("version") {
            args.validate(0, 0, &["version"])?;
            println!("Command Blocks {}", env!("CARGO_PKG_VERSION"));
            return Ok(0);
        }
        if args.options.contains_key("help") {
            args.validate(0, 1, &["help"])?;
            let topic = args.positional.first().map(String::as_str);
            validate_help_topic(topic)?;
            help(topic, english);
            return Ok(0);
        }
        if let Some(source) = embedded_source()? {
            args.validate(0, 0, &["allow", "max-steps"])?;
            file = "<bundle>".into();
            let program = compile(&source)?;
            check_external(&program)?;
            let exe = std::env::current_exe().map_err(|_| io_error())?;
            let mut runtime = Runtime::new(program, exe.parent().unwrap());
            runtime.allow = args.permissions()?;
            runtime.max_steps = args.steps()?;
            runtime.echo = true;
            runtime.run()?;
            return Ok(0);
        }
        let command = args
            .positional
            .first()
            .map(|s| s.as_str())
            .unwrap_or("help");
        match command {
            "help" => {
                args.validate(0, 2, &[])?;
                validate_help_topic(args.positional.get(1).map(String::as_str))?;
                help(args.positional.get(1).map(|s| s.as_str()), english);
            }
            "run" | "check" | "convert" | "build" | "format" => {
                let allowed = match command {
                    "run" => vec!["allow", "max-steps"],
                    "check" => vec!["target", "mc-version", "namespace"],
                    "convert" => vec!["target", "mc-version", "namespace", "output"],
                    "build" => vec!["output"],
                    _ => vec![],
                };
                args.validate(2, 2, &allowed)?;
                file = args.pos(1)?.into();
                let path = Path::new(&file);
                let source = read_source(path)?;
                let program = compile(&source)?;
                match command {
                    "run" => {
                        check_external(&program)?;
                        let mut rt = Runtime::new(
                            program,
                            path.parent()
                                .filter(|p| !p.as_os_str().is_empty())
                                .unwrap_or(Path::new(".")),
                        );
                        rt.allow = args.permissions()?;
                        rt.max_steps = args.steps()?;
                        rt.echo = true;
                        rt.run()?;
                    }
                    "check" | "convert" => {
                        let target = args.option(
                            "target",
                            if command == "convert" {
                                "minecraft"
                            } else {
                                "external"
                            },
                        );
                        if target == "minecraft" {
                            let files = Backend::new(
                                program,
                                args.option("namespace", "command_blocks"),
                                args.option("mc-version", "1.21.1"),
                            )?
                            .generate()?;
                            if command == "convert" {
                                let default = format!(
                                    "{}_datapack",
                                    path.file_stem().unwrap_or_default().to_string_lossy()
                                );
                                let output = args.option("output", &default);
                                publish(&files, Path::new(output))?;
                                println!(
                                    "{}: {output}",
                                    if english {
                                        "Datapack generated (compiler validation)"
                                    } else {
                                        "Datapack gerado (validação do compilador)"
                                    }
                                );
                            } else {
                                println!("{}", if english { "Checked." } else { "Verificado." });
                            }
                        } else if target == "external" && command == "check" {
                            check_external(&program)?;
                            println!("{}", if english { "Checked." } else { "Verificado." });
                        } else {
                            return Err(argument_error());
                        }
                    }
                    "build" => {
                        check_external(&program)?;
                        let default =
                            path.with_extension(if cfg!(windows) { "exe" } else { "bin" });
                        let output = args
                            .options
                            .get("output")
                            .map(|v| PathBuf::from(&v[0]))
                            .unwrap_or(default);
                        build(&source, &output)?;
                        println!(
                            "{}: {}",
                            if english {
                                "Packaged native interpreter and source"
                            } else {
                                "Interpretador nativo e fonte empacotados"
                            },
                            output.display()
                        );
                    }
                    "format" => {
                        let formatted = format_source(&source)?;
                        if formatted != source {
                            let backup = PathBuf::from(format!("{}.bak", path.display()));
                            fs::OpenOptions::new()
                                .write(true)
                                .create_new(true)
                                .open(backup)
                                .map_err(|_| io_error())?
                                .write_all(source.as_bytes())
                                .map_err(|_| io_error())?;
                            fs::write(path, formatted).map_err(|_| io_error())?;
                        }
                        println!("{}", if english { "Formatted." } else { "Formatado." });
                    }
                    _ => unreachable!(),
                }
            }
            "test" => {
                args.validate(1, 2, &[])?;
                let path = Path::new(args.positional.get(1).map(|s| s.as_str()).unwrap_or("."));
                let mut paths = vec![];
                collect_tests(path, &mut paths)?;
                paths.sort();
                let (mut passed, mut failed) = (0, 0);
                for path in paths {
                    let program = read_source(&path).and_then(|s| compile(&s));
                    let program = match program {
                        Ok(p) => p,
                        Err(e) => {
                            eprintln!("{}", e.render(&path.display().to_string(), english));
                            failed += 1;
                            continue;
                        }
                    };
                    for stmt in &program.body {
                        if let crate::parser::StmtKind::Test(_, body) = &stmt.kind {
                            let mut rt = Runtime::new(
                                program.clone(),
                                path.parent().unwrap_or(Path::new(".")),
                            );
                            match rt.run_test(body) {
                                Ok(()) => passed += 1,
                                Err(e) => {
                                    failed += 1;
                                    eprintln!("{}", e.render(&path.display().to_string(), english));
                                }
                            }
                        }
                    }
                }
                println!(
                    "{passed} {}, {failed} {}",
                    if english { "passed" } else { "passaram" },
                    if english { "failed" } else { "falharam" }
                );
                if passed == 0 && failed == 0 {
                    println!(
                        "{}",
                        if english {
                            "No tests found."
                        } else {
                            "Nenhum teste encontrado."
                        }
                    );
                }
                return Ok(if failed == 0 { 0 } else { 1 });
            }
            "create" => {
                args.validate(3, 3, &[])?;
                let template = args.pos(1)?;
                let (source, readme) = match template {
                    "app" => (
                        if english { APP_EN } else { APP_PT },
                        "# Command Blocks app\n\nNa pasta do projeto / From the project directory:\n\n```text\ncb run main.cb\ncb check main.cb\ncb build main.cb --output app.exe\n```\n\nAplicativo de terminal, sem interface gráfica. O executável usa o mesmo sistema/arquitetura do cb.\nTerminal application, no graphical UI. The executable targets the same platform/architecture as cb.\n",
                    ),
                    "minecraft-datapack" => (
                        DATAPACK,
                        "# Command Blocks\n\ncb convert main.cb --target minecraft --output datapack\n",
                    ),
                    _ => return Err(argument_error()),
                };
                publish(
                    &BTreeMap::from([
                        ("main.cb".into(), source.into()),
                        ("README.md".into(), readme.into()),
                        (".gitignore".into(), ".env\n*.secret\n".into()),
                    ]),
                    Path::new(args.pos(2)?),
                )?;
                println!(
                    "{}",
                    if english {
                        "Project created."
                    } else {
                        "Projeto criado."
                    }
                );
            }
            _ => return Err(argument_error()),
        }
        Ok(0)
    })();
    match result {
        Ok(code) => code,
        Err(e) => {
            eprintln!("{}", e.render(&file, english));
            1
        }
    }
}
fn help(topic: Option<&str>, english: bool) {
    if let Some(topic) = topic
        && let Some((usage, pt, en, example)) = command_help(topic)
    {
        println!(
            "Command Blocks {}\n\n{}: {usage}\n\n{}\n\n{}:\n  {example}\n\n--lang pt-br|en",
            env!("CARGO_PKG_VERSION"),
            if english { "Usage" } else { "Uso" },
            if english { en } else { pt },
            if english { "Example" } else { "Exemplo" }
        );
        return;
    }
    match topic.map(catalog::canonical).as_deref() {
        Some("text") => println!(
            "text/texto: print/imprimir, upper/maiusculas, lower/minusculas, trim/aparar, join/juntar, split/separar, replace/substituir, contains/contem, starts_with/comeca_com, ends_with/termina_com, length/tamanho, string/converter, number/numero, format/formatar"
        ),
        Some("minecraft") => println!(
            "Minecraft Java 1.21 / 1.21.1\nminecraft.message/mensagem(selector, text)\nminecraft.give_item/dar_item(selector, item, count)\nminecraft.teleport/teleportar(selector, x, y, z)\nminecraft.weather/clima(text)\nminecraft.time/horario(integer)\ncb convert FILE --target minecraft --mc-version 1.21.1 --output DIR"
        ),
        Some(_) => println!(
            "{}",
            if english {
                "See docs/bibliotecas.md (in Portuguese) for the implemented library catalog."
            } else {
                "Consulte docs/bibliotecas.md para o catálogo implementado."
            }
        ),
        None => {
            println!("Command Blocks {}\n", env!("CARGO_PKG_VERSION"));
            println!(
                "{}: cb COMMAND [OPTIONS]\n",
                if english { "Usage" } else { "Uso" }
            );
            for name in [
                "run", "check", "convert", "build", "format", "test", "create", "help",
            ] {
                let (_, pt, en, _) = command_help(name).unwrap();
                println!("  {name:<9} {}", if english { en } else { pt });
            }
            println!("\n  --help\n  --version\n  --lang pt-br|en\n");
            println!(
                "{}:\n  cb run examples/ola.cb\n  cb help convert --lang en",
                if english { "Examples" } else { "Exemplos" }
            );
        }
    }
}
fn command_help(name: &str) -> Option<(&'static str, &'static str, &'static str, &'static str)> {
    Some(match name {
        "run" => (
            "cb run FILE [--allow CAPABILITY] [--max-steps N]",
            "Executa um programa local. Permissões exigem declaração no fonte e autorização na CLI.",
            "Runs a local program. Permissions require both a source declaration and a CLI grant.",
            "cb run examples/ola.cb --lang pt-br",
        ),
        "check" => (
            "cb check FILE [--target external|minecraft] [--mc-version VERSION] [--namespace NAME]",
            "Verifica o programa sem executá-lo. Não comprova conectividade de serviços.",
            "Checks the program without executing it. Does not verify service connectivity.",
            "cb check examples/minecraft.cb --target minecraft",
        ),
        "convert" => (
            "cb convert FILE --target minecraft [--output DIR] [--mc-version VERSION] [--namespace NAME]",
            "Gera um novo datapack Java 1.21/1.21.1. Recusa recursos incompatíveis e destinos existentes.",
            "Generates a new Java 1.21/1.21.1 datapack. Rejects incompatible features and existing destinations.",
            "cb convert examples/pontos.cb --target minecraft --output meu_datapack",
        ),
        "build" => (
            "cb build FILE [--output APP]",
            "Empacota o interpretador nativo e o fonte para o mesmo sistema e arquitetura.",
            "Packages the native interpreter and source for the same operating system and architecture.",
            "cb build examples/hello.cb --output hello.exe",
        ),
        "format" => (
            "cb format FILE",
            "Formata a indentação. Cria backup .bak e recusa substituir um backup existente.",
            "Formats indentation. Creates a .bak backup and refuses to overwrite an existing backup.",
            "cb format examples/ola.cb",
        ),
        "test" => (
            "cb test [PATH]",
            "Executa os blocos de teste encontrados, sem conceder permissões externas.",
            "Runs discovered test blocks without granting external permissions.",
            "cb test examples/tests.cb",
        ),
        "create" => (
            "cb create app|minecraft-datapack DIR",
            "Cria um projeto novo.",
            "Creates a new project.",
            "cb create app meu_app",
        ),
        "help" => (
            "cb help [COMMAND|LIBRARY]",
            "Mostra ajuda de comandos e bibliotecas.",
            "Shows command and library help.",
            "cb help run --lang en",
        ),
        _ => return None,
    })
}
fn validate_help_topic(topic: Option<&str>) -> Result<()> {
    if let Some(topic) = topic
        && command_help(topic).is_none()
        && !catalog::MODULES.contains(&catalog::canonical(topic).as_str())
    {
        return Err(argument_error());
    }
    Ok(())
}
fn collect_tests(path: &Path, out: &mut Vec<PathBuf>) -> Result<()> {
    let meta = fs::symlink_metadata(path).map_err(|_| io_error())?;
    if meta.file_type().is_symlink() {
        return Ok(());
    }
    if meta.is_file() {
        if path.extension().is_some_and(|e| e == "cb") {
            out.push(path.to_path_buf());
        }
    } else {
        for entry in fs::read_dir(path).map_err(|_| io_error())? {
            let entry = entry.map_err(|_| io_error())?;
            let name = entry.file_name();
            let name = name.to_string_lossy();
            if !name.starts_with('.')
                && !["target", "node_modules", "dist"].contains(&name.as_ref())
            {
                collect_tests(&entry.path(), out)?;
            }
        }
    }
    Ok(())
}
pub fn format_source(source: &str) -> Result<String> {
    compile(source)?;
    let mut depth = 0i32;
    let mut out = String::new();
    for line in source.lines() {
        let text = line.trim();
        let tokens = lex(text)?;
        let leading = tokens
            .iter()
            .take_while(
                |t| matches!(&t.kind,TokenKind::Symbol(s) if ["}","]",")"].contains(&s.as_str())),
            )
            .count() as i32;
        if !text.is_empty() {
            out.push_str(&"    ".repeat((depth - leading).max(0) as usize));
            out.push_str(text);
        }
        out.push('\n');
        for t in tokens {
            if let TokenKind::Symbol(s) = t.kind {
                if ["{", "[", "("].contains(&s.as_str()) {
                    depth += 1;
                } else if ["}", "]", ")"].contains(&s.as_str()) {
                    depth -= 1;
                }
            }
        }
    }
    Ok(out.trim_end().to_string() + "\n")
}
fn embedded_source() -> Result<Option<String>> {
    let mut f =
        fs::File::open(std::env::current_exe().map_err(|_| io_error())?).map_err(|_| io_error())?;
    let size = f.metadata().map_err(|_| io_error())?.len();
    if size < 16 {
        return Ok(None);
    }
    f.seek(SeekFrom::End(-16)).map_err(|_| io_error())?;
    let mut footer = [0; 16];
    f.read_exact(&mut footer).map_err(|_| io_error())?;
    if &footer[8..] != MAGIC {
        return Ok(None);
    }
    let length = u64::from_le_bytes(footer[..8].try_into().unwrap());
    if length > 1_000_000 || length + 16 > size {
        return Err(io_error());
    }
    f.seek(SeekFrom::End(-16 - length as i64))
        .map_err(|_| io_error())?;
    let mut bytes = vec![0; length as usize];
    f.read_exact(&mut bytes).map_err(|_| io_error())?;
    Ok(Some(String::from_utf8(bytes).map_err(|_| io_error())?))
}
pub fn build(source: &str, output: &Path) -> Result<()> {
    if output.exists() {
        return Err(Error::new(
            "CB205",
            Span::default(),
            "O destino já existe.",
            "Output already exists.",
        ));
    }
    let executable = std::env::current_exe().map_err(|_| io_error())?;
    let mut original = fs::File::open(&executable).map_err(|_| io_error())?;
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(output)
        .map_err(|_| io_error())?;
    std::io::copy(&mut original, &mut file).map_err(|_| io_error())?;
    file.write_all(source.as_bytes()).map_err(|_| io_error())?;
    file.write_all(&(source.len() as u64).to_le_bytes())
        .map_err(|_| io_error())?;
    file.write_all(MAGIC).map_err(|_| io_error())?;
    fs::set_permissions(
        output,
        fs::metadata(executable)
            .map_err(|_| io_error())?
            .permissions(),
    )
    .map_err(|_| io_error())?;
    Ok(())
}
