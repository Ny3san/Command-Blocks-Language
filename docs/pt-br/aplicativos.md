# Aplicativos e automações

[Índice](../../README.md) · [Primeiros passos](primeiros-passos.md)

## O que funciona hoje

Command Blocks cria **aplicativos de terminal**: ferramentas que recebem dados, fazem cálculos, trabalham com listas/mapas e leem ou escrevem arquivos autorizados. Janelas, botões e atalhos globais ainda não existem.

Para começar com um projeto mínimo:

```text
cb create app meu_app
cb run meu_app/main.cb
```

Use `--lang en` na criação para gerar o exemplo em inglês. O template contém apenas uma instrução de impressão, sem estrutura desnecessária.

Crie `saudacao.cb`:

```text
variavel nome = io.ler_texto("Qual é seu nome? ")
imprimir("Olá, " + nome)
```

```text
cb run saudacao.cb
cb build saudacao.cb --output saudacao.exe
```

No Windows, o segundo comando gera um executável para a mesma arquitetura. Quem recebe esse aplicativo não precisa instalar `cb`, Rust ou Python. Ele contém o interpretador nativo e o fonte; não é compilação AOT nem proteção de propriedade intelectual. Arquivos auxiliares e módulos opcionais não são incluídos automaticamente. Em outros sistemas, use o nome de executável apropriado.

## Automação de arquivos

Crie `relatorio.cb`:

```text
permissao arquivo

variavel nomes = ["Steve", "Alex"]
variavel linhas = texto.juntar(nomes, "\n")
arquivo.escrever("jogadores.txt", linhas)
imprimir("Relatório criado")
```

```text
cb run relatorio.cb --allow arquivo
```

A pasta da fonte é a raiz permitida. `escrever` substitui o conteúdo do arquivo solicitado; use um destino próprio. A permissão no código documenta a necessidade; a autorização na execução concede o acesso. O aplicativo empacotado também exige a autorização correspondente.

## Aplicativo conectado ao jogo

Um aplicativo pode solicitar ações a um servidor local autenticado. A [ponte atual](integracoes.md) envia mensagens ou teletransporta um jogador por nome. Ela não observa a tela, não detecta teclas e não lê inventários.

Para a sequência “apertar G, achar um item, selecionar, usar e trocar”, falta um adaptador de cliente com acesso a essas informações. A [proposta](expansao.md) descreve essa extensão sem apresentá-la como funcional.
