# CLI And MCP Spec

## Goal

Expose Sourceright through a local Rust CLI first, then through an MCP server over the same Rust core once the pipeline contracts are stable.

## Initial CLI Interface

- `sourceright init`: create or confirm the local Sourceright workspace layout.
- `sourceright validate-csl`: validate canonical CSL JSON and return deterministic diagnostics.
- `sourceright report`: generate a reference integrity report from a Sourceright workspace.
- `sourceright mcp`: placeholder command that clearly reports MCP server mode is not ready yet.
- `sourceright mcp status`: status-only placeholder output that exits successfully for readiness checks.

## Planned CLI Interface

- `sourceright extract`: extract reference candidates from supported inputs.
- `sourceright normalize`: normalize reference records into canonical CSL JSON.
- `sourceright verify`: verify and enrich records through citation providers.
- `sourceright review`: manage unresolved records and review queues.
- `sourceright export`: write target formats such as RIS, ENW, BibLaTeX, XML, and YAML.
- `sourceright pipeline`: run the full extract-to-export workflow with deterministic artifacts.

## MCP Interface

Initial MCP behavior is a placeholder exposed through `sourceright mcp`. Plain `mcp` must not start or imply a server. It should print a conservative status and exit non-zero. `mcp status` may exit successfully because it only reports readiness. The first useful server increment should be read-only and should support CSL validation before workflow tools are enabled.

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
