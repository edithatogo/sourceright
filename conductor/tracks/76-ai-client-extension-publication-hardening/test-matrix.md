| Scenario | Acceptance | Evidence | Default-CI or opt-in-live |
| --- | --- | --- | --- |
| Requirements search | Each AI client has official package/extension requirements or no-package decision. | Track requirements record | Default-CI |
| MCP transcript | Client can discover tools/resources/prompts and dry-run write plan. | `examples/mcp-clients/smoke-requests.jsonl` transcript | Opt-in-live |
| Extension package | Host-specific package validates where a package path exists. | Host package build log | Default-CI |
| No overclaim | Local config is not described as a Claude, Codex, Copilot, Gemini, or Qwen plugin. | AI client policy tests | Default-CI |
