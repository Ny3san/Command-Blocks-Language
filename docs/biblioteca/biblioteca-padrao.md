# Biblioteca padrão

As funções são organizadas em módulos. Cada função tem um nome em inglês e, quando existe, um alias em português. Os dois nomes são equivalentes.

```text
print(text.upper("hello"))        # HELLO
imprimir(texto.maiusculas("olá")) # OLÁ
```

Os argumentos aparecem na ordem indicada. Quando um argumento tem tipo específico, ele é verificado: um tipo errado gera `CB103`, e uma quantidade errada de argumentos gera `CB105`.

## `print` e `assert`

| Função | Descrição |
|---|---|
| `print(value)` (`imprimir`) | Exibe um valor no terminal |
| `assert(condition)` (`verificar`) | Falha com `CB401` se a condição for falsa. Usada em testes. |

## `text` (`texto`)

| Função | Descrição |
|---|---|
| `text.upper(t)` (`texto.maiusculas`) | Converte para maiúsculas |
| `text.lower(t)` (`texto.minusculas`) | Converte para minúsculas |
| `text.trim(t)` (`texto.aparar`) | Remove espaços no início e no fim |
| `text.join(list, sep)` (`texto.juntar`) | Junta uma lista de textos com um separador |
| `text.split(t, sep)` (`texto.separar`) | Divide um texto em uma lista |
| `text.replace(t, from, to)` (`texto.substituir`) | Substitui todas as ocorrências |
| `text.contains(t, part)` (`texto.contem`) | Verifica se contém o trecho |
| `text.starts_with(t, part)` (`texto.comeca_com`) | Verifica o início |
| `text.ends_with(t, part)` (`texto.termina_com`) | Verifica o fim |
| `text.length(t)` (`texto.tamanho`) | Número de caracteres |
| `text.string(v)` (`texto.converter`) | Converte um valor em texto |
| `text.number(t)` (`texto.numero`) | Converte um texto em número |
| `text.format(t, map)` (`texto.formatar`) | Substitui `{chave}` pelos valores do mapa. Não executa código. |

```text
print(text.format("Players: {count}", {"count": 2}))   # Players: 2
```

## `math` (`matematica`)

| Função | Descrição |
|---|---|
| `math.abs(x)` (`matematica.absoluto`) | Valor absoluto |
| `math.round(x)` (`matematica.arredondar`) | Arredonda; empates vão para longe de zero |
| `math.sqrt(x)` (`matematica.raiz`) | Raiz quadrada |
| `math.sin(x)`, `math.cos(x)`, `math.tan(x)` | Funções trigonométricas (`seno`, `cosseno`, `tangente`) |
| `math.degrees(x)` (`matematica.graus`) | Radianos para graus |
| `math.radians(x)` (`matematica.radianos`) | Graus para radianos |
| `math.min(a, b)` (`matematica.minimo`) | Menor valor |
| `math.max(a, b)` (`matematica.maximo`) | Maior valor |
| `math.pow(a, b)` (`matematica.potencia`) | Potência |
| `math.random(min, max)` (`matematica.aleatorio`) | Inteiro entre `min` e `max`, incluindo os dois |

Funções que não são `random` devolvem `Number`. `math.random` **não é adequado para segredos, senhas ou tokens**.

Entrada fora do domínio de uma função, como `math.sqrt(-1)`, gera erro. Atualmente esse erro usa o código `CB302`, de limite, o que é uma inconsistência conhecida. Veja [Roadmap](../projeto/roadmap.md).

## `list` (`lista`)

| Função | Descrição |
|---|---|
| `list.create()` (`lista.criar`) | Cria uma lista vazia |
| `list.add(list, v)` (`lista.adicionar`) | Adiciona no fim |
| `list.remove(list, v)` (`lista.remover`) | Remove a primeira ocorrência. Gera erro se o valor não existir. |
| `list.get(list, i)` (`lista.obter`) | Devolve o item na posição `i` |
| `list.contains(list, v)` (`lista.contem`) | Verifica se o valor está na lista |
| `list.sort(list)` (`lista.ordenar`) | Devolve uma nova lista ordenada. Aceita listas só de números ou só de textos. |
| `list.length(list)` (`lista.tamanho`) | Número de itens |

## `map` (`mapa`)

| Função | Descrição |
|---|---|
| `map.create()` (`mapa.criar`) | Cria um mapa vazio |
| `map.set(map, key, v)` (`mapa.definir`) | Define o valor de uma chave |
| `map.get(map, key)` (`mapa.obter`) | Devolve o valor de uma chave. Gera erro se a chave não existir. |
| `map.remove(map, key)` (`mapa.remover`) | Remove uma chave. Gera `CB107` se ela não existir. |
| `map.contains(map, key)` (`mapa.contem`) | Verifica se a chave existe |
| `map.keys(map)` (`mapa.chaves`) | Lista as chaves |
| `map.length(map)` (`mapa.tamanho`) | Número de chaves |

Também é possível usar colchetes: `scores["Alex"]`.

## `json`

Disponível sem instalação e sem permissão. Ler arquivos ainda exige `permission file`.

| Função | Descrição |
|---|---|
| `json.parse(text)` (`json.ler`) | Converte JSON em valores (listas, mapas, textos, números, booleanos ou `null`) |
| `json.stringify(value)` (`json.escrever`) | Converte um valor em JSON compacto |
| `json.valid(text)` (`json.validar`) | Retorna `true` se o texto for um JSON válido |

```text
variable user = json.parse('{"name":"Alex","items":["stone","wood"]}')
print(user["name"])                 # Alex
print(json.stringify(user))         # {"items":["stone","wood"],"name":"Alex"}
```

Limites: 1 MB de entrada e 32 níveis de aninhamento. Chaves duplicadas, comentários, vírgulas finais e números não finitos são rejeitados. Números inteiros de 32 bits usam `Int`; os demais usam `Number`. Números muito grandes podem perder precisão, então guarde identificadores longos como texto.

## `io`

| Função | Descrição |
|---|---|
| `io.read_text(prompt)` (`io.ler_texto`) | Lê um texto do usuário, exibindo a pergunta |
| `io.read_number(prompt)` (`io.ler_numero`) | Lê um número do usuário |
| `io.environment()` (`io.ambiente`) | Devolve o nome do ambiente de execução (na CLI, `external`) |

## `time` (`tempo`)

| Função | Descrição |
|---|---|
| `time.now()` (`tempo.agora`) | Tempo decorrido desde o início do programa, em segundos |
| `time.sleep(seconds)` (`tempo.esperar`) | Espera entre 0 e 60 segundos |

## `event` (`evento`)

| Função | Descrição |
|---|---|
| `event.emit(name)` (`evento.emitir`) | Dispara os eventos com o nome indicado |

## `file` (`arquivo`)

Exige `permission file` e `--allow file`. Veja [Permissões e segurança](../guias/seguranca.md).

| Função | Descrição |
|---|---|
| `file.read(path)` (`arquivo.ler`) | Lê o conteúdo como texto |
| `file.write(path, text)` (`arquivo.escrever`) | Escreve o arquivo, substituindo o conteúdo anterior |
| `file.append(path, text)` (`arquivo.adicionar`) | Acrescenta texto ao final |
| `file.exists(path)` (`arquivo.existe`) | Verifica se o caminho existe |
| `file.mkdir(path)` (`arquivo.criar_pasta`) | Cria uma pasta |
| `file.list(path)` (`arquivo.listar`) | Lista os itens de uma pasta |
