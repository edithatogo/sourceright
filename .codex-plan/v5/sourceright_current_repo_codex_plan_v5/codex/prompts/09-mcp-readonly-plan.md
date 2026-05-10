Proceed with Slice 8: read-only MCP server contracts and plan.

First inspect current CLI/MCP implementation. If MCP is currently a status surface or placeholder, add contracts before implementing write tools.

Potential additive files:

```text
mcp/tools.v1.json
mcp/resources.v1.json
mcp/prompts.v1.json
docs/mcp-server-plan.md
```

Initial read-only tools/resources should cover:
- validate CSL
- generate/read reference report
- list review queue
- citation reconciliation report
- journal screening report
- legal citation report
- provenance report
- export preview

Rules:
- Read-only first.
- Local-file based first.
- No write tools until schemas, audit logs, dry-run semantics, and review workflows are stable.
- Do not require live provider calls.
- Do not add secrets or API keys.
