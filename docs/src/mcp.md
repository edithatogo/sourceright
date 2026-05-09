# MCP Plan

The initial Rust CLI includes `sourceright mcp` as a placeholder command. Until server mode is implemented, it should fail clearly rather than pretending to expose tools.

The planned MCP server should expose the same Rust core as the CLI once the reference pipeline is available.

Planned tools:

- `references.extract`
- `references.normalize`
- `references.verify`
- `references.review_queue`
- `references.export`
- `pipeline.run`

Planned resources:

- Canonical CSL JSON.
- Validation diagnostics.
- Verification sidecar.
- Review queue.
- Citation reconciliation report.
- Export manifest.

Planned prompts:

- Manual reference review.
- CSL validation explanation.
- Conflict explanation.
- Provider comparison.

The first useful MCP increment should be read-only and local-file based: validate CSL JSON, report diagnostics, and describe the unavailable pipeline tools without side effects. Write-capable tools should wait until the CLI pipeline commands have stable input and output contracts.
