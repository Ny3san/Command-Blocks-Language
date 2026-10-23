# Bibliotecas

Cada função tem um nome em inglês e, quando existe, um alias em português. Os dois funcionam igual.

Exemplo:

```text
print(text.upper("olá"))      # OLÁ
imprimir(texto.maiusculas("olá"))  # OLÁ
```

## Texto (`text` / `texto`)

| Função | Faz |
|---|---|
| `print(v)` / `imprimir(v)` | Mostra um valor na tela |
| `text.upper(t)` / `texto.maiusculas(t)` | Põe em maiúsculas |
| `text.lower(t)` / `texto.minusculas(t)` | Põe em minúsculas |
| `text.trim(t)` / `texto.aparar(t)` | Remove espaços nas pontas |
| `text.join(lista, separador)` / `texto.juntar(...)` | Junta uma lista em um texto |
| `text.split(t, separador)` / `texto.separar(...)` | Separa um texto em uma lista |
| `text.replace(t, de, para)` / `texto.substituir(...)` | Troca um trecho por outro |
| `text.contains(t, parte)` / `texto.contem(...)` | Verifica se há um trecho |
| `text.starts_with(t, parte)` / `texto.comeca_com(...)` | Verifica o começo |
| `text.ends_with(t, parte)` / `texto.termina_com(...)` | Verifica o fim |
| `text.length(t)` / `texto.tamanho(t)` | Quantidade de caracteres |
| `text.string(v)` / `texto.converter(v)` | Converte um valor em texto |
| `text.number(t)` / `texto.numero(t)` | Converte um texto em número |
| `text.format(t, mapa)` / `texto.formatar(...)` | Troca `{chave}` pelos valores do mapa |

`text.format` apenas substitui os valores. Ele não executa código.

```text
print(text.format("Jogadores: {count}", {"count": 2}))
# Jogadores: 2
```

## Matemática (`math` / `matematica`)

| Função | Faz |
|---|---|
| `math.abs(x)` / `matematica.absoluto(x)` | Valor absoluto |
| `math.round(x)` / `matematica.arredondar(x)` | Arredonda |
| `math.sqrt(x)` / `matematica.raiz(x)` | Raiz quadrada |
| `math.sin`, `math.cos`, `math.tan` | Seno, cosseno e tangente |
| `math.degrees(x)` / `math.radians(x)` | Converte entre graus e radianos |
| `math.min(a, b)` / `math.max(a, b)` | Menor e maior valor |
| `math.pow(a, b)` / `matematica.potencia(...)` | Potência |
| `math.random(min, max)` / `matematica.aleatorio(...)` | Inteiro aleatório entre `min` e `max`, incluindo os dois |

Números aleatórios servem para jogos. **Não use para gerar senhas ou chaves.**

## Listas (`list` / `lista`)

| Função | Faz |
|---|---|
| `list.create()` / `lista.criar()` | Cria uma lista vazia |
| `list.add(lista, v)` / `lista.adicionar(...)` | Adiciona no fim |
| `list.remove(lista, v)` / `lista.remover(...)` | Remove a primeira ocorrência (dá erro se não existir) |
| `list.get(lista, i)` / `lista.obter(...)` | Pega o item na posição `i` |
| `list.contains(lista, v)` / `lista.contem(...)` | Verifica se o valor está na lista |
| `list.sort(lista)` / `lista.ordenar(...)` | Retorna uma **nova** lista ordenada (a original não muda) |
| `list.length(lista)` / `lista.tamanho(...)` | Quantidade de itens |

`list.sort` só ordena listas só de números ou só de textos.

## Mapas (`map` / `mapa`)

| Função | Faz |
|---|---|
| `map.create()` / `mapa.criar()` | Cria um mapa vazio |
| `map.set(mapa, chave, v)` / `mapa.definir(...)` | Define um valor |
| `map.get(mapa, chave)` / `mapa.obter(...)` | Pega um valor |
| `map.remove(mapa, chave)` / `mapa.remover(...)` | Remove uma chave (dá erro `CB107` se não existir) |
| `map.contains(mapa, chave)` / `mapa.contem(...)` | Verifica se a chave existe |
| `map.keys(mapa)` / `mapa.chaves(...)` | Lista as chaves |
| `map.length(mapa)` / `mapa.tamanho(...)` | Quantidade de chaves |

Também é possível usar colchetes: `scores["Alex"]`.

## JSON (`json`)

| Função | Faz |
|---|---|
| `json.parse(texto)` / `json.ler(texto)` | Converte JSON em lista ou mapa |
| `json.stringify(valor)` / `json.escrever(valor)` | Converte um valor em JSON |
| `json.valid(texto)` / `json.validar(texto)` | Verifica se o texto é um JSON válido |

```text
variable user = json.parse('{"name":"Alex","items":["stone","wood"]}')
print(user["name"])               # Alex
print(json.stringify(user))       # {"items":["stone","wood"],"name":"Alex"}
```

Limites: até 1 MB e 32 níveis de aninhamento. Números muito grandes podem perder precisão, então guarde identificadores longos como texto.

## Entrada e saída (`io` / `entrada_saida`)

| Função | Faz |
|---|---|
| `io.read_text(pergunta)` / `io.ler_texto(...)` | Pede um texto ao usuário |
| `io.read_number(pergunta)` / `io.ler_numero(...)` | Pede um número ao usuário |
| `io.environment()` / `io.ambiente()` | Retorna o nome do ambiente de execução (na CLI, o texto `external`) |

## Tempo (`time` / `tempo`)

| Função | Faz |
|---|---|
| `time.now()` / `tempo.agora()` | Tempo decorrido desde o início do programa |
| `time.sleep(segundos)` / `tempo.esperar(...)` | Espera de 0 a 60 segundos |

## Eventos (`event` / `evento`)

| Função | Faz |
|---|---|
| `event.emit(nome)` / `evento.emitir(nome)` | Dispara um evento pelo nome |

## Arquivos (`file` / `arquivo`)

Exigem `permission file` no código e `--allow file` na execução. Veja [Segurança](seguranca.md).

| Função | Faz |
|---|---|
| `file.read(caminho)` / `arquivo.ler(...)` | Lê um arquivo como texto |
| `file.write(caminho, texto)` / `arquivo.escrever(...)` | Escreve um arquivo (substitui o conteúdo) |
| `file.append(caminho, texto)` / `arquivo.adicionar(...)` | Acrescenta texto ao final |
| `file.exists(caminho)` / `arquivo.existe(...)` | Verifica se o arquivo existe |
| `file.mkdir(caminho)` / `arquivo.criar_pasta(...)` | Cria uma pasta |
| `file.list(caminho)` / `arquivo.listar(...)` | Lista o conteúdo de uma pasta |

## Rede, janelas e Minecraft

Veja [Integrações](integracoes.md) para `net`/`rede` e `window`/`janela`, e [Minecraft](minecraft.md) para a biblioteca `minecraft`.
