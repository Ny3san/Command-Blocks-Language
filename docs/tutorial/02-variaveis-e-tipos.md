# 2. Variáveis e tipos

Nesta etapa você aprende a guardar valores em variáveis e constantes.

## Variáveis

Uma variável é declarada com `variable` e recebe um valor inicial:

```text
variable points = 0
points = points + 1
print(points)            # 1
```

O tipo da variável é definido pelo valor inicial. Atribuir um valor de outro tipo é um erro:

```text
variable points = 0
points = "many"          # error CB103: incompatible types
```

## Constantes

Uma constante é declarada com `constant` e não pode ser reatribuída:

```text
constant limit = 10
limit = 20               # error CB104: cannot reassign a constant
```

## Tipos básicos

| Tipo | Exemplos | Descrição |
|---|---|---|
| Inteiro (`Int`) | `0`, `-12` | Inteiro de 32 bits com sinal |
| Número (`Number`) | `1.5`, `0.25` | Decimal de 64 bits |
| Texto (`Text`) | `"Olá"`, `'Olá'` | Sequência de caracteres Unicode |
| Booleano (`Bool`) | `true`, `false` | Verdadeiro ou falso |
| Lista (`List`) | `[1, 2, 3]` | Coleção ordenada |
| Mapa (`Map`) | `{"name": "Alex"}` | Coleção de pares chave-valor, com chaves de texto |
| Nulo (`Null`) | `null` | Ausência de valor |
| Segredo (`Secret`) | `secret("TOKEN")` | Valor lido do ambiente; nunca é exibido |

Os detalhes de cada tipo estão em [Tipos](../referencia/tipos.md).

## Texto e números

```text
variable level = 3
print("Level: " + level)     # Level: 3
```

`+` concatena um texto com um valor simples. Para converter explicitamente, use `text.string` (ou `texto.converter`).

## Próximo passo

[3. Decisões e repetições](03-decisoes-e-repeticoes.md)
