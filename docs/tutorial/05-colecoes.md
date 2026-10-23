# 5. Coleções

Listas e mapas agrupam vários valores. Esta página mostra como criá-los, percorrê-los e entender como são compartilhados.

## Listas

```text
variable players = list.create()
list.add(players, "Steve")
list.add(players, "Alex")

print(players[0])             # Steve
print(list.length(players))   # 2
```

Os índices começam em zero. Índices negativos e índices fora do intervalo geram erro `CB107`.

Para percorrer uma lista:

```text
variable players = ["Steve", "Alex"]

for each player in players {
    print(text.upper(player))
}
```

`list.sort` devolve uma **nova** lista ordenada e não altera a original:

```text
variable players = ["Steve", "Alex"]
variable sorted = list.sort(players)

print(sorted[0])              # Alex
print(players[0])             # Steve
```

## Mapas

```text
variable scores = {"Steve": 10, "Alex": 20}
map.set(scores, "Alex", 25)
print(scores["Alex"])         # 25
```

As chaves são sempre textos. Ao exibir um mapa, as chaves aparecem em ordem alfabética:

```text
print({"b": 1, "a": 2})       # {"a":2,"b":1}
```

## Referência e cópia

Atribuir uma coleção a outra variável **não copia** a coleção. As duas variáveis apontam para o mesmo valor:

```text
variable a = [1]
variable b = a
b[0] = 9
print(a[0])                   # 9
```

Inserir um valor dentro de uma coleção **copia** esse valor:

```text
variable inner = [1]
variable outer = list.create()
list.add(outer, inner)
inner[0] = 2
print(outer[0][0])            # 1
```

Duas listas com o mesmo conteúdo não são iguais com `==`. A comparação verifica se são o mesmo objeto.

Para todas as funções de coleções, veja [Biblioteca padrão](../biblioteca/biblioteca-padrao.md).

## Próximo passo

[6. Eventos e testes](06-eventos-e-testes.md)
