# CLI: help and version contract

Status: **implemented**. Help never executes programs or grants permissions.

```text
cb --help
cb help
cb help run --lang en
cb run --help --lang en
cb help convert --lang en
cb --version --lang en
```

`cb help COMMAND` and `cb COMMAND --help` show the same help. Global help lists commands and descriptions; command help includes syntax, limits and a real repository example. `--lang pt-br|en` also accepts `--lang=en`; Portuguese is the default. The version comes from the compiled package.

Help and results go to stdout; errors go to stderr. Exit code 0 means success; 1 means argument, analysis or execution failure. `CB000` identifies invalid arguments. Unknown topics, duplicate options (except `--allow`) and combinations such as `--help --version` are rejected. Help accepts only the command/topic and language: use `cb run --help`, without a file or execution options.

Permissions still require a source declaration and an explicit `--allow` grant. Reading help never installs libraries. Discord REST is experimental on Windows; `discord.start`/`discord.iniciar` and Gateway events remain unavailable.

Migration: no `.cb` program changes. `cb help UNKNOWN_TOPIC`, which previously displayed generic guidance, now returns an error. No command was removed.

Planned, unavailable: `doctor`, `init`, `fmt`, `package`, JSON output and color controls. Use existing `create`, `format` and `build`. Source excerpts and multiple diagnostics belong to another milestone.

Tests: PT/EN fixtures in `tests/golden`, equivalent help forms, all commands in both languages, invalid options and stdout/stderr separation in `tests/cli_and_integrations.rs`.
