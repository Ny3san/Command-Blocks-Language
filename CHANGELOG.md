# Changelog

Todas as mudanças notáveis deste projeto são documentadas aqui.
O formato segue [Keep a Changelog](https://keepachangelog.com/pt-BR/1.1.0/).

## [Não lançado]

### Removido
- Integrações que não funcionavam ou não podiam ser verificadas: Discord, ponte Minecraft, janelas nativas e rede HTTP. Os comandos `install`, `list`, `info` e o template `discord-bot` também saíram.
- Permissões `net`, `window`, `process` e `minecraft_bridge`. Permanece apenas `file`.
- Alias `data` (apontava para um módulo inexistente).
- Snippet e autocompletar correspondentes na extensão VS Code.

### Corrigido
- Span do erro de escape inválido (`CB001`) agora aponta para a barra invertida, não para a aspa de abertura.
- `mapa.remover` / `map.remove` implementado: remove a chave e retorna `CB107` quando ela não existe. O alias já estava mapeado, mas a função não existia.

### Removido
- Pasta duplicada `command-blocks-vscode/` (mantida `vscode-command-blocks/`).
- Artefato gerado `meu_datapack/` do controle de versão. Continua sendo gerado por `cb convert ... --output meu_datapack`.
- Script `publicar-command-blocks-github.bat` (continha caminho local).

### Documentação
- Documentação reorganizada em tutorial, referência da linguagem, biblioteca padrão, ferramentas, guias e projeto, com índice em `docs/README.md`.
- Removidas as versões em inglês (`docs/en/`) e os arquivos internos de marcos e verificação.
- Exemplos da documentação são verificados com `cb check`. Nomes de funções, aliases e códigos de erro foram conferidos contra o código.
- Exemplos e trechos de código da documentação usam nomes em inglês.

### Código
- Exemplos em `examples/` usam palavras-chave e nomes em inglês. As saídas continuam idênticas.
- Comentários dos testes em inglês.
- Corrigidos erros de `cargo fmt` e `clippy` no Linux que impediam o CI de passar.

### Interno
- Avisos do compilador zerados no Linux (`unused imports`, `dead_code`).
- `.gitignore` consolidado.
- Testes de regressão para escape, `mapa.remover` e guarda de alias de evento.
