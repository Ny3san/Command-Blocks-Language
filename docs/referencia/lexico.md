# Léxico

## Comentários

Um comentário começa com `#` ou `//` e termina no fim da linha:

```text
# comment
// comment
```

## Identificadores

Um identificador é formado por letras Unicode, dígitos e `_`. Não pode começar com dígito.

```text
variable ação = 1
variable player_2 = 2
```

Nomes são sensíveis a maiúsculas e minúsculas.

## Palavras reservadas

As palavras abaixo têm significado especial na linguagem. Versões em português existem para algumas delas (veja a [lista completa em Instruções](instrucoes.md#palavras-reservadas-e-aliases)).

`variable`, `constant`, `function`, `return`, `if`, `else`, `while`, `repeat`, `times`, `for`, `each`, `in`, `event`, `test`, `import`, `permission`, `true`, `false`, `null`, `and`, `or`, `not`.

## Literais

| Literal | Exemplos | Observação |
|---|---|---|
| Inteiro | `0`, `42`, `-12` | Sinal é operador unário |
| Número | `1.5`, `0.25` | Ponto decimal obrigatório com parte fracionária |
| Texto | `"abc"`, `'abc'` | Aspas simples ou duplas |
| Booleano | `true`, `false` | |
| Nulo | `null` | |
| Lista | `[1, "a", true]` | Elementos podem ser de tipos diferentes |
| Mapa | `{"key": 1}` | Chaves devem ser textos |

### Escapes em textos

| Sequência | Caractere |
|---|---|
| `\n` | quebra de linha |
| `\r` | retorno de carro |
| `\t` | tabulação |
| `\\` | barra invertida |
| `\'` | aspas simples |
| `\"` | aspas duplas |

Outras sequências com `\` são erro `CB001`.

## Separadores

Instruções são separadas por quebra de linha ou por `;`. Entre duas instruções vizinhas é obrigatório um separador, exceto antes de `}`.

Dentro de listas, mapas e argumentos de função, quebras de linha são permitidas. Vírgula final não é aceita nesta versão.
