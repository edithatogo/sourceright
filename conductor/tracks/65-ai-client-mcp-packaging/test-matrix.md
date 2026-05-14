# AI Client MCP Packaging Test Matrix

| Area | Required check |
| --- | --- |
| Generic MCP | `initialize`, `tools/list`, `resources/list`, and `prompts/list` transcript smoke passes against `sourceright mcp`. |
| Claude/Codex snippets | Client-specific snippets are documented or explicitly deferred. |
| Copilot boundary | Docs distinguish Copilot coding-agent prep from a Copilot extension/package. |
| Write safety | Write-capable tools stay dry-run first unless `apply: true` is supplied. |
| Registry wording | Official MCP Registry, Glama, and Smithery statuses remain separate. |
| Review | `$conductor-review` checks docs and release-status wording before promotion. |
