const vscode = require('vscode');
const cp = require('child_process');

const items = [
  ["variable", "keyword", "Declares a variable"],
  ["variavel", "keyword", "Alias de variable \u2014 Declares a variable"],
  ["constant", "keyword", "Declares a constant"],
  ["constante", "keyword", "Alias de constant \u2014 Declares a constant"],
  ["function", "keyword", "Declares a function"],
  ["funcao", "keyword", "Alias de function \u2014 Declares a function"],
  ["event", "keyword", "Declares an event"],
  ["evento", "keyword", "Alias de event \u2014 Declares an event"],
  ["if", "keyword", "Conditional block"],
  ["se", "keyword", "Alias de if \u2014 Conditional block"],
  ["else", "keyword", "Alternative branch"],
  ["senao", "keyword", "Alias de else \u2014 Alternative branch"],
  ["while", "keyword", "Loop while a condition holds"],
  ["enquanto", "keyword", "Alias de while \u2014 Loop while a condition holds"],
  ["repeat", "keyword", "Repeats a block"],
  ["repetir", "keyword", "Alias de repeat \u2014 Repeats a block"],
  ["times", "keyword", "Number of repetitions"],
  ["vezes", "keyword", "Alias de times \u2014 Number of repetitions"],
  ["for", "keyword", "Loop over a collection"],
  ["para", "keyword", "Alias de for \u2014 Loop over a collection"],
  ["each", "keyword", "Used with for"],
  ["cada", "keyword", "Alias de each \u2014 Used with for"],
  ["in", "keyword", "Used with for"],
  ["em", "keyword", "Alias de in \u2014 Used with for"],
  ["return", "keyword", "Returns a value"],
  ["retornar", "keyword", "Alias de return \u2014 Returns a value"],
  ["import", "keyword", "Imports an optional library"],
  ["importar", "keyword", "Alias de import \u2014 Imports an optional library"],
  ["permission", "keyword", "Declares a permission"],
  ["permissao", "keyword", "Alias de permission \u2014 Declares a permission"],
  ["test", "keyword", "Declares a test"],
  ["teste", "keyword", "Alias de test \u2014 Declares a test"],
  ["and", "keyword", "Logical and"],
  ["e", "keyword", "Alias de and \u2014 Logical and"],
  ["or", "keyword", "Logical or"],
  ["ou", "keyword", "Alias de or \u2014 Logical or"],
  ["not", "keyword", "Logical not"],
  ["nao", "keyword", "Alias de not \u2014 Logical not"],
  ["true", "value", "Boolean true"],
  ["verdadeiro", "value", "Alias de true \u2014 Boolean true"],
  ["false", "value", "Boolean false"],
  ["falso", "value", "Alias de false \u2014 Boolean false"],
  ["null", "value", "Null value"],
  ["nulo", "value", "Alias de null \u2014 Null value"],
  ["print", "function", "Prints a value"],
  ["imprimir", "function", "Alias de print \u2014 Prints a value"],
  ["assert", "function", "Fails when the condition is false"],
  ["verificar", "function", "Alias de assert \u2014 Fails when the condition is false"],
  ["secret", "function", "Reads a secret from the environment"],
  ["segredo", "function", "Alias de secret \u2014 Reads a secret from the environment"],
  ["text", "module", "Text functions"],
  ["texto", "module", "Alias de text \u2014 Text functions"],
  ["math", "module", "Math functions"],
  ["matematica", "module", "Alias de math \u2014 Math functions"],
  ["list", "module", "List functions"],
  ["lista", "module", "Alias de list \u2014 List functions"],
  ["map", "module", "Map functions"],
  ["mapa", "module", "Alias de map \u2014 Map functions"],
  ["json", "module", "JSON library"],
  ["time", "module", "Time functions"],
  ["tempo", "module", "Alias de time \u2014 Time functions"],
  ["io", "module", "Input and output"],
  ["file", "module", "File functions (needs permission file)"],
  ["arquivo", "module", "Alias de file \u2014 File functions (needs permission file)"],
  ["minecraft", "module", "Minecraft datapack functions"]
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
