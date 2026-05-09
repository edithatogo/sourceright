# MCP Plan

The initial Rust CLI includes `sourceright mcp` as a placeholder command. Until server mode is implemented, it fails clearly rather than pretending to expose tools.

The placeholder also provides a status-only path:

- `sourceright mcp` prints status and exits non-zero because no MCP server is started.
- `sourceright mcp status` prints status and exits successfully.
- `sourceright mcp --status` is an alias for `status`.

Current status output is intentionally conservative: server mode is `not-implemented`, transport is `none`, and available MCP tools/resources/prompts are all `0`. The output points users back to the implemented CLI commands that are useful today: `validate-csl` and `report`.

The planned MCP server should expose the same Rust core as the CLI once the reference pipeline is available.

Planned tools:

- `references.extract`
- `references.normalize`
- `references.verify`
- `references.review_queue`
- `references.export`
- `references.report`
- `pipeline.run`

Planned resources:

- Canonical CSL JSON.
- Validation diagnostics.
- Verification sidecar.
- Review queue.
- Citation reconciliation report.
- Reference integrity report.
- Export manifest.

Planned prompts:

- Manual reference review.
- CSL validation explanation.
- Conflict explanation.
- Provider comparison.

The first useful MCP increment should be read-only and local-file based: validate CSL JSON, generate reference integrity reports, report diagnostics, and describe the unavailable pipeline tools without side effects. Write-capable tools should wait until the CLI pipeline commands have stable input and output contracts.
