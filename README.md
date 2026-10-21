# Command Blocks — `cb`

Linguagem bilíngue para scripts e datapacks, implementada em **Rust**. Núcleo sem dependências de terceiros e executável nativo, sem Python. Versão inicial **0.1.0**.

[Documentação em português](docs/pt-br/README.md) · [English documentation](docs/en/README.md)

## Comece pelo que você quer criar

| Objetivo | Guia | Disponibilidade |
|---|---|---|
| Aprender com um programa pequeno | [Primeiros passos](docs/pt-br/primeiros-passos.md) | Implementado; base com 20 testes passando |
| Criar um aplicativo de terminal | [Aplicativos e automações](docs/pt-br/aplicativos.md) | Scripts e empacotamento nativo |
| Criar janelas no Windows | [Janelas nativas](docs/pt-br/janelas.md) | Implementação em validação |
| Criar sistemas dentro do Minecraft | [Datapacks e command blocks](docs/pt-br/minecraft.md) | Subconjunto Java 1.21/1.21.1 |
| Conectar um aplicativo ao Minecraft | [Integrações](docs/pt-br/integracoes.md) | Cliente local; precisa de servidor/plugin |
| Criar um bot Discord | [Estado do Discord](docs/pt-br/integracoes.md#discord) | Interface inicial; bot ainda não funcional |
| Automatizar teclas e hotbar | [Proposta de expansão](docs/pt-br/expansao.md) | Proposta; não implementado |

**Para usar a linguagem:** escreva apenas `.cb` e execute com `cb`. Rust/Cargo são ferramentas de desenvolvimento do próprio compilador; não são requisitos para quem recebe o executável pronto. Ainda não há uma release publicada neste repositório.

**Estado:** versão de desenvolvimento. Não foi validada em um servidor Minecraft real. Consulte [o que falta antes de publicar](CONTRIBUTING.md).

[Publicar os commits no GitHub](docs/PUBLISH.md) · [Licença MIT](LICENSE)

[Resultados efetivamente verificados](docs/VERIFICATION.md) · [Critérios da expansão em andamento](docs/ACCEPTANCE.md)

## Compilar a partir do código-fonte

```powershell
cargo build --release
.\target\release\cb.exe run examples\ola.cb
.\target\release\cb.exe convert examples\pontos.cb --target minecraft --output meu_datapack
```

Para instalar o comando no PATH do Cargo:

```text
cargo install --path . --offline
cb help
```

Requer Rust com linker disponível para compilar. No Windows MSVC, instale o componente C++ e Windows SDK do Visual Studio Build Tools. A alternativa Rust GNU/LLVM está descrita na documentação. O executável gerado não exige Rust instalado.

```text
variavel pontos = 0

funcao ganhar_ponto() {
    pontos = pontos + 1
    imprimir("Pontuação: " + pontos)
}

evento inicio {
    repetir 5 vezes {
        ganhar_ponto()
    }
}
```

## Estado real da versão

- Implementados: lexer, parser, IR tipada, análise semântica, interpretador, CLI, texto, matemática, listas, mapas, I/O, eventos, testes, arquivos com permissões e empacotamento nativo.
- Minecraft: conversão conservadora para **Java 1.21 e 1.21.1**. Inteiros, booleanos, constantes textuais, funções sem parâmetros, condições, repetições limitadas, eventos start/tick e cinco operações Minecraft. Geração validada antes de gravar arquivos.
- Ponte: cliente HTTP local autenticado, com protocolo documentado. Precisa de servidor/plugin externo.
- Discord: **apenas estrutura inicial e validação de credenciais/tipos**. Transporte não implementado; execução retorna CB202. Não é anunciado como bot funcional.
- Não implementados: concorrência, módulos locais, JSON de entrada, HTTP genérico, execução de processos, GUI, Bedrock, plugins/mods e APIs avançadas. Consulte a matriz completa na documentação.

## Verificação

```text
cargo test --offline
cargo clippy --offline --all-targets -- -D warnings
cargo fmt --all -- --check
cb test examples/tests.cb
```

Os testes incluem um executor limitado dos comandos emitidos para verificar scoreboards e fluxo de controle. **Ele não substitui validação dentro de um servidor Minecraft real.**

## Projeto

```text
src/
  lexer.rs        tokens e posições
  parser.rs       sintaxe e árvore
  semantic.rs     análise e IR tipada
  catalog.rs      aliases, assinaturas e ambientes
  runtime.rs      interpretador e bibliotecas
  value.rs        valores e representação segura
  minecraft.rs    backend e publicação de datapacks
  integrations.rs ponte local e interface Discord
  diagnostic.rs   erros bilíngues
  cli.rs          comandos, testes, formato e empacotamento
tests/            testes Rust e integração da CLI
examples/         programas completos
docs/pt-br/       documentação brasileira
docs/en/          documentação inglesa
```
