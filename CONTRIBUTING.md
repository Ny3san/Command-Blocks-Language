# Contribuir / Contributing

O projeto é uma base em desenvolvimento, não uma release estável. Não publique credenciais, `.env`, `.cb-libraries`, diretórios `target` ou programas empacotados contendo dados privados.

The project is a development foundation, not a stable release. Do not commit credentials, `.env`, `.cb-libraries`, build caches or packaged programs containing private data.

## Antes de publicar no GitHub / Before publishing

- Executar todos os testes Rust e da CLI. / Run Rust and CLI tests.
- Validar exemplos e links da documentação. / Validate examples and documentation links.
- Verificar executáveis independentes da toolchain. / Verify standalone binaries.
- Validar datapack em um mundo de teste e registrar versão/resultado. / Validate datapacks in a test world and record the version/result.
- Manter Discord, GUI e automação de cliente identificados como não implementados enquanto for o caso. / Keep unsupported features labelled accurately.
- Preservar o arquivo LICENSE e créditos dos contribuidores; a licença do projeto é MIT. / Preserve LICENSE and contributor credits; the project uses the MIT license.

## Verificações / Checks

```text
cargo check --offline --all-targets
cargo test --offline
cargo fmt --all -- --check
cargo clippy --offline --all-targets -- -D warnings
```

Windows MSVC needs the C++ build tools and Windows SDK. See the manual for the alternative GNU/LLVM toolchain. Passing `cargo check` alone is not passing executable tests.

For a feature, include an executable example, environment compatibility, PT-BR/EN diagnostics and tests. For Minecraft, test behavior as well as file generation. Changes should preserve existing aliases and avoid adding unnecessary keywords.

This document does not publish the repository, create a remote or claim that release checks have passed.
