# MCP Plan

The initial Rust CLI includes `sourceright mcp` as a placeholder command. Until server mode is implemented, it fails clearly rather than pretending to expose tools.

The placeholder also provides a status-only path:

- `sourceright mcp` prints status and exits non-zero because no MCP server is started.
- `sourceright mcp status` prints status and exits successfully.
- `sourceright mcp --status` is an alias for `status`.

Current status output is intentionally conservative: server mode is `not-implemented`, transport is `none`, and no MCP server is started. It also reports the read-only surfaces already implemented in the Rust core and CLI so adapters can target the stable contracts without implying server transport is available.

The planned MCP server should expose the same Rust core as the CLI once the reference pipeline is available. The first contracts now available for reuse are:

- `validate-csl --json`, which returns a deterministic local-file validation envelope with `ok`, `path`, and `diagnostics` fields and uses exit code `1` for validation findings.
- `report --json`, which returns `sourceright.reference_report.v1`.
- `report --mcp-resource`, which returns the report resource envelope at `sourceright://reports/reference-integrity`.

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

The first useful MCP increment should be read-only and local-file based: validate CSL JSON, generate reference integrity reports, report diagnostics, expose the derived review queue resource, and describe unavailable pipeline tools without side effects. Write-capable tools should wait until the CLI pipeline commands have stable input and output contracts.
