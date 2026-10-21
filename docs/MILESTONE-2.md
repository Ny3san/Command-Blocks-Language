# Milestone 2 — gate Clippy e preparação de CI

Verificação local: 21 de outubro de 2026. A refatoração está **implemented**; a matriz hospedada está **experimental**.

## Problema atual

O gate Clippy falhava principalmente porque `Error` carregava quatro `String`, tornando numerosos resultados de funções grandes. Oito condições aninhadas também eram apontadas. Não havia workflow de CI.

## Por que precisa existir

Um gate de qualidade precisa passar sem supressões e sem remover verificações de segurança. A matriz deve tornar diferenças entre plataformas visíveis.

## O que mudará para o usuário

Programas `.cb`, aliases, permissões, mensagens e códigos de saída permanecem compatíveis. Consumidores diretos da API Rust devem observar a migração dos campos textuais de `Error`, descrita nos guias de diagnósticos PT/EN.

## Como será implementado

Textos do diagnóstico usam `Box<str>`, pois não precisam reservar capacidade de crescimento. Construtores mantêm as mesmas assinaturas. As condições foram simplificadas com rustfix e revisadas. Um teste verifica a saída completa dos diagnósticos nos dois idiomas. Não foram adicionadas dependências ou supressões de lint.

A matriz CI configura Linux GNU, Windows MSVC e Windows GNU/LLVM, com formatação, testes offline e Clippy. Tem somente permissão de leitura do repositório, não publica artefatos e não usa credenciais de serviços externos. A instalação da toolchain requer rede. Consulte [CI.md](CI.md).

## Diferencial em relação a Lua/Luau, Python, Rust, JavaScript/TypeScript e command blocks

Esta etapa consolida a implementação Rust e o contrato bilíngue da ferramenta. Não adiciona uma nova capacidade à linguagem nem afirma superioridade de desempenho sobre outras linguagens.

## Riscos e alternativas

Manter `String` com supressão de lint foi rejeitado. Alterar todo `Result` para erros encaixotados exigiria mudanças em mais chamadas. `Box<str>` muda o tipo de campos públicos da biblioteca Rust; use os construtores existentes ou converta os campos explicitamente. Nenhuma migração de fonte `.cb` é necessária.

Linux/MSVC e GitHub Actions ainda não foram executados. A tentativa de instalar o componente Linux no ambiente local falhou com erro de acesso à rede 10013. A matriz não deve receber status aprovado por existir um arquivo YAML. Campos e imports exclusivos do Windows ainda precisam da checagem Linux real. A sintaxe PowerShell do bloco de preparação foi verificada, não a execução hospedada completa.

## Testes de aceitação

- `cargo fmt --all -- --check`: passou.
- `cargo clippy --offline --all-targets -- -D warnings`: passou, código 0.
- `./scripts/dev.ps1 test --offline`: passou, código 0; 32 testes (8 unitários, 8 CLI/integrações, 16 linguagem), zero falhas.
- `git diff --check`: passou após a refatoração. Avisos do Git sobre LF/CRLF não eram erros de whitespace.
- Diagnósticos: localização, código, sugestão PT/EN, clonagem e Display preservados.
- Os avisos ambientais de hardlink do cache causaram cópia de arquivos; não foram suprimidos.

Discord segue testado por simulação, HTTP/ponte por loopback e GUI por controles Win32 locais. Não houve teste ao vivo de Discord ou servidor Minecraft.

## Estado antes/depois

Antes: Clippy falhando e sem matriz CI. Depois: gate Clippy e 32 testes aprovados no ambiente local; CI preparada, mas sem aprovação hospedada. Arquivos: `src/diagnostic.rs`, condições em `src/cli.rs`, `src/runtime.rs`, `src/semantic.rs`, `src/minecraft.rs`, guias PT/EN de diagnósticos, índices, `.github/workflows/ci.yml`, `docs/CI.md` e este relatório.

Próximo passo: validar a matriz por plataforma antes de anunciar portabilidade; depois ampliar diagnósticos e demais recursos P0 em outra etapa. Nenhum push ou novo commit foi realizado nesta etapa.
