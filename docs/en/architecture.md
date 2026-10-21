# Architecture and extension

[Manual](README.md) · [Grammar](../grammar.ebnf) · [Contributing](../../CONTRIBUTING.md)

Source → lexer → parser → semantic analysis → typed tree IR → external interpreter or Minecraft backend.

Rust builds the executable; users write only Command Blocks. The IR reuses the syntax tree with canonical builtin names, inferred types and original source spans. It is not bytecode or a stable serialization format.

The catalog owns aliases, function signatures, environments and permissions. Runtime/value modules own execution and safe representations. The Minecraft module validates its subset and prepares files before publication. Integration modules handle optional external protocols. The CLI owns commands, scaffolding, tests, formatting and native packaging.

To add a library function, define both aliases, argument/return types, supported environments and permissions. Add runtime or adapter behavior, tests for errors/denied permissions, an executable example and both documentation languages. Libraries are compiled into the project; dynamic plugins and source-file modules are not implemented.

A new backend consumes a validated Program, rejects unsupported semantics (including unreachable unsupported code), uses explicit version profiles and generates files in memory. Verify behavior in the target environment, not just text output.

Prefer a library function over a new keyword. Keep simple scripts linear and introduce events only when useful. Preserve existing aliases and clearly label implemented, experimental and proposed features.
