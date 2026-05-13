# MCP read-only hardening plan

## Goal

Make the MCP surface demonstrable and safe.

## Immediate docs

Add:
```text
docs/src/mcp-readonly-server.md
docs/src/mcp-examples.md
docs/src/mcp-threat-model.md
mcp/examples/codex-config.json
mcp/examples/claude-desktop-config.json
mcp/examples/cursor-config.json
mcp/examples/transcript-reference-report.md
```

## Initial read-only resources/tools

Read-only only:
```text
references.validate_csl
references.report
references.review_queue
references.citations
references.journal_screen
references.legal_report
references.provenance_report
references.export_preview
plugins.status
bench.status
```

## Explicitly defer

Do not add write-capable MCP tools until:
- schemas are stable;
- dry-run behavior is universal;
- audit logs are written;
- explicit apply/confirmation is implemented;
- remote sync credentials are scoped;
- threat model is documented.
