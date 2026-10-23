# 6. Eventos e testes

## Eventos

Um evento é um bloco que roda quando é disparado:

```text
variable points = 0

event score {
    points = points + 1
}

event start {
    repeat 3 times {
        event.emit("score")
    }
    print(points)        # 3
}
```

- `event start` roda no início do programa. Também pode ser escrito `event inicio`.
- `event.emit("nome")` dispara todos os eventos com aquele nome. A execução é síncrona, e o evento não recebe dados nesta versão.
- Se nenhum evento tiver o nome emitido, nada acontece.
- Nomes em português são convertidos para os nomes canônicos: `inicio` → `start`, `atualizar` → `tick`, `mensagem` → `message`.

A ordem de execução está em [Escopo e ordem de execução](../referencia/escopo-e-execucao.md).

## Testes

Um teste é um bloco com nome e asserções:

```text
function add(a, b) {
    return a + b
}

test "sums correctly" {
    assert(add(2, 2) == 4)
}
```

Execute com:

```text
cb test arquivo.cb
```

O comando mostra quantos testes passaram e quantos falharam. Se algum teste falhar, o código de saída é `1`.

Cada teste roda com variáveis globais novas. Os eventos `start` não são executados em `cb test`, e permissões externas não são concedidas.

## Próximo passo

[7. Aplicativo de terminal](07-aplicativo-de-terminal.md)
