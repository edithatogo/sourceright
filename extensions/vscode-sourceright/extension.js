const vscode = require('vscode');
const cp = require('child_process');
const path = require('path');

function getCliPath() {
  return vscode.workspace.getConfiguration('sourceright').get('cliPath', 'sourceright');
}

function getWorkspaceRoot() {
  const folder = vscode.workspace.workspaceFolders?.[0];
  if (!folder) {
    vscode.window.showWarningMessage('Open a workspace before running Sourceright.');
    return null;
  }
  return folder.uri.fsPath;
}

function runCliJson(args, label, options = {}) {
  const workspace = getWorkspaceRoot();
  if (!workspace) {
    return;
  }

  const cliPath = getCliPath();
  const cwd = options.cwd || workspace;
  const fullArgs = options.appendWorkspace === false ? [...args] : [...args, workspace];

  vscode.window.withProgress(
    { location: vscode.ProgressLocation.Notification, title: label },
    () =>
      new Promise((resolve) => {
        cp.execFile(cliPath, fullArgs, { cwd, maxBuffer: 10 * 1024 * 1024 }, (err, stdout, stderr) => {
          if (err) {
            const detail = (stderr || err.message || '').trim();
            vscode.window.showErrorMessage(`Sourceright failed: ${detail || err.message}`);
            resolve();
            return;
          }
          const doc = vscode.workspace.openTextDocument({
            content: stdout.trim(),
            language: 'json',
          });
          doc.then((d) => vscode.window.showTextDocument(d, { preview: true }));
          resolve();
        });
      }),
  );
}

function runCliInTerminal(args) {
  const workspace = getWorkspaceRoot();
  if (!workspace) {
    return;
  }
  const cliPath = getCliPath();
  const quoted = args.map((a) => (a.includes(' ') ? `"${a}"` : a)).join(' ');
  const terminal = vscode.window.createTerminal('Sourceright');
  terminal.show();
  terminal.sendText(`cd "${workspace}" && "${cliPath}" ${quoted}`);
}

function activate(context) {
  const commands = [
    ['sourceright.init', ['init'], 'Sourceright: Init Workspace', () => runCliInTerminal(['init'])],
    [
      'sourceright.report',
      ['report', '--json'],
      'Sourceright: Reference Report (JSON)',
      (args) => runCliJson(args, 'Sourceright report'),
    ],
    [
      'sourceright.validateCsl',
      [],
      'Sourceright: Validate CSL File',
      async () => {
        const pick = await vscode.window.showOpenDialog({
          canSelectMany: false,
          filters: { JSON: ['json'] },
        });
        if (!pick?.[0]) {
          return;
        }
        runCliJson(
          ['validate-csl', '--json', pick[0].fsPath],
          'Sourceright validate-csl',
          { appendWorkspace: false },
        );
      },
    ],
    [
      'sourceright.journalScreen',
      ['journal-screen', '--platform', 'ojs', '.sourceright'],
      'Sourceright: Journal Screen (OJS)',
      (args) => runCliJson(args, 'Sourceright journal-screen', { appendWorkspace: false }),
    ],
  ];

  for (const [id, baseArgs, title, handler] of commands) {
    context.subscriptions.push(
      vscode.commands.registerCommand(id, () => handler(baseArgs)),
    );
  }
}

function deactivate() {}

module.exports = { activate, deactivate };
