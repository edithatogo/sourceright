# Host Plugins Publish Runbook

Date: 2026-06-10

## Already on GitHub Release v0.1.20

| Asset | Install |
| --- | --- |
| `edithatogo.sourceright-0.1.20.vsix` | VS Code → Install from VSIX |
| `sourceright-claude-desktop-0.1.20-win32.mcpb` | Claude Desktop MCPB path |
| `edithatogo-sourceright-*-0.1.20.tgz` | `npm install` from release URL or git path |

Release: https://github.com/edithatogo/sourceright/releases/tag/v0.1.20

## Marketplace (tokens required)

```powershell
$env:VSCE_PAT = "<vscode-marketplace-pat>"
$env:OVSX_PAT = "<open-vsx-pat>"
powershell -File scripts/publish-host-submissions.ps1
```

## npm registry (optional)

```powershell
$env:NPM_TOKEN = "<npm-token>"
cd extensions/qwen-sourceright && npm publish --access public
cd packages/codex-sourceright-mcp && npm publish --access public
cd extensions/opencode-sourceright && npm publish --access public
```

## Git-based extension install

| Host | Command |
| --- | --- |
| Gemini CLI | `gemini extensions link extensions/gemini-sourceright` (from clone) |
| Qwen CLI | `qwen extensions install extensions/qwen-sourceright` |
| OpenCode | merge `extensions/opencode-sourceright/opencode.example.json` |

## Cline

Submitted: https://github.com/cline/mcp-marketplace/issues/1764 (open, awaiting review).
