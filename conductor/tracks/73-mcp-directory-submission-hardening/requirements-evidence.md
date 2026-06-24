# MCP Directory Requirements Evidence

Date: 2026-06-09

| Surface | Source | Retrieved | Local impact |
| --- | --- | --- | --- |
| Official MCP Registry | <https://modelcontextprotocol.io/registry/about> and <https://modelcontextprotocol.io/registry/authentication> | 2026-06-09 | `server.json` must use standardized metadata, public install data, and `io.github.*` naming. OCI image labels must match release version. |
| Smithery | <https://smithery.ai/docs/build/publish> | 2026-06-09 | Stdio servers ship as prebuilt MCPB bundles. Bundle build, manifest validation, and local MCP status smoke are required before listing claims. |
| Glama | <https://glama.ai/mcp/servers> and repository `glama.json` schema | 2026-06-09 | Valid `glama.json`, public license metadata, and MCP discoverability from repo files are required locally. Accepted status needs listing or API evidence. |

Inventory mirror: `conductor/submission-requirements.json` surfaces
`official-mcp-registry`, `smithery`, and `glama`.
