# Smithery MCPB Build Evidence

Date: 2026-05-18

## Artifact

- Release binary: `C:\tmp\sourceright-target-local\x86_64-pc-windows-gnu\release\sourceright.exe`
- MCPB package: `dist/sourceright-smithery-0.1.20-win32.mcpb`
- Platform: `win32`
- Version: `0.1.20`
- Size: `2889321` bytes

## Commands

```powershell
cargo +stable-x86_64-pc-windows-gnu build --release --target-dir C:\tmp\sourceright-target-local
powershell -NoProfile -ExecutionPolicy Bypass -File scripts\build-smithery-mcpb.ps1 -BinaryPath C:\tmp\sourceright-target-local\x86_64-pc-windows-gnu\release\sourceright.exe -Platform win32
C:\tmp\sourceright-target-local\x86_64-pc-windows-gnu\release\sourceright.exe mcp status --json
```

## Result

The MCPB build completed and emitted `sourceright.smithery_mcpb_build.v1`.
The generated manifest starts Sourceright through `bin/sourceright.exe mcp`,
sets `manifest_version` to `0.3`, and includes README plus both license files.

The release binary MCP status smoke returned:

- `server_mode`: `stdio`
- `transport`: `stdio`
- `server_started`: `false`
- `available_tools`: `14`
- `available_resources`: `8`
- `available_prompts`: `5`

## Boundary

This proves a local, release-derived Smithery MCPB package for Windows. It does
not prove Smithery registry publication, install through the Smithery UI, or an
accepted public listing.
