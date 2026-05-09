# CLI Plan

The initial Rust binary is planned around a small, stable command surface first:

- `sourceright init`
- `sourceright validate-csl`
- `sourceright report`
- `sourceright mcp`
- `sourceright mcp status`

`init` creates or confirms the local Sourceright workspace layout and prints the workspace path. `validate-csl` validates canonical CSL JSON input and returns deterministic diagnostics suitable for agents and CI. `validate-csl --json` emits a compact machine-readable envelope with `ok`, `path`, and `diagnostics` fields. `report` produces a reference integrity report that can identify AI-related citation-error signals without claiming authorship or intent.

`mcp` remains a placeholder entry point for the future local MCP server. Plain `sourceright mcp` prints the current MCP status but exits non-zero because it does not start a server. `sourceright mcp status` and `sourceright mcp --status` print the same honest status output and exit successfully for scripts that need to check readiness.

Each implemented command supports command-specific help:

- `sourceright init --help`
- `sourceright validate-csl --help`
- `sourceright report --help`
- `sourceright mcp --help`

The planned workflow command family remains:

- `sourceright extract`
- `sourceright normalize`
- `sourceright verify`
- `sourceright review`
- `sourceright export`
- `sourceright pipeline`

Commands that return structured data should support deterministic JSON output. Human-readable output can be useful for local use, but CI and agent workflows need stable machine-readable results, exit codes, and file paths. Until JSON output is added, the initial CLI keeps diagnostics line-oriented and rejects unexpected extra arguments with command-specific usage hints.

## `validate-csl` contract

Usage:

```text
sourceright validate-csl [--json] <references.csl.json>
```

Human-readable output remains line-oriented:

- valid CSL input prints `valid` to stdout and exits `0`.
- readable CSL input with validation diagnostics prints one stable diagnostic per line as `<code> <path> <message>` and exits `1`.
- usage errors, unreadable files, and JSON parse errors print an error to stderr and exit `2`.

Machine-readable output is enabled with `--json`:

```json
{"ok":false,"path":"references.csl.json","diagnostics":[{"code":"csl.title.empty","path":"$[0].title","message":"CSL item title must not be empty"}]}
```

The `path` field is the caller-supplied file path, preserved as the CLI received it. Diagnostic ordering is the validator's stable traversal order.
