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

## Client packaging status

| Client or directory | Current status | Claim boundary |
| --- | --- | --- |
| Official MCP Registry | Accepted for `0.1.20` through `server.json` and the OCI image target. | Registry acceptance does not prove every downstream client configuration. |
| Glama | Prepared through `glama.json`; no accepted listing is recorded. | Do not claim Glama availability until the listing is verified. |
| Smithery | Prepared through the MCPB/local stdio package path. | Do not claim Smithery availability until a concrete bundle/listing is verified. |
| Claude Desktop | Uses the generic local stdio pattern unless a client-specific guide is added. | This is client configuration, not a Claude plugin package. |
| Codex | Uses CLI/MCP workflow guidance unless a Codex-specific package is added. | This is repo-agent or MCP configuration, not a Codex plugin package. |
| GitHub Copilot | Repository coding-agent prep exists separately from MCP. | This is not a Copilot extension or marketplace package. |

## Local stdio snippets

### Generic MCP clients

```json
{
  "mcpServers": {
    "sourceright": {
      "command": "sourceright",
      "args": ["mcp"]
    }
  }
}
```

### Claude Desktop

Claude Desktop uses client configuration over local stdio. This is not a Claude
plugin package.

```json
{
  "mcpServers": {
    "sourceright": {
      "command": "sourceright",
      "args": ["mcp"]
    }
  }
}
```

### Codex

Codex workflows use the same local stdio server from the repository or installed
CLI path. This is not a Codex plugin package.

```json
{
  "mcpServers": {
    "sourceright": {
      "command": "sourceright",
      "args": ["mcp"]
    }
  }
}
```

### GitHub Copilot

GitHub Copilot support is repository coding-agent preparation, not an MCP client
package or Copilot extension.

```text
.github/copilot-instructions.md
.github/workflows/copilot-setup-steps.yml
.github/ISSUE_TEMPLATE/copilot_security_remediation.yml
```

## Legal Citation Connector Boundary

For legal workflows, Sourceright should be described as a citation audit and
enrichment connector. The MCP surface can extract candidate citations and
return jurisdiction/provider evidence, confidence, conflicts, and review
issues. It must not answer legal questions, predict outcomes, draft final legal
work product, or claim legal compliance. Missing or stale provider evidence is
a review issue, not a basis for a legal conclusion.
