# MCP Plan

The initial Rust CLI includes `sourceright mcp` as a placeholder command. Until server mode is implemented, it fails clearly rather than pretending to expose tools.

The placeholder also provides a status-only path:

- `sourceright mcp` prints status and exits non-zero because no MCP server is started.
- `sourceright mcp status` prints status and exits successfully.
- `sourceright mcp --status` is an alias for `status`.
- `sourceright mcp status --json` and `sourceright mcp --json` print machine-readable readiness status.
- `sourceright mcp tools --json`, `sourceright mcp resources --json`, and `sourceright mcp prompts --json` print compact copies of the checked-in MCP manifests.

Current status output is intentionally conservative: server mode is `not-implemented`, transport is `none`, and no MCP server is started. It also reports the read-only surfaces already implemented in the Rust core and CLI so adapters can target the stable contracts without implying server transport is available.

The JSON status envelope includes `server_mode`, `transport`, `server_started`,
tool/resource/prompt counts, implemented read-only surfaces, resource URIs, and
a readiness message. It is intended for wrappers and agents that should not
parse the human-readable status text.

The manifest commands are intentionally read-only. They expose `mcp/tools.v1.json`,
`mcp/resources.v1.json`, and `mcp/prompts.v1.json` through the CLI so early
MCP adapters can bind to the declared contracts without requiring server
transport or direct repository file access.

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
