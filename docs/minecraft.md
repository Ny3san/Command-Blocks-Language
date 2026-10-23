# Minecraft: datapacks

Command Blocks pode gerar um **datapack** para Minecraft Java Edition. Um datapack é uma pasta com funções que o jogo executa.

Versões suportadas: Java 1.21 e 1.21.1.

## Gerar o datapack

```text
cb convert examples/pontos.cb --target minecraft --mc-version 1.21.1 --namespace pontos --output meu_datapack
```

- `--output` é a pasta que será criada. Se ela já existir, o comando recusa e não substitui.
- Se o programa usar algo que o datapack não suporta, o comando não gera nada parcial.

## Instalar

1. Copie a pasta `meu_datapack` para `datapacks` dentro de um mundo **de teste**.
2. No jogo, digite `/reload`.
3. Verifique as mensagens no chat ou no log.

Este passo depende de você ter acesso aos comandos do mundo.

## Como o programa vira datapack

| Command Blocks | Minecraft |
|---|---|
| Variáveis inteiras e booleanas globais | Scores em um objetivo próprio |
| Funções sem parâmetros | Arquivos `.mcfunction` |
| `print` | Mensagem no chat (`tellraw @a`) |
| `if` / `else` | Comandos `execute` |
| `repeat N times` | N chamadas de uma função auxiliar (N entre 0 e 1000) |
| `event start` | Executa ao carregar o datapack |
| `event tick` | Executa a cada tick |

Os scores são globais ao programa, não por jogador. Ao recarregar o datapack, as variáveis voltam ao valor inicial.

## Usar com command blocks

O datapack não roda um arquivo `.cb` dentro do Minecraft. Um command block pode chamar uma função do pacote:

```text
function pontos:load
```

## Exemplo de evento

```text
event start {
    minecraft.message("@a", "Bem-vindo ao Command Blocks!")
    minecraft.give_item("@a", "minecraft:diamond", 1)
    minecraft.teleport("@a", 0, 80, 0)
    minecraft.weather("clear")
    minecraft.time(1000)
}
```

Este programa **não pode ser convertido** para datapack em todas as suas partes. Use `cb check` para ver o que é compatível.

## Seletores e itens

- Seletores aceitos: `@a`, `@p`, `@r`, `@s`, `@e` ou um nome de jogador.
- Itens aceitos nesta versão: `diamond`, `stone`, `dirt`, `apple`, `stick`, `iron_ingot`, `gold_ingot`, `emerald`, `diamond_sword`, `bow`, `arrow` (sempre com `minecraft:`).
- Clima aceito: `clear`, `rain` ou `thunder`.

## Limitações

Ainda não existem nesta versão:

- funções com parâmetros ou valor de retorno;
- variáveis locais, listas, mapas e números decimais;
- divisão, resto e `while`;
- recursão;
- chamadas a APIs externas.

Também não há detecção de teclas, controle da hotbar ou interação com o mouse. Isso depende de um adaptador de cliente, descrito em [Roadmap](roadmap.md).

Os testes verificam a estrutura dos arquivos gerados e simulam os scores. **Ainda é preciso validar o datapack em um Minecraft real.**
