# Linha de comando (`cb`)

## Comandos

| Comando | Para que serve |
|---|---|
| `cb run arquivo.cb` | Executa um programa |
| `cb check arquivo.cb` | Verifica erros sem executar |
| `cb test arquivo.cb` | Executa os blocos `test` |
| `cb convert arquivo.cb --target minecraft --output pasta` | Gera um datapack |
| `cb build arquivo.cb --output app.exe` | Gera um executável (Windows) |
| `cb format arquivo.cb` | Ajusta a indentação |
| `cb create app nome` | Cria um projeto de aplicativo |
| `cb create minecraft-datapack nome` | Cria um projeto de datapack |
| `cb create discord-bot nome` | Cria um projeto de bot (ainda não funciona) |
| `cb install minecraft_bridge` | Habilita uma biblioteca opcional nesta pasta |
| `cb list` | Lista as bibliotecas opcionais |
| `cb info nome` | Mostra informações sobre uma biblioteca |
| `cb help [comando]` | Mostra a ajuda |
| `cb --version` | Mostra a versão |

## Opções

| Opção | Efeito |
|---|---|
| `--allow nome` | Autoriza um recurso declarado com `permission` (ex.: `--allow file`) |
| `--max-steps N` | Limite de passos de execução (padrão: 100000) |
| `--lang pt-br` ou `--lang en` | Idioma das mensagens (padrão: português) |
| `--target minecraft` | Destino para `check` ou `convert` |
| `--mc-version 1.21.1` | Versão do Minecraft (1.21 ou 1.21.1) |
| `--namespace nome` | Nome do datapack |
| `--output caminho` | Onde salvar o resultado |

## Exemplos

```text
cb run examples/hello.cb
cb run examples/window.cb --allow window
cb check examples/minecraft.cb --target minecraft
cb test examples/tests.cb
cb help convert --lang en
```

## Códigos de saída

- `0`: sucesso.
- `1`: erro de argumentos, de análise ou de execução.

As mensagens de ajuda vão para a saída padrão. Os erros vão para a saída de erro.

## Formatação

`cb format` ajusta a indentação e preserva comentários e textos. Antes de alterar, cria um arquivo `.bak`. Se esse backup já existir, o comando recusa para não perder o anterior.

## Bibliotecas opcionais

`cb install` apenas habilita módulos que já existem no projeto. Ele **não** baixa pacotes de outros lugares. O arquivo `.cb-libraries` fica ao lado do código.
