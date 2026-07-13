# VSIX Install/Uninstall Smoke (Refresh)

Date: 2026-06-09

## Artifact

- VSIX: `dist/vscode-smoke/edithatogo.sourceright-0.1.20.vsix`
- Smoke script: `scripts/smoke-vscode-vsix.ps1`

## Command

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File scripts\build-vscode-vsix.ps1 -OutputDir dist\vscode-smoke
powershell -NoProfile -ExecutionPolicy Bypass -File scripts\smoke-vscode-vsix.ps1 -VsixPath dist\vscode-smoke\edithatogo.sourceright-0.1.20.vsix
```

## Result

```json
{
    "schema_version": "sourceright.vscode_vsix_smoke.v1",
    "vsix": "C:\\Users\\60217257\\OneDrive - Flinders\\repos\\sourceright\\dist\\vscode-smoke\\edithatogo.sourceright-0.1.20.vsix",
    "code_command": "code.cmd",
    "extensions_dir": "C:\\tmp\\sourceright-vscode-vsix-smoke\\extensions",
    "user_data_dir": "C:\\tmp\\sourceright-vscode-vsix-smoke\\user-data",
    "installed_extension": "edithatogo.sourceright",
    "install_smoke": "passed",
    "uninstall_smoke": "passed"
}
```

## Boundary

This refresh re-proves isolated install/uninstall smoke on 2026-06-09. It does
not submit to VS Code Marketplace or Open VSX and does not claim an accepted
public listing.
