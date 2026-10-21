# Verified baseline / Base verificada

This record applies to the initial Rust core, not to proposed capabilities.

## Executed successfully

- Full Rust suite: **20 tests passed** (7 CLI/integration tests and 13 language/backend tests).
- Language examples: greeting and the three `.cb` tests completed successfully.
- Native Windows build using Rust GNU/LLVM with static compiler runtime linking.
- Copied `cb.exe` alone into a new temporary directory; restricted PATH to Windows System32; executed a `.cb` program successfully.
- From that isolated copy, packaged an application and ran it successfully under the same restricted PATH.
- Executable scan no longer lists the toolchain's `libunwind.dll`; remaining DLL names are Windows system components.
- Markdown local file links resolved; Rust formatting check passed.

The full test command on this development machine is `scripts/dev.ps1 test --offline`. It uses the isolated toolchain if present. Other environments can use `cargo test --offline` with a working native linker.

## Not verified by those results

- A live Minecraft server or player interaction.
- A live Discord bot; its current interface is a scaffold.
- Graphical application functionality or hotbar automation.
- Linux/macOS runtime behavior.
- Absence of every possible bug or complete resistance to hostile programs.

Test doubles are used for the local HTTP bridge and a subset of generated Minecraft commands. They are not presented as live-service validation.
