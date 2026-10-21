# Manual — Command Blocks 0.1

## Instalação e primeiros passos

O compilador e o interpretador são escritos em Rust; o núcleo não possui dependências de terceiros. Compile com Rust estável e um linker apropriado:

```text
cargo build --release --offline
cargo install --path . --offline
cb --version
cb run examples/ola.cb
cb run examples/hello.cb --lang en
```

No Windows, o perfil Rust MSVC exige **Visual Studio Build Tools com C++ e Windows SDK**. `cargo check` não precisa linkar; portanto, passar nele não prova que `cargo build` funcionará.

Uma alternativa oficial para projetos Rust puros é a toolchain `stable-x86_64-pc-windows-gnullvm`, com componente `rust-mingw` e linker `rust-lld`. Consulte [as instruções oficiais de Rust](https://doc.rust-lang.org/rustc/platform-support/windows-gnullvm.html). Os scripts locais de desenvolvimento, quando usados, mantêm essa toolchain em `target/rustup-local`.

Depois de compilado, copie `cb.exe` (Windows) ou `cb` (Linux/macOS) para uma pasta no PATH. O executável não requer Python, Cargo ou Rust instalados. Compilar para outro sistema exige a toolchain desse sistema; não é conversão automática de executáveis.

## Sintaxe e tipos

```text
variavel pontos = 0
constante limite = 10
funcao somar(a, b) {
    retornar a + b
}
enquanto pontos < limite {
    pontos = somar(pontos, 1)
}
se pontos >= limite {
    imprimir("Você venceu: " + pontos)
} senao {
    imprimir("Continue")
}
```

Instruções são separadas por nova linha ou `;`. Comentários começam com `#` ou `//`. Strings aceitam aspas simples/duplas e escapes básicos. Identificadores aceitam Unicode. Use parênteses para explicitar precedência; `and/e` e `or/ou` fazem curto-circuito. [Gramática](../grammar.ebnf).

| Tipo | Exemplos | Semântica |
|---|---|---|
| Inteiro | `0`, `-12` | 32 bits com sinal; soma/subtração/multiplicação dão a volta no limite, como scoreboards |
| Número | `1.5` | Float de 64 bits; NaN e infinito não são resultados válidos |
| Texto | `"Olá"` | UTF-8; índices e tamanho contam caracteres Unicode, não grafemas |
| Booleano | `verdadeiro`, `false` | Condições exigem booleano, não conversão implícita de números |
| Lista | `[1, 2]` | Elementos heterogêneos; índice começa em zero; índices negativos recusados |
| Mapa | `{"nome": "Alex"}` | Chaves textuais, ordenadas na exibição |
| Nulo | `nulo`, `null` | Retorno de funções sem `retornar` |
| Segredo | `segredo("TOKEN")` | Obtido do ambiente; representação sempre censurada |

Variáveis têm tipo inferido. Reatribuição de tipo incompatível é recusada. Parâmetros, retorno de funções e valores extraídos de coleções podem ter tipo `any` na IR; nesses casos a validação ocorre na execução. Não há sistema completo de inferência interprocedural.

`constante` impede reatribuir o nome; não torna uma lista/mapa profundamente imutável. Coleções atribuídas a outro nome compartilham a mesma coleção. Inserções em coleções copiam o valor inserido, evitando ciclos. Igualdade de listas/mapas compara identidade, não igualdade estrutural.

`+` concatena texto com valores simples. `/` produz número decimal; `%` é resto, com o sinal do dividendo. Divisão por zero é erro. Números aleatórios servem para jogos, **não para gerar credenciais**.

## Português e inglês

Pode misturar os idiomas. Mensagens usam `--lang pt-br` por padrão e `--lang en` em inglês.

| Português | Inglês |
|---|---|
| variavel / constante / funcao / retornar | variable / constant / function / return |
| se / senao / enquanto | if / else / while |
| repetir / vezes | repeat / times |
| para cada … em | for each … in |
| evento / inicio / atualizar | event / start / tick |
| importar / permissao / teste | import / permission / test |
| imprimir / verificar / segredo | print / assert / secret |
| verdadeiro / falso / nulo | true / false / null |
| e / ou / nao | and / or / not |

## Funções, escopos, eventos e testes

Funções nomeadas têm escopo léxico global, parâmetros locais e retorno opcional. Blocos criam escopos; alterar uma variável existente altera o escopo que a declarou. Não há closures, funções anônimas ou módulos de arquivos na 0.1. Funções, eventos, testes, imports e permissões só podem ser declarados no nível principal.

Primeiro executam as declarações e instruções principais, na ordem de origem. Depois são emitidos os eventos `inicio/start`, na ordem de declaração. Evite chamar funções que leem globais ainda não inicializadas.

```text
variavel pontos = 0
evento marcar {
    pontos = pontos + 1
}
evento inicio {
    repetir 3 vezes { evento.emitir("marcar") }
    imprimir(pontos)
}
```

Eventos personalizados são síncronos, sem payload nesta versão. Emitir um evento sem handlers não faz nada. `tick/atualizar` só funciona no Minecraft. Não há scheduler, remoção dinâmica de handlers ou concorrência.

```text
teste "soma corretamente" {
    verificar(2 + 2 == 4)
}
```

`cb test [pasta-ou-arquivo]` encontra `.cb`, analisa os arquivos e executa cada teste com globais novas. Executa inicializadores principais, mas não eventos start. Permissões externas não são concedidas em testes. Reporta aprovados/falhas; qualquer falha retorna código 1. Sem testes, informa zero testes. `cargo test` testa o próprio compilador.

## Bibliotecas implementadas

Os pares abaixo são inglês / português; os argumentos aparecem na mesma ordem. Imports do núcleo são opcionais; os opcionais exigem import e habilitação.

| Módulo | Funções |
|---|---|
| `text` / `texto` | `print/imprimir(v)`, `upper/maiusculas(t)`, `lower/minusculas(t)`, `trim/aparar(t)`, `join/juntar(lista, separador)`, `split/separar(t, separador)`, `replace/substituir(t, de, para)` |
| `text` / `texto` | `contains/contem(t, parte)`, `starts_with/comeca_com(t, parte)`, `ends_with/termina_com(t, parte)`, `length/tamanho(t)`, `string/converter(v)`, `number/numero(t)`, `format/formatar(t, mapa)` |
| `math` / `matematica` | `abs/absoluto`, `round/arredondar`, `sqrt/raiz`, `sin/seno`, `cos/cosseno`, `tan/tangente`, `degrees/graus`, `radians/radianos`: um número |
| `math` / `matematica` | `min/minimo(a,b)`, `max/maximo(a,b)`, `pow/potencia(a,b)`, `random/aleatorio(inteiro_min,inteiro_max)` inclusivo |
| `list` / `lista` | `create/criar()`, `add/adicionar(lista,v)`, `remove/remover(lista,v)`, `get/obter(lista,indice)`, `contains/contem(lista,v)`, `sort/ordenar(lista)`, `length/tamanho(lista)` |
| `map` / `mapa` | `create/criar()`, `set/definir(mapa,chave,v)`, `get/obter(mapa,chave)`, `contains/contem(mapa,chave)`, `keys/chaves(mapa)`, `length/tamanho(mapa)` |
| `io` / `entrada_saida` | `print/imprimir(v)`, `read_text/ler_texto(prompt)`, `read_number/ler_numero(prompt)`, `environment/ambiente()` |
| `event` / `evento` | `emit/emitir(nome)` |
| `time` / `tempo` | `now/agora()` duração monotônica desde o runtime, `sleep/esperar(segundos)` de 0 a 60 |
| `file` / `arquivo` | `read/ler(caminho)`, `write/escrever(caminho,texto)`, `append/adicionar(caminho,texto)`, `exists/existe(caminho)`, `mkdir/criar_pasta(caminho)`, `list/listar(caminho)` |

`list.sort` retorna outra lista, ordena só números ou só textos; `remove` remove a primeira ocorrência ou dá erro. `text.format` substitui `{chave}` pelo mapa, sem executar código. `math.round` arredonda empates para longe de zero. Funções matemáticas, exceto random, retornam número decimal.

## CLI

```text
cb run programa.cb [--allow arquivo] [--max-steps 100000] [--lang en]
cb check programa.cb [--target minecraft]
cb convert programa.cb --target minecraft --output pack --mc-version 1.21.1 --namespace meu_mapa
cb build programa.cb --output aplicativo.exe
cb test [caminho]
cb format programa.cb
cb create minecraft-datapack meu_mapa
cb create discord-bot meu_bot
cb install minecraft_bridge
cb install discord
cb list
cb info discord
cb help texto
```

`check` analisa sem executar nem acessar serviços. `check --target minecraft` valida a geração inteira em memória. `format` ajusta indentação e preserva comentários/strings; cria `.bak` antes de alterar. Se esse backup já existe e há outra alteração, falha para preservá-lo.

`build` empacota o interpretador nativo atual com a fonte em um executável para o mesmo sistema/arquitetura. **Não é compilação AOT do programa**, não oculta o fonte e não inclui segredos de ambiente nem arquivos adicionais. No Windows, use extensão `.exe`.

`install` habilita módulos embarcados em `.cb-libraries` na pasta atual. Não é um gerenciador de pacotes remoto. `discord` habilita apenas sua interface inicial, com aviso; `database`, `web` e `graphics` não estão disponíveis.

## Minecraft e conversão

Veja [Minecraft](minecraft.md) para o subconjunto aceito, estrutura, instalação e limitações. O mesmo `examples/pontos.cb` roda externamente e gera scoreboards, funções e mensagens. Já `examples/minecraft.cb` só pode ser convertido.

## Permissões e segredos

```text
permissao arquivo
arquivo.escrever("config.txt", "modo = seguro")
imprimir(arquivo.ler("config.txt"))
```

Execute com `cb run programa.cb --allow arquivo`. Declaração sem concessão na CLI, ou concessão sem declaração, não basta. Aliases aceitos: arquivo/file, rede/net, processo/process, ponte_minecraft/minecraft_bridge.

Arquivos ficam restritos à pasta da fonte; caminhos absolutos, `..` e symlinks que apontem para fora são bloqueados. Não é uma sandbox do sistema operacional e não deve executar código hostil com permissões. O controle de caminhos não oferece proteção contra um processo externo que altere links simultaneamente.

`segredo("NOME")` só lê variáveis de ambiente. Segredos tipados são censurados na impressão e dentro de coleções; valores de variáveis de ambiente com TOKEN/PASSWORD/SECRET/API_KEY no nome também são removidos das saídas. Erros nunca incluem argumentos, trechos de código ou respostas privadas dos serviços. Isso não identifica automaticamente qualquer senha literal arbitrária: **não escreva credenciais no fonte**. O empacotamento contém o fonte.

Há limites de passos, recursão, aninhamento, tamanho de fonte/valores e respostas da ponte. Isso não constitui isolamento completo de memória/CPU por processo. A versão não executa processos externos; `process.run` está reservado e falha explicitamente.

## Erros

Formato: `arquivo:linha:coluna: CBxxx: causa`, seguido de sugestão. Falhas da CLI usam `<cli>:1:1`. Códigos: CB000 argumentos, CB001 lexer, CB002 sintaxe, CB009 I/O, CB101 nomes/escopos, CB102 função indisponível, CB103 tipos, CB104 constante, CB105 argumentos, CB106 tamanho/aninhamento, CB107 índice, CB108 operação, CB201 ambiente/backend incompatível, CB202 recurso opcional/ausente, CB203 versão, CB204 namespace, CB205 destino existente, CB301 permissão, CB302 limite, CB303 segredo ausente, CB304 integração, CB401 verificação.

## Extensão, plano e limites

[Arquitetura e como estender](arquitetura.md) · [Integrações e Discord](integracoes.md)

Não implementados: módulos de arquivos, funções anônimas, callbacks como valores, JSON de entrada, HTTP genérico, data/calendário, schedulers, concorrência, subprocessos, dispositivos, janelas, bancos de dados, web, áudio, gráficos, APIs Discord, servidor da ponte, plugins/mods, Bedrock e outras versões Minecraft. Sintaxes futuras da proposta original, como `5 segundos`, `evento.iniciar {}` e `discord.bot token = ...`, ainda não fazem parte da gramática.

Exemplos completos em `examples/`: `ola.cb`, `hello.cb`, `pontos.cb`, `collections.cb`, `events.cb`, `minecraft.cb`, `tests.cb`.
