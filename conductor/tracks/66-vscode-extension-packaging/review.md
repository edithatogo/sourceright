# Track 66 — VS Code Extension Packaging Review

## Status

Completed as explicit deferral with contract.

## Evidence

| Area | Result |
| --- | --- |
| Current repo assets | `.vscode/settings.json` is development tooling; `examples/mcp-clients/vscode-mcp.json` is MCP client configuration. |
| Package decision | `packaging-decision.md` records no VSIX, no Marketplace listing, and no Open VSX listing. |
| Future contract | Extension must reuse CLI/MCP/LSP wrapper outputs and must not reimplement verification logic. |
| Diagnostics | Mapping from Sourceright JSON outputs to VS Code Problems is documented. |
| Write safety | Future commands stay preview-only until explicit apply semantics and audit logs exist. |
| Marketplace boundary | Public Marketplace/Open VSX acceptance is deferred to Track 69 evidence. |

## Remaining External Work

- Create an actual VSIX package if editor demand justifies it.
- Add local install/uninstall smoke for that package.
- Add Workspace Trust and binary discovery implementation tests.
- Record VS Code Marketplace/Open VSX listing evidence only after acceptance.

## Review Outcome

Track 66 should not create extension code prematurely. The repo now has the
required explicit deferral and future contract while preserving the public claim
boundary that development settings and MCP snippets are not a VS Code extension.
