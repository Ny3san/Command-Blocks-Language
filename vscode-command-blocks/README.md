# Command Blocks para VS Code

Extensão para arquivos `.cb`. Oferece:

- realce de sintaxe para palavras-chave (em inglês e em português), textos, números, comentários e funções;
- autocompletar de palavras-chave, funções e bibliotecas;
- snippets para estruturas comuns;
- comandos para verificar e executar o arquivo aberto.

## Instalar

Requer [Node.js](https://nodejs.org/) e o VS Code 1.85 ou superior.

Na pasta `vscode-command-blocks`, gere o pacote e instale:

```text
npx @vscode/vsce package
code --install-extension command-blocks-language-0.1.0.vsix
```

Para testar sem instalar, abra a pasta no VS Code e pressione `F5`. Uma nova janela (Extension Development Host) será aberta com a extensão ativa.

## Configurar o executável

Os comandos de verificação e execução usam o executável `cb`. Se ele não estiver no `PATH`, indique o caminho em `settings.json`:

```json
{
  "commandBlocks.executable": "C:\\caminho\\para\\cb.exe"
}
```

## Snippets

| Prefixo | Gera |
|---|---|
| `var` | `variable` |
| `const` | `constant` |
| `func` | `function` |
| `event` | `event` |
| `if` | `if` |
| `rep` | `repeat ... times` |
| `while` | `while` |
| `test` | `test` com `assert` |
| `imp` | `import` |
| `perm` | `permission` |
| `window` | Janela (experimental, Windows) |
