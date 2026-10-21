const vscode = require('vscode');
const cp = require('child_process');

const items = [
  ['variavel', 'keyword', 'Declara uma variável'], ['constante', 'keyword', 'Declara uma constante'],
  ['funcao', 'keyword', 'Declara uma função'], ['evento', 'keyword', 'Declara um evento'],
  ['se', 'keyword', 'Condição'], ['senao', 'keyword', 'Alternativa da condição'],
  ['enquanto', 'keyword', 'Loop enquanto'], ['repetir', 'keyword', 'Repetição'],
  ['vezes', 'keyword', 'Quantidade da repetição'], ['para', 'keyword', 'Loop para cada item'],
  ['cada', 'keyword', 'Iteração'], ['em', 'keyword', 'Membro da iteração'],
  ['retornar', 'keyword', 'Retorna um valor'], ['importar', 'keyword', 'Importa uma biblioteca'],
  ['permissao', 'keyword', 'Declara uma permissão'], ['teste', 'keyword', 'Declara um teste'],
  ['verdadeiro', 'value', 'Booleano verdadeiro'], ['falso', 'value', 'Booleano falso'], ['nulo', 'value', 'Valor nulo'],
  ['imprimir', 'function', 'Imprime texto'], ['ler', 'function', 'Lê entrada do usuário'],
  ['tamanho', 'function', 'Retorna o tamanho de um valor'], ['texto', 'function', 'Converte para texto'],
  ['numero', 'function', 'Converte para número'], ['lista', 'function', 'Cria uma lista'],
  ['mapa', 'function', 'Cria um mapa'], ['json', 'module', 'Biblioteca JSON'],
  ['tempo', 'module', 'Biblioteca de tempo'], ['arquivo', 'module', 'Biblioteca de arquivos'],
  ['rede', 'permission', 'Permissão de rede'], ['file', 'permission', 'Permissão de arquivos'],
  ['processo', 'permission', 'Permissão de processos'], ['window', 'permission', 'Permissão de janelas'],
  ['minecraft_bridge', 'permission', 'Permissão da ponte Minecraft'],
  ['discord', 'module', 'Biblioteca Discord'], ['minecraft', 'module', 'Biblioteca Minecraft']
];

function completionProvider() {
  return {
    provideCompletionItems(document, position) {
      const result = [];
      for (const [label, kind, detail] of items) {
        const item = new vscode.CompletionItem(label, vscode.CompletionItemKind[kind === 'value' ? 'Value' : kind === 'module' ? 'Module' : kind === 'permission' ? 'EnumMember' : kind === 'function' ? 'Function' : 'Keyword']);
        item.detail = `Command Blocks — ${detail}`;
        item.documentation = new vscode.MarkdownString(`**${label}**\\n\\n${detail}`);
        result.push(item);
      }
      return result;
    }
  };
}

function runCb(command, editor) {
  const executable = vscode.workspace.getConfiguration('commandBlocks').get('executable', 'cb');
  const file = editor.document.fileName;
  const terminal = vscode.window.createTerminal('Command Blocks');
  terminal.show(true);
  terminal.sendText(`${quote(executable)} ${command} ${quote(file)}`);
}
function quote(value) { return process.platform === 'win32' ? `"${String(value).replace(/"/g, '\\"')}"` : `'${String(value).replace(/'/g, "'\\''")}'`; }

function activate(context) {
  const selector = { language: 'command-blocks', scheme: 'file' };
  context.subscriptions.push(vscode.languages.registerCompletionItemProvider(selector, completionProvider(), '.', ':'));
  context.subscriptions.push(vscode.commands.registerCommand('command-blocks.check', () => {
    const editor = vscode.window.activeTextEditor; if (editor) runCb('check', editor);
  }));
  context.subscriptions.push(vscode.commands.registerCommand('command-blocks.run', () => {
    const editor = vscode.window.activeTextEditor; if (editor) runCb('run', editor);
  }));
}
function deactivate() {}
module.exports = { activate, deactivate };
