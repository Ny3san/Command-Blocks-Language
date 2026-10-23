# Expressões

## Operadores

| Categoria | Operadores |
|---|---|
| Aritméticos | `+`, `-`, `*`, `/`, `%` |
| Unários | `+`, `-`, `not` |
| Comparação | `==`, `!=`, `<`, `>`, `<=`, `>=` |
| Lógicos | `and`, `or`, `not` |

## Precedência

Do menor para o maior:

| Nível | Operadores | Associatividade |
|---|---|---|
| 1 | `or` | esquerda |
| 2 | `and` | esquerda |
| 3 | `==`, `!=` | esquerda |
| 4 | `<`, `>`, `<=`, `>=` | esquerda |
| 5 | `+`, `-` | esquerda |
| 6 | `*`, `/`, `%` | esquerda |
| 7 | unários `+`, `-`, `not` | — |
| 8 | acesso a índice `x[i]` e chamadas `f(x)` | — |

Use parênteses para deixar a intenção explícita. Por exemplo, `1 < 2 == true` é avaliado como `(1 < 2) == true`.

## Aritmética

- `+` soma números. Com textos, concatena. Com um texto e um valor simples, converte o valor para texto.
- `-`, `*` seguem as regras de `Int` e `Number`.
- `/` sempre produz um `Number`: `7 / 2` resulta em `3.5`.
- `%` é o resto da divisão e tem o sinal do dividendo: `7 % -3` é `1`; `-7 % 3` é `-1`.
- Dividir por zero gera `CB108`.

## Lógica e curto-circuito

`and` e `or` avaliam o lado direito apenas quando necessário:

```text
print(false and (1 / 0 > 1))   # false: the right side is not evaluated
```

Os operandos devem ser `Bool`. Usar um número em `and` gera `CB103`.

## Comparações

`==` e `!=` aceitam quaisquer tipos e nunca geram erro. `<`, `>`, `<=` e `>=` exigem tipos compatíveis.

## Índices

`x[i]` lê um item de lista ou um valor de mapa. Índices de lista começam em zero e não aceitam negativos. Índice ou chave inexistente gera `CB107`.

Para textos, `"abc"[1]` resulta em `"b"`.
