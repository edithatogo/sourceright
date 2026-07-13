# Smithery Install Smoke (2026-06-10)

## Local MCPB install smoke — passed

| Step | Command / artifact | Result |
| --- | --- | --- |
| MCP status (cargo install) | `sourceright mcp status --json` | 14 tools, 8 resources, 5 prompts |
| MCPB build | `pwsh scripts/build-smithery-mcpb.ps1 -BinaryPath ~/.cargo/bin/sourceright.exe -Platform win32` | `dist/sourceright-smithery-0.1.20-win32.mcpb` |
| MCPB extract + status | Expand archive → `bin/sourceright.exe mcp status --json` | Same surface counts as above |

## Registry listing — live

| Check | URL | Result |
| --- | --- | --- |
| Listing page | https://smithery.ai/servers/edithatogo/sourceright | **200** — Local transport, 14/8/5 surfaces |
| Directory probe | `scripts/azure/run-directory-probes.ps1` | smithery_listing **200** (2026-06-10) |

## Registry gateway install — passed (2026-06-10)

| Check | Result |
| --- | --- |
| `smithery mcp add edithatogo/sourceright --client cursor --config '{}'` (pipe `N` for optional config) | **success** — `qualifiedName: edithatogo/sourceright`, `transport: stdio` |
| Cursor config `~/.cursor/mcp.json` | `npx -y @smithery/cli@latest run edithatogo/sourceright` |
| Smithery bundle cache | `~/.smithery/cache/servers/edithatogo/sourceright/current/server.mcpb` (2.98 MB) |
| Running stdio server | `sourceright.exe` PID from cache path (EBUSY on re-run confirms active server) |

Hosted HTTP gateway URLs (`server.smithery.ai`, `run.tools`) return **404** for this
Local MCPB listing; install proof is via Smithery CLI bundle download + stdio launch.

## Claim boundary

Listing URL + MCPB release + registry gateway install smoke satisfy Track 73
`publicly_accepted` for Smithery on 2026-06-10. Linux MCPB parity remains optional.
