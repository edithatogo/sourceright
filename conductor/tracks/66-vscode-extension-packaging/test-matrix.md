# VS Code Extension Packaging Test Matrix

| Area | Required check | Evidence |
| --- | --- | --- |
| Packaging | VSIX manifest/build exists, or the track records an explicit deferral. | Explicit deferral recorded in `packaging-decision.md`; no VSIX or listing is claimed. |
| Diagnostics | Fixture diagnostics map to VS Code Problems without duplicating core parsing logic. | Future mapping contract records `validate-csl --json`, `report --json`, `citations`, and MCP outputs as sources. |
| Core reuse | Extension invokes CLI JSON, MCP, or a thin LSP wrapper around the Rust core. | Contract requires CLI/MCP/LSP wrapper reuse and prohibits TypeScript reimplementation of verification logic. |
| Write safety | Fix commands are preview-only until explicit apply semantics are implemented and audited. | Contract requires preview-only commands until explicit apply and audit logs exist. |
| Security | Workspace trust, binary discovery, live-provider opt-in, and secret handling are documented. | Contract requires Workspace Trust and opt-in live providers. |
| Review | `$conductor-review` checks public wording before marketplace claims are promoted. | `tests/vscode_extension_packaging_policy.rs` enforces deferral and no-extension claim boundaries. |
