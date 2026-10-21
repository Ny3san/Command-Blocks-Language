# Milestone 1 — contrato de ajuda da CLI

Verificação: 21 de outubro de 2026. Escopo: primeiro milestone após auditoria, sem reescrever a linguagem nem expandir integrações.

## Problema atual

A ajuda global não tinha descrições bilíngues; a ajuda por comando não existia. Os atalhos de ajuda e versão não compartilhavam a validação normal de argumentos. A ajuda do Discord descrevia incorretamente a ausência de qualquer transporte.

## Por que precisa existir

Iniciantes precisam descobrir os comandos existentes sem executar código. Ferramentas precisam distinguir sucesso, erro e saída de ajuda.

## O que mudará para o usuário

`cb help`, `cb --help`, `cb help run`, `cb run --help` e `cb --version --lang en` têm comportamento consistente. Há descrições PT/EN, exemplos reais e versão obtida do pacote. Erros vão para stderr, ajuda para stdout. Tópicos desconhecidos retornam CB000 e código 1.

## Como será implementado

O parser existente trata help/version como switches, preserva validação estrita e despacha ajuda antes de executar fonte embutido. Um catálogo privado de comandos fornece sintaxe, descrições e exemplos. Não há nova dependência. Como pré-requisito da validação Windows GNU/LLVM, a ligação WinHTTP usa `raw-dylib`, gerando a biblioteca de importação no compilador. A DLL WinHTTP do Windows continua necessária; não é uma dependência de outra linguagem.

Referência: [Rust Reference — external blocks](https://doc.rust-lang.org/reference/items/external-blocks.html#dylib-versus-raw-dylib).

## Diferencial em relação a Lua/Luau, Python, Rust, JavaScript/TypeScript e command blocks

Este milestone não introduz um novo modelo de linguagem. Torna acessível, em português e inglês, o contrato dos modos local e Minecraft da mesma CLI. Não promete superioridade de desempenho nem compatibilidade além do backend atual.

## Riscos e alternativas

Mantém o parser manual para evitar dependências e preservar o build offline. Um parser de terceiros pode ser reavaliado quando houver mais comandos. Tópicos desconhecidos deixam de mostrar orientação genérica e passam a falhar; programas `.cb` não precisam migrar. `raw-dylib` foi verificado no alvo Windows GNU/LLVM local, não em uma matriz de plataformas.

## Testes de aceitação

- `cargo fmt --all -- --check`: passou.
- `cargo check --offline --all-targets`: passou antes da última ampliação dos testes; a suíte nativa compilou e executou as alterações finais.
- `./scripts/dev.ps1 test --offline`: passou, 31 testes (7 unitários, 8 de CLI/integrações, 16 de linguagem), zero falhas. Toolchain local Windows GNU/LLVM com CRT estático. Avisos de hardlink no cache resultaram em cópia de arquivos.
- Fixtures `tests/golden/help-run.{en,pt-br}.txt`: saída completa conferida. Os 11 comandos têm ajuda testada nos dois idiomas.
- Executável empacotado: `--help` não executa o fonte embutido.
- `git diff --check`: passou; avisos de conversão LF/CRLF do Git, sem erros de whitespace.
- Execução manual de `cb help run --lang en`: saída esperada e código 0.
- `cargo clippy --offline --all-targets -- -D warnings`: falhou na base, principalmente `result_large_err`. Não houve supressão de lint. O gate de qualidade global permanece pendente.

Os testes de Discord usam transporte simulado. HTTP e ponte usam loopback; GUI usa controles Win32 locais. Nenhum resultado acima comprova Discord ao vivo, servidor Minecraft real, Linux/macOS ou release MSVC.

## Estado antes/depois

Antes: ajuda parcial/inconsistente. Depois: contrato de ajuda/versão **implemented**, com testes e guias PT/EN. Discord continua **experimental** no REST Windows; Gateway/eventos não estão implementados. O projeto completo ainda não está pronto para uma release de produção.

Arquivos deste milestone: `src/cli.rs`, `tests/cli_and_integrations.rs`, `tests/golden/`, `docs/pt-br/cli.md`, `docs/en/cli.md`, links nos respectivos índices e este relatório. Pré-requisito: atributo de ligação em `src/net.rs`. As expansões JSON/HTTP/Win32/REST já estavam no checkout antes do milestone; não são apresentadas como novas entregas desta fase.

Próximo milestone sugerido: corrigir os gates Clippy sem silenciar avisos, consolidar a matriz CI e os diagnósticos. `doctor`, cores, saída JSON, módulos, LSP, Gateway Discord e macros seguem fora desta entrega. Nenhum push foi realizado.
