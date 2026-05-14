# Track 69 Marketplace Evidence Model

This file is the repo-local evidence model for public marketplaces, registries,
directories, and host package listings. It records the current state without
performing private account-side submissions.

## Evidence Rules

| State | Required evidence |
| --- | --- |
| accepted | Public URL, version or artifact id, verification date, install metadata, and the submission/listing evidence source. |
| prepared | Repository metadata, package/config source, verification date, and blocking requirement for public acceptance. |
| deferred | Explicit missing artifact, claim boundary, blocking requirement, and revisit trigger. |

Prepared metadata is not public marketplace acceptance. Local config examples,
source skeletons, and package manifests can support pilots, but release notes
must not imply public installability until the accepted evidence row exists.

## Current Evidence

| Host or directory | State | Version/artifact | Evidence | Blocking requirement or install metadata |
| --- | --- | --- | --- | --- |
| GitHub Release | accepted | `v0.1.20` | <https://github.com/edithatogo/sourceright/releases/tag/v0.1.20>, verified 2026-05-11. | Platform binaries and SHA-256 checksums are attached to the release. |
| crates.io | accepted | `0.1.20` | <https://crates.io/crates/sourceright>, verified 2026-05-11. | Install with `cargo install sourceright`. |
| docs.rs | accepted | `0.1.20` | <https://docs.rs/crate/sourceright/0.1.20>, verified 2026-05-11. | Public Rust API docs generated from the crates.io package. |
| Official MCP Registry | accepted | `0.1.20` | <https://registry.modelcontextprotocol.io/v0.1/servers?search=io.github.edithatogo/sourceright>, verified 2026-05-13. | Registry metadata binds to `server.json` and `ghcr.io/edithatogo/sourceright-mcp:0.1.20`. |
| GHCR MCP image | prepared | `0.1.20` | Referenced by the Official MCP Registry listing, verified 2026-05-13. | Direct GHCR package-page visibility still requires separate package permission/visibility verification. |
| Glama | prepared | repository metadata | `glama.json` and public license metadata exist, verified 2026-05-13. | Accepted status requires an external Glama listing or API verification. |
| Smithery | prepared | MCPB/local package path | `smithery/mcpb/manifest.template.json` and `scripts/build-smithery-mcpb.ps1`, verified 2026-05-14. | Accepted status requires a built `.mcpb` bundle from a release binary and a verified Smithery listing. |
| Claude Desktop | prepared | local MCP client config | `examples/mcp-clients/claude-desktop.json`, verified 2026-05-14. | This is client configuration, not a Claude marketplace package; accepted status requires host-specific listing evidence if a package is created. |
| Codex | prepared | CLI/MCP workflow config | `examples/mcp-clients/codex-config.toml` and `codex-mcp.json`, verified 2026-05-14. | This is repo-agent/client configuration, not a Codex marketplace package. |
| GitHub Copilot | prepared | coding-agent prep | `.github/copilot-instructions.md`, setup workflow, and `examples/mcp-clients/github-copilot-coding-agent.md`, verified 2026-05-14. | Entitlement/settings verification is required before enabled claims; extension claims require a separate package. |
| Generic MCP clients | prepared | stdio snippets | `examples/mcp-clients/generic-mcp-client.json`, `generic-stdio.json`, and `smoke-requests.jsonl`, verified 2026-05-14. | Named-client claims require transcript smoke and install evidence for that client. |
| Zotero | prepared | CLI/Web API adapter | `docs/src/zotero-plugin-install.md`, `plugins/manifests/citation-manager.zotero.toml`, and `fixtures/providers/zotero/`, verified 2026-05-15. | No Zotero `.xpi` or Plugin Gallery listing is claimed; plugin marketplace claims require a package decision, install proof, and accepted listing evidence. |
| OJS/PKP | prepared | generic plugin source skeleton | `plugins/ojs/sourceright/`, OJS fixture, and policy tests, verified 2026-05-15. | No PKP Plugin Gallery acceptance is claimed; accepted status requires live OJS compatibility proof and Gallery listing evidence. |
| VS Code Marketplace / Open VSX | deferred | none | Track 66 records no VSIX package or marketplace listing. | Requires VSIX package, Workspace Trust support, diagnostics fixture, install smoke, and accepted listing evidence. |
| Microsoft AppSource / Word add-in | deferred | none | Track 67 records no Office Add-in manifest, taskpane, sideload smoke, or AppSource listing. | Requires Office Add-in manifest, sideload smoke, range provenance, reversible write plan, and accepted listing evidence. |
| LibreOffice Extensions | deferred | none | Track 68 records no `.oxt`, UNO bridge, install smoke, or LibreOffice Extensions listing. | Requires `.oxt` package, Writer range mapping, install/uninstall smoke, reversible write plan, and accepted listing evidence. |
| Homebrew, Scoop, winget, npm, PyPI | deferred | none | Release-status docs record no maintained package-manager wrapper or manifest. | Requires stable binary layout, checksum automation, and thin wrapper or manifest policy before submission. |
| Chocolatey | n/a | none | Release-status docs record the .NET ecosystem mismatch. | Not targeted unless the distribution strategy changes. |

## Private Submission Boundary

Private account-side submissions are not represented as completed repo proof.
They can be added only after the submitted artifact, account action, and public
listing state are recorded with a URL, date, version or artifact id, and install
metadata.
