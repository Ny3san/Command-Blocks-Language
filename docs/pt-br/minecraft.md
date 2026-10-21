# Minecraft, datapacks e command blocks

[Índice](../../README.md) · [Integrações externas](integracoes.md)

## Conversão atual

```text
cb convert examples/pontos.cb --target minecraft --mc-version 1.21.1 --namespace pontos --output meu_datapack
```

O compilador analisa todo o programa antes de publicar o diretório. Se encontrar recurso incompatível, não produz um datapack parcial. Um destino existente não é substituído.

Copie `meu_datapack` para a pasta `datapacks` do mundo Java correspondente. Faça isso em um mundo de teste. Use `/reload` e consulte a saída/log do jogo. Esta etapa exige acesso aos comandos do mundo.

## Como o programa se transforma

| Command Blocks | Minecraft |
|---|---|
| Variáveis inteiras/booleanas globais | Jogadores fictícios em um objetivo scoreboard próprio |
| Funções sem parâmetros | Arquivos `.mcfunction` |
| `imprimir` | `tellraw @a` |
| `se`/`senao` | `execute` e funções auxiliares |
| `repetir N vezes` | N chamadas de uma função auxiliar, com N literal entre 0 e 1000 |
| `evento inicio` | Tag de função `minecraft:load` |
| `evento atualizar` | Tag de função `minecraft:tick` |

Os scores são globais ao programa, não por jogador. `inicio` também é executado ao recarregar datapacks. Recarregar reinicializa as variáveis; criar novamente um objetivo já existente pode gerar um aviso, sem apagar o objetivo.

## Estrutura gerada

```text
pack.mcmeta
cb-report.json
data/minecraft/tags/function/load.json
data/minecraft/tags/function/tick.json
data/<namespace>/function/load.mcfunction
data/<namespace>/function/tick.mcfunction
data/<namespace>/function/user/...
data/<namespace>/function/internal/...
```

O relatório registra perfil, namespace, objetivo, scores e quantidade de funções. Os diretórios são `function`, no singular. Suporte explícito: Java 1.21 e 1.21.1, formato 48. Outros patches são recusados; não basta trocar o número no arquivo para garantir compatibilidade. [Notas oficiais da versão 1.21](https://www.minecraft.net/pt-pt/article/minecraft-java-edition-1-21).

## Usar com command blocks

O produto gerado é um datapack. Não existe um interpretador `.cb` dentro de um command block. Um command block pode chamar uma função instalada com `function namespace:caminho`.

Por exemplo, `function pontos:load` dispara novamente a inicialização do pacote criado acima, inclusive zerando suas variáveis. Para ações específicas, consulte os arquivos gerados; nomes internos e nomes `user/fN` são detalhes da versão inicial, não uma API estável. Exportações com nomes públicos previsíveis estão propostas para a próxima etapa.

## Operações básicas

```text
evento inicio {
    minecraft.mensagem("@a", "Bem-vindo!")
    minecraft.dar_item("@a", "minecraft:diamond", 1)
    minecraft.teleportar("@a", 0, 80, 0)
    minecraft.clima("clear")
    minecraft.horario(1000)
}
```

Seletores aceitos são simples (`@a`, `@p`, `@r`, `@s`, `@e`) ou nomes de jogador. O contexto de execução do Minecraft continua valendo: `@s` não encontra uma entidade se a função não tiver executor de entidade.

Itens aceitos nesta base: diamond, stone, dirt, apple, stick, iron_ingot, gold_ingot, emerald, diamond_sword, bow e arrow, sempre com prefixo `minecraft:`. Clima: clear, rain ou thunder. Essa lista pequena evita anunciar validação de todo o registro de itens.

## Limitações explícitas

Sem funções com parâmetros/retorno, variáveis locais, listas/mapas, textos mutáveis, decimais, divisão/resto, while, recursão ou APIs externas. Constantes textuais literais e concatenação de texto com inteiros em mensagens são aceitas. Booleanos variáveis podem controlar condições, mas não são mensagens dinâmicas no backend inicial.

Existe limite estático de complexidade para reduzir expansão excessiva. Não há detecção de entrada de jogadores, teclas arbitrárias, controle de hotbar ou interação de mouse. Para isso, veja a [proposta de integração de cliente](expansao.md).

Os testes do compilador verificam a estrutura e simulam o subconjunto de scoreboards e condições. Ainda é necessária validação em um Minecraft real; sucesso do conversor não é uma alegação de teste dentro do jogo.
