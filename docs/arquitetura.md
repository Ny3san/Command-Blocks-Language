# Arquitetura

Esta página é para quem quer contribuir com o compilador. Para usar a linguagem, veja [Primeiros passos](primeiros-passos.md).

## Fluxo

```text
arquivo .cb → lexer → parser → análise semântica → IR tipada
                                                  ├→ interpretador (cb run)
                                                  └→ backend Minecraft (cb convert)
```

- **Lexer** transforma o texto em tokens e guarda linha e coluna.
- **Parser** monta a árvore de instruções e expressões, com a precedência dos operadores.
- **Análise semântica** resolve nomes e escopos, verifica tipos, constantes e permissões.
- **IR tipada** é a árvore final, com os aliases já normalizados para o nome em inglês. Não é um formato estável.

## Arquivos principais

| Arquivo | Responsabilidade |
|---|---|
| `src/lexer.rs` | Texto, tokens, escapes e posições |
| `src/parser.rs` | Expressões e instruções |
| `src/semantic.rs` | Nomes, escopos, tipos e permissões |
| `src/catalog.rs` | Nomes em inglês, aliases em português, tipos e permissões de cada função |
| `src/runtime.rs` e `src/value.rs` | Execução, valores e limites |
| `src/minecraft.rs` | Verificação e geração do datapack |
| `src/integrations.rs` | Contratos das integrações opcionais |
| `src/cli.rs` | Comandos e ajuda |
| `src/window.rs`, `src/net.rs` | Janelas e rede (Windows) |
| `src/json.rs` | Biblioteca JSON |

## Adicionar uma função de biblioteca

1. Escolha o nome em inglês e um alias em português.
2. Registre em `src/catalog.rs`: argumentos, retorno, ambientes e permissão.
3. Implemente em `src/runtime.rs` (ou no módulo opcional).
4. Escreva testes para: caso normal, argumento inválido, permissão ausente.
5. Atualize [Bibliotecas](bibliotecas.md).

## Adicionar um backend

O backend recebe a IR já validada. Deve:

- verificar tudo antes de gravar qualquer arquivo;
- recusar o que não consegue manter com o mesmo comportamento;
- gerar a saída em memória antes de publicar o diretório.

Testes que comparam apenas texto gerado não provam o comportamento. Teste também no ambiente de destino.

## Diagnósticos

Erros usam os códigos `CBxxx` de [Erros](erros.md). Mensagens existem em português e inglês. Ao criar um erro novo, escreva as duas versões e uma sugestão real, nunca genérica.

## Princípios

- Prefira uma função de biblioteca a uma palavra-chave nova.
- Mantenha os aliases existentes funcionando.
- Não marque como “pronto” nada que não foi testado.
