# JSON

JSON permite trocar dados com arquivos e serviços. A biblioteca faz parte do núcleo e não exige instalação ou permissão. Ler um arquivo ainda exige a permissão `arquivo`.

```cb
variavel usuario = json.ler('{"nome":"Alex","idade":20}')
imprimir(usuario["nome"])
variavel serializado = json.escrever(usuario)
verificar(json.validar(serializado))
```

| Português | Inglês | Resultado |
|---|---|---|
| `json.ler(texto)` | `json.parse(text)` | Valor, lista ou mapa |
| `json.escrever(valor)` | `json.stringify(value)` | Texto JSON compacto |
| `json.validar(texto)` | `json.valid(text)` | Booleano |

`null` vira `nulo`, números inteiros de 32 bits usam o tipo inteiro e os demais números usam ponto flutuante de 64 bits. Números grandes podem perder precisão: mantenha identificadores como os do Discord em texto. Objetos viram mapas com chaves de texto; a saída ordena suas chaves.

O codec aceita até 1 MB e 32 níveis. Rejeita chaves duplicadas, comentários, vírgulas finais, escapes inválidos e números não finitos. Erros usam `CB107` e apontam a chamada no arquivo `.cb`; o conteúdo privado do JSON não é mostrado. Valores do tipo segredo são serializados como `[REDACTED]`. A biblioteca não interpreta código nem acessa a rede. A conversão de chamadas JSON para Minecraft ainda é incompatível.
