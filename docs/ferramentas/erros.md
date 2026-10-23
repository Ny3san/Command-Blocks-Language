# Erros

## Formato

Todo erro indica o local e o código:

```text
program.cb:2:10: CB101: Name not declared.
Suggestion: Review the statement and its arguments.
```

- `arquivo:linha:coluna` aponta o início do problema.
- `CBxxx` é o código do erro. Use-o para pesquisar nesta página.
- A sugestão aparece na linha seguinte. Use `--lang en` para as mensagens em inglês.

## Códigos

| Código | Categoria | Quando ocorre |
|---|---|---|
| CB000 | Argumentos | Opção ou comando inválido na linha de comando |
| CB001 | Léxico | Escape inválido em texto, ou caractere não reconhecido |
| CB002 | Sintaxe | Estrutura não permitida pela gramática |
| CB009 | Arquivos | Falha de leitura ou escrita |
| CB101 | Nomes | Nome não declarado, fora do escopo ou declaração em lugar não permitido |
| CB102 | Funções | Função desconhecida ou não implementada |
| CB103 | Tipos | Tipos incompatíveis em atribuição, operação ou argumento |
| CB104 | Constantes | Tentativa de reatribuir uma constante |
| CB105 | Argumentos | Quantidade de argumentos incorreta |
| CB106 | Limites | Programa, valor ou aninhamento grande demais |
| CB107 | Índices | Índice ou chave inexistente |
| CB108 | Operações | Operação inválida, como divisão por zero |
| CB201 | Ambiente | Recurso incompatível com o ambiente ou com o destino |
| CB202 | Integrações | Biblioteca ausente, não habilitada ou sem transporte implementado |
| CB203 | Versão | Versão do Minecraft não suportada |
| CB204 | Namespace | Namespace inválido |
| CB205 | Destino | O destino já existe |
| CB301 | Permissões | Permissão não declarada no programa ou não autorizada na CLI |
| CB302 | Limites de execução | Limite de passos, recursão ou tamanho atingido |
| CB303 | Segredos | Variável de ambiente exigida por `secret` não existe |
| CB304 | Integrações | Falha de comunicação com um serviço externo |
| CB401 | Verificação | Falha de `assert` ou de verificação interna |

## Erros conhecidos

- `math.sqrt` e outras funções com entrada fora do domínio geram `CB302`, que é o código de limite. Um código específico de domínio será criado.

## Como resolver

- **CB002:** confira parênteses, chaves e aspas perto do ponto indicado.
- **CB101 e CB102:** confira a grafia e se a declaração vem antes do uso.
- **CB103:** confira os tipos dos valores envolvidos. Use `text.string` ou `text.number` para converter.
- **CB301:** declare a permissão no código e autorize com `--allow`.
- **CB303:** defina a variável de ambiente indicada antes de executar.
