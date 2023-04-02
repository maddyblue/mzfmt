// The module 'vscode' contains the VS Code extensibility API
// Import the module and reference it with the alias vscode in your code below
import * as vscode from "vscode";

class FmtImpl {
  instance: any;

  constructor(instance: any) {
    this.instance = instance;
  }
}
let impl: FmtImpl;

function prettySQL(text: string): string {
  if (!impl) {
    return text;
  }
  const width = vscode.workspace.getConfiguration().get('editor.wordWrapColumn', 80)
  return impl.instance.pretty_str(text, width);
}

// this method is called when your extension is activated
// your extension is activated the very first time the command is executed
export function activate(context: vscode.ExtensionContext) {
  __webpack_public_path__ =
    context.extensionUri.toString().replace("file:///", "") + "/dist/web/";

  require("mzfmt-wasm").then((rust: any) => {
    impl = new FmtImpl(rust);
  });

  let disposable;

  disposable = vscode.commands.registerCommand(
    "vscode-mzfmt-web.formatSQL",
    () => {
      var editor = vscode.window.activeTextEditor;
      if (!editor) {
        return;
      }
      var selection = editor.selection;
      const text = editor.document.getText(selection);
      editor.edit((builder) => {
        builder.replace(selection, prettySQL(text));
      });
    }
  );
  context.subscriptions.push(disposable);

  disposable = vscode.languages.registerDocumentFormattingEditProvider(
    { scheme: 'file', language: 'sql' }, {
    provideDocumentFormattingEdits(document: vscode.TextDocument): vscode.TextEdit[] {
      const doc = document.getText();
      const text = prettySQL(doc);
      var firstLine = document.lineAt(0);
      var lastLine = document.lineAt(document.lineCount - 1);
      var textRange = new vscode.Range(firstLine.range.start, lastLine.range.end);
      return [vscode.TextEdit.replace(textRange, text + '\n')];
    }
  });
  context.subscriptions.push(disposable);
}

// this method is called when your extension is deactivated
export function deactivate() { }
