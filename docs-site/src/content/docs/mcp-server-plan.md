---
title: MCP server plan
description: The local MCP server transport, contract, and migration path.
---

`sourceright mcp` starts the local server runtime.

Read-only inspection commands stay separate from server startup. `sourceright
mcp status` and the manifest subcommands are for status and contract
inspection, not for mutating state or changing transport behavior.

The next MCP increment should stay local-file based, contract-first, and
auditable. The manifests under `mcp/` define the current tools, resources, and
prompts, including dry-run write tools that require explicit `apply: true`
before they mutate workspace files.

Current read-only tools:

- validate CSL JSON;
- generate a reference integrity report;
- list the review queue;
- reconcile manuscript citations;
- screen a journal submission;
- analyze legal citations;
- build a claim/source provenance report;
- evaluate deterministic style and recency policy checks;
- preview export artifacts with `exports.preview`;
- discover validated plugin manifests and execution gates.

Write-capable tools should remain dry-run by default, require explicit apply,
and keep audit logs alongside any workspace mutation.
