# Registry Metadata Validation

Date: 2026-06-09  
Target release: `0.1.20`

## Validated Artifacts

| Artifact | Validation | Result |
| --- | --- | --- |
| `server.json` | Version matches `Cargo.toml`; OCI package `ghcr.io/edithatogo/sourceright-mcp:0.1.20`; schema `2025-12-11` | Pass |
| `glama.json` | Schema `https://glama.ai/mcp/schemas/server.json`; maintainer `edithatogo` | Pass |
| Smithery MCPB template | Manifest v0.3; stdio binary entry `mcp`; version `0.1.20` | Pass |
| OCI Dockerfile labels | MCP registry ownership labels present | Pass |
| Release workflow | Registry submission binds to release image version | Pass |

## Commands

```powershell
cargo +stable-x86_64-pc-windows-gnu test --locked --test mcp_distribution_checks --target-dir C:\tmp\sourceright-target-track73
cargo +stable-x86_64-pc-windows-gnu test --locked --test smithery_distribution_policy --target-dir C:\tmp\sourceright-target-track73
powershell -NoProfile -ExecutionPolicy Bypass -File scripts\verify-release-surface-refresh.ps1
```

## Result

All default-CI MCP distribution policy tests passed on 2026-06-09. Official MCP
Registry remains accepted for verified version `0.1.20` and requires refresh
evidence before future version claims.
