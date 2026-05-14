# AI Client MCP Packaging Plan

1. [x] Reconcile `server.json`, `glama.json`, Smithery MCPB files, and MCP
   runtime docs against host-specific install claims.
2. [x] Add client-specific install and limitation docs for Claude, Codex,
   GitHub Copilot, and generic MCP clients.
3. [x] Keep policy tests aligned so host-plugin claims require matching package
   evidence.
4. [x] Run MCP transcript smoke, docs checks, and policy tests.
5. [x] Promote only the hosts with accepted listings or validated local package
   artifacts; Claude, Codex, Copilot, and generic MCP clients remain
   `prepared` configuration surfaces, not accepted host plugins.

## Implementation Notes

- Claude Desktop is documented as local MCP client configuration over stdio,
  not as a Claude plugin.
- Codex is documented as repo-agent CLI/MCP workflow guidance and local MCP
  configuration examples, not as a Codex plugin.
- GitHub Copilot remains repository coding-agent preparation through existing
  `.github/` instructions and setup workflow; entitlement and extension claims
  remain out of scope.
- Generic MCP clients are documented through protocol-level stdio snippets and
  transcript smoke expectations.
