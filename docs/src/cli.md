# CLI Plan

The initial Rust binary is planned around a small, stable command surface first:

- `sourceright init`
- `sourceright validate-csl`
- `sourceright report`
- `sourceright mcp`

`init` should create or confirm the local Sourceright workspace layout. `validate-csl` should validate canonical CSL JSON input and return deterministic diagnostics suitable for agents and CI. `report` should produce a reference integrity report that can identify AI-related citation-error signals without claiming authorship or intent. `mcp` is a placeholder entry point for the future local MCP server and should make that status explicit until server mode is implemented.

The planned workflow command family remains:

- `sourceright extract`
- `sourceright normalize`
- `sourceright verify`
- `sourceright review`
- `sourceright export`
- `sourceright pipeline`

Commands that return structured data should support deterministic JSON output. Human-readable output can be useful for local use, but CI and agent workflows need stable machine-readable results, exit codes, and file paths.
