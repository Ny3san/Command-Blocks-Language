# Linha de comando

```text
cb COMMAND [OPTIONS]
```

## Comandos

| Comando | Descrição |
|---|---|
| `run <file>` | Verifica e executa um programa |
| `check <file>` | Verifica um programa sem executá-lo |
| `test <path>` | Executa os blocos `test` de um arquivo ou pasta |
| `convert <file>` | Gera um datapack do Minecraft |
| `build <file>` | Gera um executável com o interpretador e o código-fonte |
| `format <file>` | Ajusta a indentação do código |
| `create <kind> <name>` | Cria um projeto: `app` ou `minecraft-datapack` |
| `help [command]` | Mostra a ajuda |

## Opções

| Opção | Descrição |
|---|---|
| `--allow <name>` | Autoriza um recurso declarado com `permission`. Atualmente: `file`. |
| `--max-steps <n>` | Limite de passos de execução. Padrão: 100000. |
| `--lang <pt-br\|en>` | Idioma das mensagens. Padrão: `pt-br`. |
| `--target minecraft` | Destino de `check` e `convert`. |
| `--mc-version <v>` | Versão do Minecraft: `1.21` ou `1.21.1`. |
| `--namespace <name>` | Namespace do datapack gerado. |
| `--output <path>` | Caminho do resultado de `convert`, `build` e `create`. |
| `--help`, `--version` | Ajuda e versão. |

## Exemplos

```text
cb run examples/hello.cb
cb check examples/minecraft.cb --target minecraft
cb test examples/tests.cb
cb convert examples/pontos.cb --target minecraft --mc-version 1.21.1 --namespace pontos --output datapack
cb create app meu_app
cb help convert --lang en
```

## Comportamento

| Comando | Detalhes |
|---|---|
| `run` | Verifica o programa inteiro antes de executar. Um erro impede a execução. |
| `check` | Não executa o programa. |
| `test` | Não concede permissões externas. Sai com código `1` se algum teste falhar. |
| `convert` | Gera todos os arquivos em memória antes de gravar. Não substitui um destino existente. |
| `build` | Empacota o interpretador e o código-fonte. Não compila para código de máquina. |
| `format` | Quando altera o arquivo, cria `arquivo.cb.bak` antes. Recusa se o backup já existir. |

## Códigos de saída

| Código | Significado |
|---|---|
| `0` | Sucesso |
| `1` | Erro de argumentos, de análise ou de execução |

A ajuda vai para a saída padrão; erros vão para a saída de erro.
