# CLI Plan

The initial Rust binary is planned around a small, stable command surface first:

- `sourceright init`
- `sourceright validate-csl`
- `sourceright report`
- `sourceright export`
- `sourceright mcp`
- `sourceright mcp status`

`init` creates or confirms the local Sourceright workspace layout and prints the workspace path. `validate-csl` validates canonical CSL JSON input and returns deterministic diagnostics suitable for agents and CI. `validate-csl --json` emits a compact machine-readable envelope with `ok`, `path`, and `diagnostics` fields. `report` produces a reference integrity report that can identify AI-related citation-error signals without claiming authorship or intent; `report --json` and `report --mcp-resource` expose the same report through machine-readable envelopes. `export` writes clean XML, ENW, RIS, BibLaTeX, and YAML outputs from the workspace CSL file.

`mcp` remains a placeholder entry point for the future local MCP server. Plain `sourceright mcp` prints the current MCP status but exits non-zero because it does not start a server. `sourceright mcp status` and `sourceright mcp --status` print the same honest status output and exit successfully for scripts that need to check readiness.

Each implemented command supports command-specific help:

- `sourceright init --help`
- `sourceright validate-csl --help`
- `sourceright report --help`
- `sourceright export --help`
- `sourceright mcp --help`

The planned workflow command family remains:

- `sourceright extract`
- `sourceright normalize`
- `sourceright verify`
- `sourceright review`
- `sourceright pipeline`

Commands that return structured data support deterministic JSON for the implemented surfaces. Human-readable output remains useful for local use, while CI and agent workflows can depend on stable machine-readable results, exit codes, and file paths. The CLI rejects unexpected extra arguments with command-specific usage hints.

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

## `report` contract

Usage:

```text
sourceright report [--json|--mcp-resource] [.sourceright-directory]
```

Default Markdown output renders the editor-facing audit report. `--json` emits compact `sourceright.reference_report.v1` JSON with summary counters and stable issue records. `--mcp-resource` wraps the JSON report as an MCP-ready resource envelope at `sourceright://reports/reference-integrity`.

## `export` contract

Usage:

```text
sourceright export [--all|--format <format>] [.sourceright-directory]
```

The default is the full export suite. Supported format names are `yaml`, `xml`, `ris`, `enw`, and `biblatex`. The command writes deterministic files into the workspace `exports` directory and prints the written paths.
