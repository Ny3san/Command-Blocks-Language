use crate::value::Type;
#[derive(Debug, Clone)]
pub struct Spec {
    pub args: Vec<Type>,
    pub result: Type,
    pub environment: &'static str,
    pub permission: Option<&'static str>,
}
pub fn module(name: &str) -> &str {
    match name {
        "texto" => "text",
        "matematica" => "math",
        "lista" => "list",
        "mapa" => "map",
        "entrada_saida" => "io",
        "arquivo" => "file",
        "processo" => "process",
        "rede" => "net",
        "evento" => "event",
        "teste" => "test",
        "tempo" => "time",
        "data" => "date",
        "ponte_minecraft" => "minecraft_bridge",
        _ => name,
    }
}
pub fn canonical(name: &str) -> String {
    if let Some((m, f)) = name.split_once('.') {
        let m = module(m);
        let f = match f {
            "imprimir" => "print",
            "maiusculas" => "upper",
            "minusculas" => "lower",
            "aparar" => "trim",
            "juntar" => "join",
            "separar" => "split",
            "substituir" => "replace",
            "contem" => "contains",
            "comeca_com" => "starts_with",
            "termina_com" => "ends_with",
            "tamanho" => "length",
            "converter" => "string",
            "numero" => "number",
            "formatar" => "format",
            "absoluto" => "abs",
            "arredondar" => "round",
            "raiz" => "sqrt",
            "seno" => "sin",
            "cosseno" => "cos",
            "tangente" => "tan",
            "graus" => "degrees",
            "radianos" => "radians",
            "minimo" => "min",
            "maximo" => "max",
            "potencia" => "pow",
            "aleatorio" => "random",
            "criar" => "create",
            "adicionar" => {
                if m == "file" {
                    "append"
                } else {
                    "add"
                }
            }
            "remover" => "remove",
            "obter" => "get",
            "ordenar" => "sort",
            "definir" => "set",
            "chaves" => "keys",
            "ler_texto" => "read_text",
            "ler_numero" => "read_number",
            "ambiente" => "environment",
            "emitir" => "emit",
            "esperar" => "sleep",
            "agora" => "now",
            "ler" => "read",
            "escrever" => "write",
            "existe" => "exists",
            "criar_pasta" => "mkdir",
            "listar" => "list",
            "executar" => "run",
            "iniciar" => "start",
            "responder" => "reply",
            "enviar" => "send",
            "mensagem" => "message",
            "dar_item" => "give_item",
            "teleportar" => "teleport",
            "clima" => "weather",
            "horario" => "time",
            _ => f,
        };
        format!("{m}.{f}")
    } else {
        match name {
            "imprimir" => "print",
            "verificar" => "assert",
            "segredo" => "secret",
            _ => module(name),
        }
        .into()
    }
}
pub fn event(name: &str) -> &str {
    match name {
        "inicio" => "start",
        "atualizar" => "tick",
        "mensagem" => "message",
        _ => name,
    }
}
pub fn spec(name: &str) -> Option<Spec> {
    use Type::*;
    let (args, result) = match name {
        "print" | "text.print" | "io.print" => (vec![Any], Null),
        "assert" => (vec![Bool], Null),
        "secret" => (vec![Text], Secret),
        "text.upper" | "text.lower" | "text.trim" => (vec![Text], Text),
        "text.join" => (vec![List, Text], Text),
        "text.split" => (vec![Text, Text], List),
        "text.replace" => (vec![Text, Text, Text], Text),
        "text.contains" | "text.starts_with" | "text.ends_with" => (vec![Text, Text], Bool),
        "text.length" => (vec![Text], Int),
        "text.string" => (vec![Any], Text),
        "text.number" => (vec![Text], Number),
        "text.format" => (vec![Text, Map], Text),
        "math.abs" | "math.round" | "math.sqrt" | "math.sin" | "math.cos" | "math.tan"
        | "math.degrees" | "math.radians" => (vec![Number], Number),
        "math.min" | "math.max" | "math.pow" => (vec![Number, Number], Number),
        "math.random" => (vec![Int, Int], Int),
        "list.create" => (vec![], List),
        "list.add" | "list.remove" => (vec![List, Any], Null),
        "list.get" => (vec![List, Int], Any),
        "list.contains" => (vec![List, Any], Bool),
        "list.sort" => (vec![List], List),
        "list.length" => (vec![List], Int),
        "map.create" => (vec![], Map),
        "map.set" => (vec![Map, Text, Any], Null),
        "map.get" => (vec![Map, Text], Any),
        "map.contains" => (vec![Map, Text], Bool),
        "map.keys" => (vec![Map], List),
        "map.length" => (vec![Map], Int),
        "io.read_text" => (vec![Text], Text),
        "io.read_number" => (vec![Text], Number),
        "io.environment" => (vec![], Text),
        "event.emit" => (vec![Text], Null),
        "time.sleep" => (vec![Number], Null),
        "time.now" => (vec![], Number),
        "file.read" => (vec![Text], Text),
        "file.write" | "file.append" => (vec![Text, Text], Null),
        "file.exists" => (vec![Text], Bool),
        "file.mkdir" => (vec![Text], Null),
        "file.list" => (vec![Text], List),
        "process.run" => (vec![Text, List], Map),
        "discord.start" => (vec![Secret], Null),
        "discord.reply" => (vec![Text], Null),
        "minecraft_bridge.send" => (vec![Text], Null),
        "minecraft.message" => (vec![Text, Text], Null),
        "minecraft.give_item" => (vec![Text, Text, Int], Null),
        "minecraft.teleport" => (vec![Text, Number, Number, Number], Null),
        "minecraft.weather" => (vec![Text], Null),
        "minecraft.time" => (vec![Int], Null),
        _ => return None,
    };
    let m = name.split('.').next().unwrap_or("");
    let environment = if m == "minecraft" {
        "minecraft"
    } else if matches!(
        m,
        "file" | "process" | "discord" | "minecraft_bridge" | "time"
    ) || name == "secret"
        || name.starts_with("io.read_")
    {
        "external"
    } else {
        "common"
    };
    let permission = match m {
        "file" => Some("file"),
        "process" => Some("process"),
        "discord" => Some("net"),
        "minecraft_bridge" => Some("minecraft_bridge"),
        _ => None,
    };
    Some(Spec {
        args,
        result,
        environment,
        permission,
    })
}
pub const MODULES: &[&str] = &[
    "text",
    "math",
    "list",
    "map",
    "io",
    "event",
    "time",
    "file",
    "process",
    "discord",
    "minecraft_bridge",
    "minecraft",
];
