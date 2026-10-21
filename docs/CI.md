# Continuous integration / Integração contínua

Estado / Status: **experimental**, até a primeira execução no GitHub / until the first GitHub run.

`.github/workflows/ci.yml` configura três alvos / configures three targets:

| Runner | Toolchain |
| --- | --- |
| Ubuntu | stable-x86_64-unknown-linux-gnu |
| Windows | stable-x86_64-pc-windows-msvc |
| Windows | stable-x86_64-pc-windows-gnullvm |

Cada alvo executa formatação, testes e Clippy com avisos tratados como erros. O workflow não publica artefatos, não altera o repositório e não usa tokens de Discord ou Minecraft. A instalação da toolchain e o checkout usam rede; os comandos Cargo usam `--offline`. Testes de transporte usam loopback ou simulação, sem serviços externos.

Each target runs formatting, tests and Clippy with warnings treated as errors. The workflow does not publish artifacts, modify the repository or use Discord/Minecraft tokens. Toolchain installation and checkout require network access; Cargo commands use `--offline`. Transport tests use loopback or mocks, not external services.

GNU/LLVM usa o componente `rust-mingw` e o linker `rust-lld` da própria toolchain, conforme a [documentação oficial Rust](https://doc.rust-lang.org/rustc/platform-support/windows-gnullvm.html). Checkout usa [actions/checkout v6.0.2](https://github.com/actions/checkout/releases/tag/v6.0.2), sem persistência de credenciais.

GNU/LLVM uses the toolchain's `rust-mingw` component and `rust-lld` linker, following the official Rust documentation linked above. Checkout uses the linked v6.0.2 release without persisting credentials.

Limites / Limits: Windows GNU/LLVM foi testado localmente / was tested locally. Isso não prova aprovação dos runners Linux/MSVC ou execução hospedada / this does not prove Linux/MSVC or hosted CI success. Stable e as imagens de runner evoluem; ainda não é um build de release reproduzível / Stable and runner images change; this is not a reproducible release build yet. Sem remoto/push, o resultado hospedado permanece pendente / without a remote/push, hosted results remain pending.
