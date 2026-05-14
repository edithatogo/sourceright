# AI Client MCP Packaging Plan

1. Reconcile `server.json`, `glama.json`, Smithery MCPB files, and MCP runtime
   docs against host-specific install claims.
2. Add client-specific install and limitation docs for Claude, Codex, GitHub
   Copilot, and generic MCP clients.
3. Add policy tests that prevent host-plugin claims without matching package
   evidence.
4. Run MCP transcript smoke, docs checks, and policy tests.
5. Promote only the hosts with accepted listings or validated local package
   artifacts.
