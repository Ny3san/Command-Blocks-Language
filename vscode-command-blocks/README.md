# Command Blocks Language para VS Code

Extensão local para arquivos `.cb`: syntax highlighting, snippets, autocompletar de palavras-chave/bibliotecas e comandos para verificar/executar o arquivo.

## Instalação rápida no Windows

Na pasta desta extensão, execute:

```powershell
code --install-extension .\command-blocks-language-0.1.0.vsix
```

Se o `.vsix` ainda não existir, use o comando de empacotamento indicado pelo assistente ou pressione `F5` no VS Code para testar em uma janela Extension Development Host.

O executável pode ser configurado em `settings.json`:

```json
{
  "commandBlocks.executable": "D:\\pinto\\target\\release\\cb.exe"
}
```
