# CLI And MCP Spec

## Goal

Expose the reference verification pipeline through a local CLI and an MCP server.

## CLI Interface

- `sourceright init`
- `sourceright extract`
- `sourceright normalize`
- `sourceright verify`
- `sourceright review`
- `sourceright export`
- `sourceright pipeline`
- `sourceright mcp`

## MCP Interface

- Tools: `references.extract`, `references.verify`, `references.review_queue`, `references.export`.
- Resources: canonical CSL JSON, verification sidecar, review queue, reconciliation report, and export manifest.
- Prompts: manual review, conflict explanation, and provider comparison.
