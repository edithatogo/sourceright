# Smithery MCPB Publish Evidence

Date: 2026-06-10

## Root cause (prior `400 No values to set`)

Smithery CLI `tee()` builds stdio deploy payloads from MCPB `manifest.json`. When the
bundle lacked embedded `tools` / `resources` / `prompts` and had no `user_config`,
the registry deploy API rejected the release with `400 {"error":"No values to set"}`.

## Fix

- `scripts/build-smithery-mcpb.ps1` now copies `mcp/server-card.json` surfaces into
  the generated manifest (`tools`, `resources`, `prompts`).
- `smithery/mcpb/manifest.template.json` adds optional `user_config.workspace_root`
  so Smithery extracts a `configSchema` for the listing UI.

## Commands

```powershell
cargo +stable-x86_64-pc-windows-gnu build --release --target-dir C:\tmp\sourceright-target-live
powershell -NoProfile -File scripts\build-smithery-mcpb.ps1 `
  -BinaryPath C:\tmp\sourceright-target-live\x86_64-pc-windows-gnu\release\sourceright.exe `
  -Platform win32
smithery mcp publish dist\sourceright-smithery-0.1.20-win32.mcpb -n edithatogo/sourceright
```

## Result

| Qualified name | Release ID | Status | MCP URL |
| --- | --- | --- | --- |
| `edithatogo/sourceright-mcpb-hardened` | `5df5e22b-4950-409c-b13a-a19a178203e3` | SUCCESS | `https://sourceright-mcpb-hardened--edithatogo.run.tools` |
| `edithatogo/sourceright` | `263ee636-5d24-4010-9dd9-e199d4f7b848` | SUCCESS | `https://sourceright--edithatogo.run.tools` |

Listing probe: `https://smithery.ai/servers/edithatogo/sourceright` → **200** (2026-06-10).

## Boundary

This proves Smithery stdio MCPB publication for the Windows bundle and a live registry
page for `edithatogo/sourceright`. It does not prove Glama indexing, Linux MCPB parity,
or GitHub Pages well-known serving (still blocked for dot paths).
