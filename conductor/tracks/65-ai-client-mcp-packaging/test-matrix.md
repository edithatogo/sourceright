# AI Client MCP Packaging Test Matrix

| Area | Required check |
| --- | --- |
| Generic MCP | `initialize`, `tools/list`, `resources/list`, and `prompts/list` transcript smoke passes against `sourceright mcp`. |
| Claude/Codex snippets | Client-specific snippets are documented in `docs/src/mcp.md` and `examples/mcp-clients/` or explicitly deferred. |
| Copilot boundary | Docs distinguish Copilot coding-agent prep from a Copilot extension/package and from MCP client packaging. |
| Write safety | Write-capable tools stay dry-run first unless `apply: true` is supplied. |
| Registry wording | Official MCP Registry, Glama, and Smithery statuses remain separate. |
| Review | `$conductor-review` checks docs and release-status wording before promotion. |

## Current Evidence

| Area | Evidence |
| --- | --- |
| Generic MCP | `docs/src/mcp.md` and `examples/mcp-clients/generic-stdio.json` include stdio launch and discovery smoke expectations. |
| Claude Desktop | `docs/src/mcp.md` and `examples/mcp-clients/claude-desktop.json` document `mcpServers.sourceright` local stdio configuration. |
| Codex | `docs/src/mcp.md`, `examples/mcp-clients/codex-config.toml`, and `examples/mcp-clients/codex-mcp.json` document local CLI/MCP workflow configuration. |
| GitHub Copilot | `docs/src/mcp.md` and `examples/mcp-clients/github-copilot-coding-agent.md` point to coding-agent prep only and do not claim a Copilot extension. |
| Write safety | `docs/src/mcp.md` records dry-run `workspace.init` proof expectations and keeps `apply: true` as an explicit mutation boundary. |
| Registry wording | `docs/src/release-status.md` separates Official MCP Registry accepted status from Glama, Smithery, Claude Desktop, Codex, generic MCP clients, and GitHub Copilot prepared states. |
