# CLI And MCP Spec

## Goal

Expose Sourceright through a local Rust CLI first, then through an MCP server over the same Rust core once the pipeline contracts are stable.

## Initial CLI Interface

- `sourceright init`: create or confirm the local Sourceright workspace layout.
- `sourceright validate-csl`: validate canonical CSL JSON and return deterministic diagnostics.
- `sourceright report`: generate a reference integrity report from a Sourceright workspace, with Markdown, JSON, and MCP-resource output modes.
- `sourceright mcp`: start the local stdio MCP server.
- `sourceright mcp status`: report MCP readiness, read-only resources, and
  apply-gated write surfaces without starting the server.

## `validate-csl` Contract

`sourceright validate-csl [--json] <references.csl.json>` validates a caller-supplied CSL JSON file path without rewriting the file.

Exit codes:

- `0`: the file is readable, parses as CSL JSON, and has no validation diagnostics.
- `1`: the file is readable and parses as CSL JSON, but validation diagnostics were found.
- `2`: usage errors, I/O errors, JSON parse errors, and other runtime failures.

Human-readable output:

- success prints `valid`.
- validation failure prints stable diagnostic lines as `<code> <path> <message>`.

Machine-readable output:

- `--json` prints compact JSON to stdout as `{"ok":bool,"path":string,"diagnostics":[{"code":string,"path":string,"message":string}]}`.
- `path` preserves the caller-supplied file path string.
- diagnostics preserve the Rust validator's deterministic traversal order.

## Planned CLI Interface

- `sourceright extract`: extract reference candidates from supported inputs.
- `sourceright normalize`: normalize reference records into canonical CSL JSON.
- `sourceright verify`: verify and enrich records through citation providers.
- `sourceright review`: manage unresolved records and review queues.
- `sourceright export`: write target formats such as RIS, ENW, BibLaTeX, XML, and YAML.
- `sourceright pipeline`: run the full extract-to-export workflow with deterministic artifacts.

## MCP Interface

MCP behavior is exposed through `sourceright mcp`. Plain `mcp` starts the
stdio server; `mcp status` exits successfully because it only reports readiness.
The Rust core and CLI expose read-only contracts for adapters plus explicit
apply-gated write tools for workspace initialization, review-decision import,
and export writes.

Implemented and planned tools:

- `references.review_queue`
- `references.validate_csl`
- `references.report`
- `references.citations`
- `journal.screen_submission`
- `exports.preview`
- `workspace.init`
- `review.import_decisions`
- `exports.write`
- planned later: `references.extract`, `references.normalize`,
  `references.verify`, and `pipeline.run`

Planned resources:

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
