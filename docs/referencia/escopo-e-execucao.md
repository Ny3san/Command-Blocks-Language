# Escopo e execução

## Escopo

- **Globais:** variáveis e constantes declaradas no nível principal são visíveis em todo o arquivo, inclusive dentro de funções e eventos.
- **Blocos:** variáveis declaradas dentro de `if`, `while`, `repeat`, `for each`, funções, eventos ou testes existem somente nesse bloco.
- **Atribuição:** atribuir a um nome existente altera a variável que o declarou, mesmo que a atribuição esteja em outro bloco.
- **Parâmetros:** existem somente dentro da função.

Não há closures nesta versão. Uma função enxerga globais, mas não variáveis de blocos externos onde não foi declarada.

## Ordem de execução

1. As instruções do nível principal são executadas na ordem do arquivo.
2. Em seguida, os eventos `start` são executados, na ordem de declaração.

Por isso, funções ou eventos que leem uma variável declarada depois de seu uso podem falhar. Declare as variáveis antes de usá-las.

## Eventos

- `event.emit` executa todos os eventos com o nome indicado antes de devolver o controle.
- Os eventos não recebem dados.
- Não há agendamento, paralelismo ou remoção de eventos.

## Testes

`cb test` executa cada bloco `test` com globais novas. Os eventos `start` não são executados.

## Limites de execução

Por padrão, o interpretador permite até 100000 passos (`--max-steps`). Recursão e aninhamento também têm limites. Ao atingir um limite, a execução para com erro `CB302`.
