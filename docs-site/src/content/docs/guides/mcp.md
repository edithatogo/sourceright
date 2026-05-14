---
title: MCP
description: Read-only MCP server contracts, resources, prompts, and runtime expectations.
---

Sourceright's MCP surface is read-first and audit-friendly.

- Tools and resources expose read-only contracts unless an explicit write path
  is dry-run safe and auditable.
- Status and manifest commands remain available even when transport changes.
- Write-capable behavior must preserve schemas, provenance, and explicit apply
  semantics.
- `server.json` and `glama.json` describe registry and directory metadata for
  the packaged MCP surface.

## Legal Citation Connector Boundary

For legal workflows, Sourceright should be described as a citation audit and
enrichment connector. The MCP surface can extract candidate citations and
return jurisdiction/provider evidence, confidence, conflicts, and review
issues. It must not answer legal questions, predict outcomes, draft final legal
work product, or claim legal compliance. Missing or stale provider evidence is
a review issue, not a basis for a legal conclusion.
