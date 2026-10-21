use command_blocks::{
    cli::format_source,
    compile,
    diagnostic::Error,
    minecraft::{Backend, publish},
    runtime::Runtime,
    semantic::check_external,
};
use std::{
    collections::BTreeMap,
    fs,
    path::PathBuf,
    sync::atomic::{AtomicUsize, Ordering},
};
static NEXT: AtomicUsize = AtomicUsize::new(0);
struct Temp(PathBuf);
impl Temp {
    fn new() -> Self {
        let p = std::env::temp_dir().join(format!(
            "cb-test-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&p).unwrap();
        Self(p)
    }
}
impl Drop for Temp {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
fn run(source: &str) -> Result<Vec<String>, Error> {
    let mut rt = Runtime::new(compile(source)?, ".");
    rt.run()?;
    Ok(rt.output)
}
fn convert(source: &str) -> Result<BTreeMap<String, String>, Error> {
    Backend::new(compile(source)?, "testing", "1.21.1")?.generate()
}

#[test]
fn windows_require_permissions_and_reject_minecraft() {
    assert_eq!(
        compile("window.open(\"App\", 320, 240)").unwrap_err().code,
        "CB301"
    );
    let source = "permission window\nwindow.open(\"App\", 320, 240)";
    assert_eq!(run(source).unwrap_err().code, "CB301");
    assert_eq!(convert(source).unwrap_err().code, "CB201");
    compile(include_str!("../examples/window.cb")).unwrap();
}

#[test]
fn json_library_bilingual_and_private_errors() {
    let output = run(include_str!("../examples/json.cb")).unwrap();
    assert_eq!(output[0], "Alex");
    assert_eq!(run("print(json.valid(\"[1,]\"))").unwrap(), ["false"]);
    assert_eq!(
        run("print(json.stringify(json.parse(\"[true,2,null]\")))").unwrap(),
        ["[true,2,null]"]
    );
    let error = run("json.ler(\"private-invalid-json\")").unwrap_err();
    assert_eq!(error.code, "CB107");
    assert!(!format!("{error:?}").contains("private-invalid-json"));
    assert_eq!(convert("json.valid(\"{}\")").unwrap_err().code, "CB201");
}

#[test]
fn network_permissions_and_environment_are_explicit() {
    assert_eq!(
        compile("net.get(\"https://example.com/\")")
            .unwrap_err()
            .code,
        "CB301"
    );
    assert_eq!(
        run("permissao rede\nrede.get(\"https://example.com/\")")
            .unwrap_err()
            .code,
        "CB301"
    );
    assert_eq!(
        convert("permission net\nnet.get(\"https://example.com/\")")
            .unwrap_err()
            .code,
        "CB201"
    );
    compile("permission net\nnet.request(\"POST\", \"https://example.com/\", \"{}\", {\"Content-Type\": \"application/json\"}, 5000)").unwrap();
}

#[test]
fn bilingual_variables_constants_and_types() {
    assert_eq!(
        run(
            "variavel pontos = 2\nconstante limite = 5\npontos = pontos + limite\nimprimir(pontos)"
        )
        .unwrap(),
        ["7"]
    );
    assert_eq!(
        run("variable points = 2\nconstant limit = 5\npoints = points + limit\nprint(points)")
            .unwrap(),
        ["7"]
    );
    assert_eq!(compile("constant a = 1\na = 2").unwrap_err().code, "CB104");
    assert_eq!(
        compile("variable a = 1\na = \"bad\"").unwrap_err().code,
        "CB103"
    );
    assert_eq!(compile("print(missing)").unwrap_err().code, "CB101");
}
#[test]
fn operators_and_overflow_have_defined_semantics() {
    assert_eq!(run("print(2 + 3 * 4)\nprint(8 / 2)\nprint(8 % 3)\nprint(2147483647 + 1)\nprint(-2147483648)\nprint(-2147483648 % -1)").unwrap(),["14","4","2","-2147483648","-2147483648","0"]);
    assert_eq!(run("print(1 / 0)").unwrap_err().code, "CB108");
    assert_eq!(compile("print(true + 1)").unwrap_err().code, "CB103");
    assert_eq!(
        run("print(false and (1 / 0 > 2))\nprint(true or (1 / 0 > 2))\nprint(true == 1)").unwrap(),
        ["false", "true", "false"]
    );
}
#[test]
fn text_unicode_and_math() {
    assert_eq!(run("texto.imprimir(texto.maiusculas(\"olá\"))\nprint(text.length(\"ação\"))\nprint(text.replace(\"abcabc\",\"a\",\"z\"))\nprint(text.format(\"Hi {name}\",{\"name\":\"Alex\"}))\nprint(math.sqrt(9))\nprint(matematica.aleatorio(7,7))").unwrap(),["OLÁ","4","zbczbc","Hi Alex","3","7"]);
    assert_eq!(run("print(math.sqrt(-1))").unwrap_err().code, "CB302");
    assert_eq!(run("print(\"Olá, \" + 3)").unwrap(), ["Olá, 3"]);
}
#[test]
fn lists_maps_indices_and_mutation() {
    assert_eq!(run("variavel xs = lista.criar()\nlista.adicionar(xs, 3)\nlista.adicionar(xs, 1)\nxs[0] = 2\nprint(list.sort(xs))\nvariable m = {\"name\":\"Steve\"}\nmap.set(m,\"name\",\"Alex\")\nprint(m[\"name\"])\nprint(list.length(xs))\nprint(map.keys(m))").unwrap(),["[1,2]","Alex","2","[\"name\"]"]);
    assert_eq!(run("print([1][-1])").unwrap_err().code, "CB107");
    assert_eq!(run("print({}[\"absent\"])").unwrap_err().code, "CB107");
    // Self insertion is a snapshot, not a reference cycle.
    assert_eq!(
        run("variable a = []\nlist.add(a,a)\nprint(a)").unwrap(),
        ["[[]]"]
    );
}
#[test]
fn loops_conditions_and_scopes() {
    assert_eq!(run("variable x = 0\nrepeat 3 times { x = x + 1 }\nwhile x < 5 { x = x + 1 }\nif x == 5 { print(\"yes\") } else { print(\"no\") }\nfor each v in [1,2] { print(v) }\nprint(x)").unwrap(),["yes","1","2","5"]);
    assert_eq!(
        compile("if true { variable inner = 1 }\nprint(inner)")
            .unwrap_err()
            .code,
        "CB101"
    );
    assert_eq!(compile("if 1 { print(1) }").unwrap_err().code, "CB103");
    let mut rt = Runtime::new(compile("while true { }").unwrap(), ".");
    rt.max_steps = 20;
    assert_eq!(rt.run().unwrap_err().code, "CB302");
}
#[test]
fn functions_returns_events_and_runtime_types() {
    assert_eq!(run("function sum(a,b) { return a + b }\nprint(sum(2,3))\nvariable value = 4\nevent start { print(value) }\nevent custom { print(\"custom\") }\nevent.emit(\"custom\")").unwrap(),["5","custom","4"]);
    assert_eq!(
        run("function f(a) { return a - 1 }\nf(\"bad\")")
            .unwrap_err()
            .code,
        "CB103"
    );
    assert_eq!(
        compile("function f(a) { return a }\nf()").unwrap_err().code,
        "CB105"
    );
    assert_eq!(compile("return 1").unwrap_err().code, "CB101");
    assert_eq!(run("function f() { f() }\nf()").unwrap_err().code, "CB302");
}
#[test]
fn lexer_parser_errors_and_localized_locations() {
    for source in [
        "variable x =",
        "if true {",
        "print(\"unfinished)",
        "print(1) print(2)",
        "print(@)",
    ] {
        assert!(compile(source).is_err(), "{source}");
    }
    let error = compile("\nprint(\"sensitive\" - 1)").unwrap_err();
    assert_eq!(error.span.line, 2);
    assert!(error.span.column > 0);
    assert!(error.render("program.cb", false).contains("Sugestão"));
    assert!(
        error
            .render("program.cb", true)
            .contains("Incompatible types")
    );
    assert!(!error.render("program.cb", true).contains("sensitive"));
    assert_eq!(
        compile(&format!("print({}1{})", "(".repeat(300), ")".repeat(300)))
            .unwrap_err()
            .code,
        "CB106"
    );
    assert_eq!(
        compile(&format!("print({})", vec!["1"; 300].join("+")))
            .unwrap_err()
            .code,
        "CB106"
    );
}
#[test]
fn secret_values_are_redacted_even_inside_collections() {
    let mut rt = Runtime::new(compile("variable token = secret(\"TEST_TOKEN\")\nprint(token)\nprint([token])\nprint({\"key\":token})\nprint(text.string(token))\nprint(\"private-value\")").unwrap(),".");
    rt.environment
        .insert("TEST_TOKEN".into(), "private-value".into());
    rt.run().unwrap();
    assert!(rt.output.iter().all(|v| !v.contains("private-value")));
    assert!(rt.output.iter().all(|v| v.contains("REDACTED")));
    assert_eq!(
        compile("permission net\ndiscord.start(\"hard-coded-token\")")
            .unwrap_err()
            .code,
        "CB103"
    );
}
#[test]
fn file_permissions_and_traversal_are_enforced() {
    let tmp = Temp::new();
    assert_eq!(compile("file.read(\"x\")").unwrap_err().code, "CB301");
    let program =
        compile("permission file\nfile.write(\"a.txt\",\"ok\")\nprint(file.read(\"a.txt\"))")
            .unwrap();
    let mut rt = Runtime::new(program.clone(), &tmp.0);
    assert_eq!(rt.run().unwrap_err().code, "CB301");
    let mut rt = Runtime::new(program, &tmp.0);
    rt.allow.insert("file".into());
    rt.run().unwrap();
    assert_eq!(rt.output, ["ok"]);
    let mut rt = Runtime::new(
        compile("permission file\nfile.read(\"../outside\")").unwrap(),
        &tmp.0,
    );
    rt.allow.insert("file".into());
    assert_eq!(rt.run().unwrap_err().code, "CB301");
}
#[test]
fn formatting_preserves_comments_strings_and_behavior() {
    let source = "# comment\nvariable x = 1\nif true {\nprint(\"{ # not syntax }\") # keep me\n}\n";
    let formatted = format_source(source).unwrap();
    assert!(formatted.contains("    print"));
    assert!(formatted.contains("# keep me"));
    assert_eq!(format_source(&formatted).unwrap(), formatted);
    assert_eq!(run(source).unwrap(), run(&formatted).unwrap());
}
#[test]
fn minecraft_structure_and_publication() {
    let files = convert(command_blocks::cli::DATAPACK).unwrap();
    assert!(files["pack.mcmeta"].contains("\"pack_format\":48"));
    assert!(files.contains_key("data/testing/function/load.mcfunction"));
    assert!(files.contains_key("data/testing/function/tick.mcfunction"));
    assert!(files["data/minecraft/tags/function/load.json"].contains("testing:load"));
    assert!(!files.keys().any(|p| p.contains("/functions/")));
    let tmp = Temp::new();
    let output = tmp.0.join("pack");
    publish(&files, &output).unwrap();
    assert_eq!(
        fs::read_to_string(output.join("pack.mcmeta")).unwrap(),
        files["pack.mcmeta"]
    );
    assert_eq!(publish(&files, &output).unwrap_err().code, "CB205");
    assert!(!files["cb-report.json"].contains("Sistema iniciado"));
}
#[test]
fn minecraft_rejects_incompatible_and_invalid_constructs() {
    for source in [
        "permission file\nprint(file.read(\"x\"))",
        "variable a = []",
        "while true { }",
        "function f(x) { print(x) }",
        "function f() { f() }\nf()",
        "repeat 1001 { print(1) }",
        "print(1 / 2)",
        "variable x = 1.5",
        "minecraft.give_item(\"@a\",\"minecraft:invented_item\",1)",
        "minecraft.message(\"@a\\nsay bad\",\"x\")",
        "event unknown { print(1) }",
        "test \"a\" { assert(true) }",
    ] {
        assert_eq!(convert(source).unwrap_err().code, "CB201", "{source}");
    }
    assert_eq!(
        Backend::new(compile("").unwrap(), "testing", "1.21.9")
            .err()
            .unwrap()
            .code,
        "CB203"
    );
    assert_eq!(
        Backend::new(compile("").unwrap(), "../bad", "1.21.1")
            .err()
            .unwrap()
            .code,
        "CB204"
    );
    assert_eq!(
        check_external(&compile("minecraft.time(0)").unwrap())
            .unwrap_err()
            .code,
        "CB201"
    );
    assert_eq!(
        check_external(&compile("event tick { print(1) }").unwrap())
            .unwrap_err()
            .code,
        "CB201"
    );
}

// A deliberately small command executor validates the generated scoreboard/control-flow subset.
// It does not replace in-game validation of Minecraft itself.
struct Machine {
    files: BTreeMap<String, String>,
    scores: BTreeMap<String, i32>,
    steps: usize,
    messages: usize,
}
impl Machine {
    fn function(&mut self, name: &str) {
        let (ns, name) = name.split_once(':').unwrap();
        let body = self.files[&format!("data/{ns}/function/{name}.mcfunction")].clone();
        for line in body.lines().filter(|l| !l.is_empty()) {
            self.command(line);
        }
    }
    fn command(&mut self, line: &str) {
        self.steps += 1;
        assert!(self.steps < 10000);
        let w: Vec<_> = line.split_whitespace().collect();
        match w[0] {
            "function" => self.function(w[1]),
            "tellraw" => self.messages += 1,
            "scoreboard" if w[1] == "objectives" => {}
            "scoreboard" if w[2] == "set" => {
                self.scores.insert(w[3].into(), w[5].parse().unwrap());
            }
            "scoreboard" if w[2] == "operation" => {
                let a = *self.scores.get(w[3]).unwrap_or(&0);
                let b = self.scores[w[6]];
                let v = match w[5] {
                    "=" => b,
                    "+=" => a.wrapping_add(b),
                    "-=" => a.wrapping_sub(b),
                    "*=" => a.wrapping_mul(b),
                    _ => panic!("{line}"),
                };
                self.scores.insert(w[3].into(), v);
            }
            "execute" => {
                let mut i = 1;
                let mut yes = true;
                while w[i] != "run" {
                    let invert = w[i] == "unless";
                    assert_eq!(w[i + 1], "score");
                    let a = self.scores[w[i + 2]];
                    let matches = if w[i + 4] == "matches" {
                        let v = a == w[i + 5].parse::<i32>().unwrap();
                        i += 6;
                        v
                    } else {
                        let b = self.scores[w[i + 5]];
                        let v = match w[i + 4] {
                            "=" => a == b,
                            "<" => a < b,
                            ">" => a > b,
                            "<=" => a <= b,
                            ">=" => a >= b,
                            _ => panic!("{line}"),
                        };
                        i += 7;
                        v
                    };
                    yes &= matches != invert;
                }
                if yes {
                    self.command(&w[i + 1..].join(" "));
                }
            }
            _ => panic!("Unexpected emitted command: {line}"),
        }
    }
}
#[test]
fn minecraft_scoreboard_behavior_matches_interpreter() {
    for source in [
        command_blocks::cli::DATAPACK,
        "variable x = 0\nvariable flag = true\nif flag { flag = false; x = 5 } else { x = 99 }\nprint(x)",
        "variable x = 2147483647\nx = x + 1\nif x < 0 { x = 7 }\nprint(x)",
        "variable x = 1\nrepeat 5 { x = x * 2 }\nif x == 32 and not false { x = x - 2 }\nprint(x)",
    ] {
        let mut machine = Machine {
            files: convert(source).unwrap(),
            scores: BTreeMap::new(),
            steps: 0,
            messages: 0,
        };
        machine.function("testing:load");
        let output = run(source).unwrap();
        let last = output
            .last()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<i32>()
            .unwrap();
        assert_eq!(machine.scores["#v0"], last, "{source}");
        assert_eq!(machine.messages, output.len());
    }
}
