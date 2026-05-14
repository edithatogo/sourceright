# VS Code Extension Packaging Test Matrix

| Area | Required check |
| --- | --- |
| Packaging | VSIX manifest/build exists, or the track records an explicit deferral. |
| Diagnostics | Fixture diagnostics map to VS Code Problems without duplicating core parsing logic. |
| Core reuse | Extension invokes CLI JSON, MCP, or a thin LSP wrapper around the Rust core. |
| Write safety | Fix commands are preview-only until explicit apply semantics are implemented and audited. |
| Security | Workspace trust, binary discovery, live-provider opt-in, and secret handling are documented. |
| Review | `$conductor-review` checks public wording before marketplace claims are promoted. |
