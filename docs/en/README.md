# Command Blocks 0.1

[CLI help and exit codes](cli.md)

[Diagnostics and Rust API migration](diagnostics.md)

[Native windows (under validation)](windows.md)

[JSON library](json.md)

[Network and APIs (under validation)](network.md)

[Project overview](../../README.md) · [Minecraft](minecraft.md) · [Integrations](integrations.md) · [Architecture](architecture.md) · [Proposed expansion](roadmap.md)

## Start with one line

Create `hello.cb`:

```text
print("Hello, world!")
```

Run `cb run hello.cb --lang en`. No event, import or function is needed for a first program. Users write only Command Blocks. A prebuilt executable does not require Python, Rust or Cargo. There is no published release yet; developers can build from source with Rust and a working native linker.

```text
cargo build --release --offline
cargo install --path . --offline
cb help
```

On Windows MSVC, install the Visual Studio C++ tools and Windows SDK. For pure Rust projects, the [official GNU/LLVM alternative](https://doc.rust-lang.org/rustc/platform-support/windows-gnullvm.html) uses the Rust MinGW component and rust-lld. Verify standalone dependencies before distributing any build.

## Language basics

```text
variable points = 0
constant limit = 5

function add(a, b) {
    return a + b
}

repeat 5 times {
    points = add(points, 1)
}

if points >= limit {
    print("You won!")
} else {
    print("Keep going")
}
```

Statements use newlines or semicolons; braces delimit blocks. Comments start with `#` or `//`. Strings use either quote style with basic escapes. See the [grammar](../grammar.ebnf). The example above runs externally; the initial Minecraft backend does not support function parameters.

Values: signed 32-bit integers, finite 64-bit decimals, Unicode text, booleans, null, heterogeneous lists, text-keyed maps and protected secrets. Integer addition/subtraction/multiplication wrap at 32 bits. Division returns a decimal; division by zero fails. Remainder has the dividend's sign. Random numbers are for games, not credentials.

Variable types are inferred. Function parameters/returns and collection lookups may be dynamically checked. Constants prevent rebinding, not deep collection mutation. Lists/maps share identity when assigned; insertion copies the inserted value to prevent cycles. List/map equality compares identity. Indexes start at zero and cannot be negative; text indexes count Unicode characters, not graphemes.

## Two equivalent vocabularies

| English | Brazilian Portuguese |
|---|---|
| variable, constant, function, return | variavel, constante, funcao, retornar |
| if, else, while | se, senao, enquanto |
| repeat, times, for each … in | repetir, vezes, para cada … em |
| event, start, tick, test | evento, inicio, atualizar, teste |
| import, permission | importar, permissao |
| print, assert, secret | imprimir, verificar, segredo |
| true, false, null, and, or, not | verdadeiro, falso, nulo, e, ou, nao |

You may mix aliases. Diagnostics default to Portuguese; `--lang en` selects English. Conditions require booleans; logical operators short-circuit.

## Functions, events and tests

Functions have global lexical scope and local parameters. Blocks introduce scopes. No closures, anonymous functions or source-file modules exist yet. Imports, permissions, functions, events and tests are top-level declarations.

Top-level executable statements run first in source order, followed by `event start` handlers. Calling a function before a global it reads is initialized fails. Custom events are synchronous and have no payload. `event.emit("name")` invokes handlers; an unhandled event does nothing. `event tick` is Minecraft-only.

```text
test "addition" {
    assert(2 + 2 == 4)
}
```

`cb test [path]` discovers `.cb` files and executes each test with fresh globals, including top-level initializers but excluding startup handlers. External permissions are not granted. Failures return exit code 1. No discovered tests is reported explicitly. `cargo test` tests the implementation itself.

## Library reference

| Module (PT alias) | Implemented functions |
|---|---|
| text (texto) | print(value), upper(text), lower(text), trim(text), join(list, separator), split(text, separator), replace(text, from, to), contains(text, part), starts_with(text, part), ends_with(text, part), length(text), string(value), number(text), format(template, map) |
| math (matematica) | abs, round, sqrt, sin, cos, tan, degrees, radians: one number; min(a,b), max(a,b), pow(a,b), random(integer_min, integer_max) inclusive |
| list (lista) | create(), add(list,value), remove(list,value), get(list,index), contains(list,value), sort(list), length(list) |
| map (mapa) | create(), set(map,key,value), get(map,key), contains(map,key), keys(map), length(map) |
| io (entrada_saida) | print(value), read_text(prompt), read_number(prompt), environment() |
| event (evento) | emit(name) |
| time (tempo) | now(): monotonic seconds since runtime start; sleep(seconds): 0–60 |
| file (arquivo) | read(path), write(path,text), append(path,text), exists(path), mkdir(path), list(path) |

Core imports are optional. Portuguese function aliases are listed in the [Portuguese reference](../pt-br/README.md). `sort` returns a new list of only numbers or only text. `remove` removes the first match or fails. `format` replaces map keys without evaluating code. `round` rounds ties away from zero; math functions other than random return decimals.

## Terminal applications

Start with `cb create app my_app --lang en`, then `cb run my_app/main.cb`. The template contains one print statement and a short guide, without unnecessary boilerplate. Omit `--lang en` for the Portuguese template.

```text
variable name = io.read_text("Your name: ")
print("Hello, " + name)
```

Use `cb build hello.cb --output hello.exe` on Windows. This packages the current native interpreter and source for the same platform/architecture. It does not require `cb` on the destination machine, does not perform AOT compilation, and does not hide source. Environment credentials and extra assets are not automatically included. Optional modules still need their local configuration. GUI windows, buttons and global hotkeys are not implemented.

## CLI

```text
cb run FILE [--allow file] [--max-steps 100000] [--lang en]
cb check FILE [--target minecraft]
cb convert FILE --target minecraft --mc-version 1.21.1 --namespace example --output pack
cb build FILE --output APP
cb test [PATH]
cb format FILE
cb create minecraft-datapack NAME
cb create discord-bot NAME
cb install minecraft_bridge
cb install discord
cb list
cb info discord
cb help text
```

`check` never runs the program or contacts services. Minecraft checking generates the plan in memory. `format` preserves comments/strings and adjusts indentation, creating `.bak` before editing; an existing backup blocks another changed write. `install` enables bundled optional interfaces in `.cb-libraries`; it is not a remote package manager. Discord is explicitly a scaffold.

## Permissions and errors

Sensitive calls require both `permission file` (or another capability) and a matching CLI grant. File operations are confined to the source directory; absolute paths, parent traversal and existing symlinks escaping that root are rejected. This is not an OS sandbox and does not defend against another process racing filesystem changes. `file.write` overwrites the requested file.

`secret("NAME")` reads an environment variable and returns a protected value. Secret values are redacted even within collections; environment values with TOKEN/PASSWORD/SECRET/API_KEY in their names are also redacted from output. Diagnostics omit source excerpts and private service data. Arbitrary literal passwords cannot be automatically identified: never embed credentials in source or packages.

Errors contain file, line, column, code, cause and a suggestion. Examples: CB103 type mismatch; CB201 incompatible environment/backend; CB202 unavailable integration; CB301 permission denied; CB302 resource limit; CB303 missing secret; CB304 integration failure; CB401 assertion failure. `--lang en` applies to diagnostics as well.

## Limits and next steps

Source size, expression nesting, recursion, execution steps, individual values and bridge responses have limits; this is not complete process-level CPU/memory isolation. Process execution is reserved but unimplemented. Also absent: input JSON, general HTTP, calendar dates, schedulers, concurrency, file modules, GUI, devices, live Discord transport, a bridge server, client automation, Bedrock and other Minecraft profiles.

Complete runnable examples are under `examples/`. Proposed additions are in the [roadmap](roadmap.md), not advertised as available APIs. See [contribution and release checks](../../CONTRIBUTING.md).
