# Linguagem

Esta página descreve a sintaxe. A gramática formal completa está em [grammar.ebnf](grammar.ebnf).

## Comentários

```text
# comentário
// também é comentário
```

## Valores e tipos

| Tipo | Exemplo | Observação |
|---|---|---|
| Inteiro | `0`, `-12` | 32 bits com sinal |
| Número | `1.5` | Decimal de 64 bits |
| Texto | `"Olá"` ou `'Olá'` | Aceita escapes como `\n`, `\t`, `\"` |
| Booleano | `true`, `false` | Condições precisam ser booleanas |
| Lista | `[1, 2, 3]` | Índices começam em zero |
| Mapa | `{"nome": "Alex"}` | Chaves são textos |
| Nulo | `null` | Resultado de função sem `return` |
| Segredo | `secret("TOKEN")` | Valor de variável de ambiente; nunca aparece na tela |

## Variáveis e constantes

```text
variable score = 0
constant max_score = 10
score = score + 1
```

Uma constante não pode ser alterada. O tipo é descoberto automaticamente a partir do valor inicial, e um valor de tipo diferente não pode ser atribuído depois.

## Operadores

| Tipo | Operadores |
|---|---|
| Aritmética | `+`, `-`, `*`, `/`, `%` |
| Comparação | `==`, `!=`, `<`, `>`, `<=`, `>=` |
| Lógica | `and`, `or`, `not` |

- `+` também junta textos: `"Ponto " + 3` vira `"Ponto 3"`.
- `/` produz um número decimal.
- `%` é o resto da divisão.
- Dividir por zero é um erro.
- Use parênteses para deixar a precedência clara.

## Condições

```text
if score >= 5 {
    print("Bom!")
} else {
    print("Tente de novo")
}
```

## Repetições

```text
# repete um número fixo de vezes
repeat 5 times {
    print("oi")
}

# enquanto a condição for verdadeira
while score < 10 {
    score = score + 1
}

# percorre uma lista
for each name in ["Alex", "Steve"] {
    print(name)
}
```

## Funções

```text
function add(a, b) {
    return a + b
}

print(add(2, 3))
```

- Funções são declaradas no nível principal do arquivo.
- Parâmetros existem só dentro da função.
- Sem `return`, a função devolve `null`.
- Não há funções anônimas nem módulos de arquivos nesta versão.

## Eventos

Eventos são blocos que rodam quando algo acontece.

```text
variable points = 0

event score {
    points = points + 1
}

event start {
    repeat 3 times {
        event.emit("score")
    }
    print(points)
}
```

| Evento | Quando roda |
|---|---|
| `start` | No início do programa (em português: `inicio`) |
| `tick` | A cada tick do jogo; só no Minecraft (em português: `atualizar`) |

Você também pode criar eventos com nomes próprios e disparar com `event.emit("nome")`. Os eventos são síncronos e não recebem dados nesta versão.

## Testes

```text
test "adds numbers" {
    assert(add(2, 2) == 4)
}
```

Execute com `cb test arquivo.cb`. O comando mostra quantos testes passaram e quantos falharam. Se algum falhar, o código de saída é `1`.

## Imports e permissões

```text
import text
permission file
```

- `import` habilita bibliotecas opcionais.
- `permission` declara que o programa precisa de um recurso externo, como arquivos ou rede.
- A declaração sozinha não basta: o usuário também precisa autorizar na linha de comando com `--allow`. Veja [Segurança](seguranca.md).

## Palavras em português e inglês

Você pode escrever com palavras-chave em português, em inglês ou misturar os dois idiomas. A forma recomendada é o inglês.

| Português | Inglês |
|---|---|
| `variavel` | `variable` |
| `constante` | `constant` |
| `funcao` | `function` |
| `retornar` | `return` |
| `se` / `senao` | `if` / `else` |
| `enquanto` | `while` |
| `repetir` / `vezes` | `repeat` / `times` |
| `para cada` / `em` | `for each` / `in` |
| `evento` / `inicio` / `atualizar` | `event` / `start` / `tick` |
| `importar` / `permissao` / `teste` | `import` / `permission` / `test` |
| `imprimir` / `verificar` / `segredo` | `print` / `assert` / `secret` |
| `verdadeiro` / `falso` / `nulo` | `true` / `false` / `null` |
| `e` / `ou` / `nao` | `and` / `or` / `not` |
