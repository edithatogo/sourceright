# Track 77 — VS Code / Open VSX Requirements Evidence

Date: 2026-06-09

## Inventory alignment

| Surface | Inventory id | Submission packet |
| --- | --- | --- |
| VS Code Marketplace | `vscode-open-vsx` | `conductor/submission-packets/vscode-open-vsx.md` |
| Open VSX | `vscode-open-vsx` | same |

## Official requirements sources (searched 2026-05-18)

| Surface | Source |
| --- | --- |
| VS Code Marketplace | [Publishing Extensions](https://code.visualstudio.com/api/working-with-extensions/publishing-extension) — `vsce` packaging, metadata, publisher credentials |
| Open VSX | [Publishing Extensions](https://github.com/eclipse/openvsx/wiki/Publishing-Extensions) — separate registry steps from Marketplace |

## Local package contract

Frozen in Track 66/77 and `extensions/vscode-sourceright/`:

| Gate | Evidence |
| --- | --- |
| Thin VSIX scaffold calling CLI diagnostics | `extension.js` invokes `sourceright report --json` only |
| Workspace Trust | `package.json` `untrustedWorkspaces` support |
| Preview-first writes | No export/apply commands in extension surface |
| VSIX build | `scripts/build-vscode-vsix.ps1` → `dist/*.vsix` |
| Install/uninstall smoke | `scripts/smoke-vscode-vsix.ps1` with isolated extension host dirs |
| Listing metadata draft | `marketplace-metadata-draft.md` |

## Claim boundary

A local VSIX plus isolated install/uninstall smoke is **not** a VS Code
Marketplace or Open VSX **accepted** listing. Public acceptance requires listing
URL, version, and approval-gated external submission.
