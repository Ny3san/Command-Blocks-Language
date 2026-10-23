# 1. Primeiro programa

Este tutorial apresenta a linguagem Command Blocks (CB) passo a passo. Ao final, você terá escrito programas com variáveis, decisões, repetições, funções, eventos e testes.

**Pré-requisitos:** o executável `cb` disponível no `PATH`. Para compilá-lo a partir do código-fonte, veja [Instalação](../ferramentas/instalacao.md).

## Escrevendo um programa

Crie um arquivo chamado `hello.cb` com o conteúdo:

```text
print("Hello, world!")
```

Arquivos de Command Blocks usam a extensão `.cb`. Cada instrução ocupa uma linha; ponto e vírgula também separa instruções.

## Executando

```text
cb run hello.cb
```

Saída:

```text
Hello, world!
```

O comando `cb run` lê o arquivo, verifica se ele está correto e só então o executa. Se houver um erro, a execução não começa e a mensagem indica a linha e a coluna do problema.

## Comentários

Comentários começam com `#` ou `//` e vão até o fim da linha:

```text
# this is a comment
// this is also a comment
print("visible")
```

## Próximo passo

[2. Variáveis e tipos](02-variaveis-e-tipos.md)
