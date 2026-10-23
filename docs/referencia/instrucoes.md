# Instruções

## Declarações

```text
variable <name> = <expression>
constant <name> = <expression>
<name> = <expression>
```

O nome precisa ser declarado antes de ser atribuído. Uma constante não pode ser atribuída depois de declarada.

## Controle de fluxo

Os trechos abaixo são modelos de sintaxe; os nomes entre `<>` devem ser substituídos.

```text
if <condition> {
    <statements>
} else {
    <statements>
}

while <condition> {
    <statements>
}

repeat <count> times {
    <statements>
}

for each <name> in <collection> {
    <statements>
}
```

- `condition` deve ser `Bool`.
- `count` é uma expressão que resulta em número inteiro.
- `for each` percorre os itens de uma lista.

## Funções

```text
function <name>(<parameter1>, <parameter2>) {
    return <expression>
}
```

- Funções são declaradas no nível principal do arquivo.
- `return` sem valor não é aceito nesta versão; sem `return`, o resultado é `null`.
- Funções não podem ser declaradas dentro de blocos.

## Eventos

```text
event <name> {
    <statements>
}

event.emit("<name>")
```

`event start` (ou `event inicio`) é executado no início do programa. Os demais eventos são executados quando emitidos.

## Testes

```text
test "<description>" {
    assert(<condition>)
}
```

Blocos `test` só são executados por `cb test`. Veja [Eventos e testes](../tutorial/06-eventos-e-testes.md).

## Imports e permissões

```text
import <library>
permission <resource>
```

- `import` habilita uma biblioteca opcional.
- `permission` declara que o programa usa um recurso externo, como `file`, `net`, `window` ou `minecraft_bridge`.
- A declaração não basta. O usuário também precisa autorizar com `--allow` na execução. Veja [Permissões e segurança](../guias/seguranca.md).

`import`, `permission`, `function`, `event` e `test` só podem aparecer no nível principal do arquivo.

## Palavras reservadas e aliases

| Inglês | Português |
|---|---|
| `variable` | `variavel` |
| `constant` | `constante` |
| `function` | `funcao` |
| `return` | `retornar` |
| `if` / `else` | `se` / `senao` |
| `while` | `enquanto` |
| `repeat` / `times` | `repetir` / `vezes` |
| `for each` / `in` | `para cada` / `em` |
| `event` / `start` / `tick` | `evento` / `inicio` / `atualizar` |
| `import` / `permission` / `test` | `importar` / `permissao` / `teste` |
| `print` / `assert` / `secret` | `imprimir` / `verificar` / `segredo` |
| `true` / `false` / `null` | `verdadeiro` / `falso` / `nulo` |
| `and` / `or` / `not` | `e` / `ou` / `nao` |

Os dois idiomas podem aparecer no mesmo programa. O inglês é a forma recomendada e a usada nos exemplos.
