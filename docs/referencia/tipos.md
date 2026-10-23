# Tipos

## Tipos primitivos

| Tipo | Faixa ou formato | Observações |
|---|---|---|
| `Int` | 32 bits com sinal (−2147483648 a 2147483647) | Estouro dá a volta: `2147483647 + 1` resulta em `-2147483648` |
| `Number` | Ponto flutuante de 64 bits | Resultados NaN e infinito não são válidos |
| `Text` | Sequência Unicode | Índices e tamanho contam caracteres, não grafemas |
| `Bool` | `true` ou `false` | Condições exigem `Bool` |
| `Null` | `null` | Valor de função sem `return` |
| `Secret` | Lido com `secret("NAME")` | Nunca é exibido ou incluído em erros |

## Tipos compostos

| Tipo | Descrição |
|---|---|
| `List` | Sequência indexada a partir de zero. Pode conter valores de tipos diferentes. |
| `Map` | Associação de textos a valores. A exibição ordena as chaves. |

## Inferência e atribuição

O tipo de uma variável é inferido do valor inicial. Atribuir um valor de outro tipo gera `CB103`.

Parâmetros de função, retornos de funções e valores retirados de coleções podem ter tipo desconhecido em tempo de análise. Nesses casos, a verificação ocorre durante a execução.

## Constantes

`constant` impede reatribuir o nome. Não torna listas ou mapas imutáveis: `constant items = [1]` ainda permite `list.add(items, 2)`.

## Igualdade

| Comparação | Resultado |
|---|---|
| `1 == "1"` | `false`, sem erro |
| `[1] == [1]` | `false`: listas e mapas são comparados por identidade |
| `null == null` | `true` |

`<`, `>`, `<=` e `>=` exigem tipos compatíveis. Comparar número com texto gera `CB103`.

## Coleções e referências

Atribuir uma lista ou mapa a outra variável cria outra referência ao mesmo objeto. Inserir um valor em uma coleção armazena uma **cópia** desse valor. Detalhes em [Coleções](../tutorial/05-colecoes.md).

## Conversões

Não há conversões implícitas entre texto e número, exceto na concatenação com `+`, em que o valor simples é convertido para texto:

```text
print("Score: " + 10)      # Score: 10
```

Para converter explicitamente:

| Função | Efeito |
|---|---|
| `text.string(v)` | Converte para texto |
| `text.number(t)` | Converte texto para número; texto inválido gera `CB103` |
