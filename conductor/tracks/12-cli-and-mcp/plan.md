# CLI And MCP Plan

1. Document the initial Rust CLI surface: `init`, `validate-csl`, and placeholder `mcp`.
2. Define deterministic output, exit-code, and file-path contracts for `validate-csl`.
3. Keep `mcp` documented as an explicit placeholder until server mode is wired to the Rust core.
4. Plan the later workflow commands: `extract`, `normalize`, `verify`, `review`, `export`, and `pipeline`.
5. Plan MCP tools/resources/prompts only as interfaces over stabilized Rust core behavior.
6. Add adapter package plans only after the CLI and MCP contracts stabilize.
