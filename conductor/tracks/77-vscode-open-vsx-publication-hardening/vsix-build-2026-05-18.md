# VSIX Build Evidence

Date: 2026-05-18

## Artifact

- Scaffold: `extensions/vscode-sourceright/`
- Package: `dist/edithatogo.sourceright-0.1.20.vsix`
- Smoke package: `dist/vscode-smoke/edithatogo.sourceright-0.1.20.vsix`
- Build script: `scripts/build-vscode-vsix.ps1`
- Smoke script: `scripts/smoke-vscode-vsix.ps1`

## Command

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File scripts\build-vscode-vsix.ps1
powershell -NoProfile -ExecutionPolicy Bypass -File scripts\build-vscode-vsix.ps1 -OutputDir dist\vscode-smoke
powershell -NoProfile -ExecutionPolicy Bypass -File scripts\smoke-vscode-vsix.ps1 -VsixPath dist\vscode-smoke\edithatogo.sourceright-0.1.20.vsix
```

## Result

The VSIX build completed and emitted
`sourceright.vscode_vsix_build.v1`. The package declares Workspace Trust support
and exposes only the read-only `sourceright report --json` command.

The isolated extension-host smoke completed and emitted
`sourceright.vscode_vsix_smoke.v1`.

- Extension: `edithatogo.sourceright`
- Extensions directory: `C:\tmp\sourceright-vscode-vsix-smoke\extensions`
- User-data directory: `C:\tmp\sourceright-vscode-vsix-smoke\user-data`
- Install smoke: `passed`
- Uninstall smoke: `passed`

## Boundary

This proves a local VSIX package scaffold plus isolated install/uninstall
smoke. Marketplace/Open VSX publication and accepted listing evidence remain
separate gates.
