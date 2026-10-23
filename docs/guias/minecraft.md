# Minecraft: datapacks

Command Blocks gera **datapacks** para Minecraft Java Edition. Um datapack é uma pasta com funções que o jogo executa.

Versões suportadas: Java 1.21 e 1.21.1.

## Gerar um datapack

```text
cb convert examples/pontos.cb --target minecraft --mc-version 1.21.1 --namespace pontos --output datapack
```

- `--output` é a pasta criada. Se ela já existir, o comando recusa e não substitui o conteúdo.
- O compilador verifica o programa inteiro antes de gravar. Se houver algo incompatível, nenhum arquivo é criado.

## Instalar no jogo

1. Copie a pasta gerada para `datapacks` dentro de um mundo **de teste**.
2. Execute `/reload` no jogo.
3. Confira as mensagens no chat ou no log.

Este passo exige acesso aos comandos do mundo.

## Como o programa vira datapack

| Command Blocks | Minecraft |
|---|---|
| Variáveis `Int` e `Bool` globais | Scores em um objetivo próprio |
| Funções sem parâmetros | Arquivos `.mcfunction` |
| `print` | Mensagem no chat (`tellraw @a`) |
| `if` / `else` | Comandos `execute` |
| `repeat N times` | N chamadas de uma função auxiliar, com N entre 0 e 1000 |
| `event start` | Executado ao carregar o datapack |
| `event tick` | Executado a cada tick |

Os scores são globais ao programa, não por jogador. Ao recarregar o datapack, as variáveis voltam ao valor inicial.

## Chamar as funções a partir de command blocks

O datapack não executa arquivos `.cb` dentro do Minecraft. Um command block pode chamar uma função do pacote:

```mcfunction
function pontos:load
```

Os nomes internos dos arquivos gerados (como `user/f0`) não são uma API estável.

## Operações disponíveis

Este exemplo passa em `cb check --target minecraft`:

```text
event start {
    minecraft.message("@a", "Welcome to Command Blocks!")
    minecraft.give_item("@a", "minecraft:diamond", 1)
    minecraft.teleport("@a", 0, 80, 0)
    minecraft.weather("clear")
    minecraft.time(1000)
}
```

| Função | Descrição |
|---|---|
| `minecraft.message(selector, text)` | Envia uma mensagem |
| `minecraft.give_item(selector, item, amount)` | Entrega um item |
| `minecraft.teleport(selector, x, y, z)` | Teletransporta |
| `minecraft.weather(kind)` | Define o clima: `clear`, `rain` ou `thunder` |
| `minecraft.time(ticks)` | Define o horário |

Seletores aceitos: `@a`, `@p`, `@r`, `@s`, `@e` ou um nome de jogador. O contexto de execução do Minecraft continua valendo: `@s` não encontra entidade se a função não tiver um executor.

Itens aceitos nesta versão: `diamond`, `stone`, `dirt`, `apple`, `stick`, `iron_ingot`, `gold_ingot`, `emerald`, `diamond_sword`, `bow` e `arrow`, sempre com o prefixo `minecraft:`.

## Limitações

Ainda não são suportados:

- funções com parâmetros ou valor de retorno;
- variáveis locais, listas, mapas, números decimais;
- divisão, resto e `while`;
- recursão e chamadas a APIs externas;
- detecção de teclas, controle da hotbar e interação com o mouse.

O backend também aplica um limite estático de complexidade para evitar expansão excessiva.

## Validação

Os testes verificam a estrutura dos arquivos gerados e simulam os scores e as condições. **O datapack ainda precisa ser validado em um Minecraft real.** Um conversor bem-sucedido não garante o comportamento dentro do jogo.
