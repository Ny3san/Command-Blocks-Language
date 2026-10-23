# 3. Decisões e repetições

## `if` e `else`

```text
variable points = 3

if points >= 5 {
    print("You won!")
} else {
    print("Keep going")
}
```

A condição precisa ser um booleano. Não existe conversão implícita de números para verdadeiro ou falso.

## `while`

Repete enquanto a condição for verdadeira:

```text
variable n = 0
while n < 3 {
    n = n + 1
}
print(n)                 # 3
```

## `repeat`

Repete um número fixo de vezes:

```text
repeat 3 times {
    print("tick")
}
```

A quantidade pode ser uma expressão: `repeat 2 + 1 times { ... }`.

## Blocos e escopo

Um bloco é delimitado por `{` e `}`. Variáveis declaradas dentro de um bloco só existem nele:

```text
if true {
    variable inside = 2
}
print(inside)            # error CB101: name not declared
```

Alterar uma variável declarada fora do bloco altera o valor original.

## Próximo passo

[4. Funções](04-funcoes.md)
