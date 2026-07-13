# VS Code / Open VSX submission — 2026-06-10

| Artifact | Path |
| --- | --- |
| VSIX | `dist/edithatogo.sourceright-0.1.20.vsix` |
| Extension | `extensions/vscode-sourceright/` (init, report, validate-csl, journal-screen) |
| Publish script | `scripts/publish-host-submissions.ps1` (`VSCE_PAT`, `OVSX_PAT`) |

Build: `scripts/build-host-packages.ps1` — **pass**.

GitHub Release asset (2026-06-10): https://github.com/edithatogo/sourceright/releases/download/v0.1.20/edithatogo.sourceright-0.1.20.vsix

Marketplace/Open VSX publish requires publisher tokens; run `publish-host-submissions.ps1` when `VSCE_PAT` and `OVSX_PAT` are set.
