# Arquitetura

Esta página é voltada a quem quer contribuir com o compilador.

## Fluxo de processamento

```text
arquivo .cb → lexer → parser → análise semântica → IR tipada
                                                  ├→ interpretador (cb run)
                                                  └→ backend Minecraft (cb convert)
```

| Etapa | Responsabilidade |
|---|---|
| Lexer | Transforma o texto em tokens, com linha e coluna |
| Parser | Constrói a árvore de instruções e expressões, respeitando a precedência |
| Análise semântica | Resolve nomes e escopos, verifica tipos, constantes e permissões |
| IR tipada | Árvore final, com aliases normalizados para os nomes em inglês. Não é um formato estável. |
| Interpretador | Executa a IR |
| Backend Minecraft | Verifica o subconjunto suportado e gera o datapack em memória antes de gravar |

## Módulos

| Arquivo | Responsabilidade |
|---|---|
| `src/lexer.rs` | Texto, tokens, escapes e posições |
| `src/parser.rs` | Expressões e instruções |
| `src/semantic.rs` | Nomes, escopos, tipos e permissões |
| `src/catalog.rs` | Nomes canônicos, aliases, tipos, ambientes e permissões de cada função |
| `src/runtime.rs`, `src/value.rs` | Execução, valores e limites |
| `src/minecraft.rs` | Verificação e geração do datapack |
| `src/integrations.rs` | Contratos das integrações opcionais |
| `src/window.rs`, `src/net.rs` | Janelas e rede (Windows) |
| `src/json.rs` | Biblioteca JSON |
| `src/cli.rs` | Comandos, ajuda e geração de projetos |
| `src/diagnostic.rs` | Erros com código, localização e sugestão, em português e inglês |

## Adicionar uma função de biblioteca

1. Escolha o nome em inglês e um alias em português.
2. Registre em `src/catalog.rs` os argumentos, o retorno, os ambientes e a permissão.
3. Implemente em `src/runtime.rs` (ou no módulo opcional).
4. Escreva testes para o caso normal, para argumentos inválidos e para a permissão ausente.
5. Atualize a [Biblioteca padrão](../biblioteca/biblioteca-padrao.md).

## Adicionar um backend

Um backend recebe a IR já validada e deve:

- verificar tudo antes de gravar qualquer arquivo;
- recusar, com erro explícito, o que não consegue preservar com o mesmo comportamento;
- gerar a saída em memória antes de publicar o diretório.

Testes que comparam apenas texto gerado não provam o comportamento. Valide também no ambiente de destino.

## Diagnósticos

Todo erro usa um código `CBxxx` da [tabela de erros](../ferramentas/erros.md) e precisa de mensagens em português e em inglês. Sugestões devem ser específicas. Sugestões genéricas são um problema a ser corrigido.

## Princípios

- Prefira uma função de biblioteca a uma palavra-chave nova.
- Mantenha os aliases existentes funcionando.
- Não marque como pronto nada que não foi testado.
- Documente o estado real: funcionando, experimental ou não implementado.
