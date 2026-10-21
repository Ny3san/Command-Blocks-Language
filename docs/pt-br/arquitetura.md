# Arquitetura e contribuição técnica

[Índice](../../README.md) · [Gramática](../grammar.ebnf) · [Contribuir](../../CONTRIBUTING.md)

```text
Fonte .cb → Lexer → Parser → Análise semântica → IR tipada
                                                ├→ Interpretador externo
                                                └→ Backend Minecraft → Datapack
```

Rust constrói o executável. O usuário escreve apenas Command Blocks. A IR atual é uma árvore tipada: reutiliza a estrutura sintática, normaliza aliases e anota tipos, mantendo posições de origem. Não é bytecode nem um formato estável de intercâmbio.

## Responsabilidades

- `lexer.rs`: texto, tokens, escapes e posições.
- `parser.rs`: precedência, expressões e instruções.
- `semantic.rs`: nomes, escopos, constantes, assinaturas, permissões e IR.
- `catalog.rs`: nomes canônicos, aliases, tipos, ambiente e permissão de cada função.
- `runtime.rs` e `value.rs`: execução, valores, limites e bibliotecas.
- `minecraft.rs`: verificação do subconjunto, comandos e publicação do diretório.
- `integrations.rs`: contratos de integrações opcionais.
- `cli.rs`: comandos, exemplos gerados, testes, formatação e empacotamento.

## Adicionar uma biblioteca

Comece por uma função pequena: nome inglês, alias português, argumentos, retorno, ambientes e permissão. Registre em `catalog.rs`; implemente no runtime ou adaptador opcional; acrescente testes de sucesso, argumento inválido, permissão ausente e conversão incompatível. Atualize ambos os idiomas da documentação.

Bibliotecas nativas são compiladas com o projeto nesta versão. Não há SDK de plugins dinâmicos ou importação de módulos `.cb`. Isso é uma limitação publicada, não uma instrução para o usuário escrever em outra linguagem.

## Adicionar um backend

Receba uma `Program` validada, faça análise de capacidades e gere todos os arquivos em memória antes de publicar. Recuse o que não consegue preservar semanticamente, incluindo código morto. Mantenha versões em perfis explícitos. Teste o resultado no ambiente de destino; comparação de texto gerado é insuficiente para provar comportamento.

## Melhorar a simplicidade

Prefira biblioteca a palavra-chave nova. Mostre primeiro um programa linear curto; introduza funções, eventos e permissões somente quando necessários. Preserve aliases e programas existentes. Novos recursos devem ter estados claros: implementado, experimental ou proposto. Não marque uma operação sem implementação como sucesso.
