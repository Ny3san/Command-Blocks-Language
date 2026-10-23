# Instalação

## Requisitos

- [Rust](https://rustup.rs/) estável.
- No Windows com a toolchain MSVC: **Visual Studio Build Tools** com a carga de trabalho C++ e o Windows SDK.

Uma alternativa para Windows é a toolchain `stable-x86_64-pc-windows-gnullvm`, com o componente `rust-mingw` e o linker `rust-lld`. Consulte a [documentação oficial do Rust](https://doc.rust-lang.org/rustc/platform-support/windows-gnullvm.html).

`cargo check` não gera o executável. Use `cargo build` para verificar a instalação.

## Compilar

```text
cargo build --release --offline
```

O executável fica em:

- Linux e macOS: `target/release/cb`
- Windows: `target\release\cb.exe`

Copie o executável para uma pasta do `PATH` para usar `cb` de qualquer lugar.

## Verificar a instalação

```text
cb --version
cb run examples/hello.cb
```

## Idioma das mensagens

As mensagens usam português por padrão. Para inglês, use `--lang en` em qualquer comando.
