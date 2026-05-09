# CLI And MCP Plan

1. Document the initial Rust CLI surface: `init`, `validate-csl`, `report`, and placeholder `mcp`. Completed.
2. Define deterministic output, exit-code, and file-path contracts for `validate-csl`. Completed for text output, `--json`, and explicit exit codes.
3. Keep `mcp` documented as an explicit placeholder until server mode is wired to the Rust core, with `mcp status` available for readiness checks. Completed with read-only surface reporting.
4. Plan the later workflow commands: `extract`, `normalize`, `verify`, `review`, `export`, and `pipeline`. Completed in the CLI/MCP docs and test matrix.
5. Plan MCP tools/resources/prompts only as interfaces over stabilized Rust core behavior. Completed with read-only report and validation contracts exposed before server transport.
6. Add adapter package plans only after the CLI and MCP contracts stabilize. Deferred to later publishing/adapter work by design; the current CLI/MCP track is complete.
