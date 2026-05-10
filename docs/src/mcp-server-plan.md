# MCP Server Plan

`sourceright mcp` is currently a status surface. It does not start a server.

The next MCP increment should be read-only, local-file based, and contract-first.
The manifests under `mcp/` define the initial tools, resources, and prompts
without implying that transport is implemented.

Initial read-only tools:

- validate CSL JSON;
- generate a reference integrity report;
- list the review queue;
- reconcile manuscript citations;
- screen a journal submission;
- analyze legal citations;
- build a claim/source provenance report;
- evaluate deterministic style and recency policy checks;
- preview export artifacts.

Write-capable tools should wait until schema validation, audit logs, dry-run
semantics, and manual review workflows are stable.
