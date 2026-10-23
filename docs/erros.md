# Erros

Um erro mostra o arquivo, a linha, a coluna, um código e uma sugestão:

```text
programa.cb:2:10: CB101: Nome não declarado.
Sugestão: Revise a instrução e seus argumentos.
```

Use `--lang en` para ver as mensagens em inglês.

## Códigos

| Código | Significado |
|---|---|
| CB000 | Argumentos da linha de comando inválidos |
| CB001 | Erro de texto (por exemplo, escape inválido em uma string) |
| CB002 | Erro de sintaxe |
| CB009 | Erro de leitura ou escrita de arquivo |
| CB101 | Nome não declarado ou fora do escopo |
| CB102 | Função desconhecida ou não disponível |
| CB103 | Tipo incompatível |
| CB104 | Tentativa de alterar uma constante |
| CB105 | Número ou tipo de argumentos errado em uma função |
| CB106 | Programa grande demais ou aninhado demais |
| CB107 | Índice ou chave não encontrada |
| CB108 | Operação inválida (por exemplo, divisão por zero) |
| CB201 | Recurso incompatível com o ambiente ou com o backend |
| CB202 | Recurso opcional ausente ou não habilitado |
| CB203 | Versão do Minecraft não suportada |
| CB204 | Namespace inválido |
| CB205 | Destino já existe |
| CB301 | Permissão não declarada ou não autorizada |
| CB302 | Limite de execução atingido |
| CB303 | Segredo ausente no ambiente |
| CB304 | Erro de integração externa |
| CB401 | Falha de verificação interna |

## Como corrigir

A linha e a coluna apontam onde o problema foi encontrado. Em erros de sintaxe, confira parênteses, chaves e aspas perto desse ponto.
