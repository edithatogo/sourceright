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
| Glama | accepted | https://glama.ai/mcp/servers/c7qsbvekc1 | API `c7qsbvekc1` (namespace `edithatogo`, slug `sourceright`), verified 2026-06-10. | Listing via Glama Add Server; slug-path URLs still 404. |
| Smithery | accepted | https://smithery.ai/servers/edithatogo/sourceright | MCPB release `263ee636-5d24-4010-9dd9-e199d4f7b848`; registry install smoke via `smithery mcp add --client cursor`, verified 2026-06-10. | Install via `npx -y @smithery/cli@latest run edithatogo/sourceright` (see `smithery-install-smoke-2026-06-10.md`). |
| Claude Desktop | prepared | local MCP client config | Track 76 MCP smoke (`mcp-client-smoke-2026-06-09.md`) and `claude-desktop.json`, verified 2026-06-09. | Client configuration, not a Claude marketplace package; optional MCPB path in Track 73. |
| Codex | prepared | CLI/MCP workflow config | Track 76 MCP smoke and `codex-config.toml` / `codex-mcp.json`, verified 2026-06-09. | Repo-agent/client configuration, not a Codex marketplace package. |
| GitHub Copilot | prepared | coding-agent prep | Track 76 hardening; copilot instructions and coding-agent example, verified 2026-06-09. | Entitlement/settings verification required before enabled claims; no extension listing. |
| Gemini CLI extensions | prepared | no-package decision | Track 76 package decisions; MCP/CLI config only, verified 2026-06-09. | No Gemini CLI extension package until schema pinning and install smoke. |
| Qwen CLI extensions | prepared | no-package decision | Track 76 package decisions; MCP/CLI config only, verified 2026-06-09. | No Qwen CLI extension package until schema pinning and install smoke. |
| Generic MCP clients | prepared | stdio snippets | Track 76 MCP smoke and `smoke-requests.jsonl`, verified 2026-06-09. | Named-client claims require transcript smoke and install evidence for that client. |
| Zotero | prepared | CLI/Web API adapter | Track 74 adapter hardening (`zotero-adapter-hardening-2026-06-09.md`), install docs, manifest, and fixtures, verified 2026-06-09. | No Zotero `.xpi` or Plugin Gallery listing is claimed; live disposable-library smoke remains opt-in. |
| EndNote | prepared | ENW/RIS file handoff | Track 74 reparse verification (`endnote-reparse-verification-2026-06-09.md`), golden fixtures, and export docs, verified 2026-06-09. | No EndNote plugin or live sync is claimed; desktop import is operator-verified. |
| OJS/PKP | prepared | generic plugin install archive | Track 75 colab-equivalent smoke (`ojs-colab-smoke-2026-06-10.md`) plus fixture/package smoke, verified 2026-06-10. | No PKP Plugin Gallery acceptance; disposable Docker OJS install remains opt-in. |
| VS Code Marketplace / Open VSX | prepared | local VSIX package | Track 77 hardening (`vsix-smoke-2026-06-09.md`), Workspace Trust, and isolated install/uninstall smoke, verified 2026-06-09. | No VS Code Marketplace or Open VSX acceptance is claimed; accepted status requires public listing evidence. |
| Microsoft AppSource / Word add-in | deferred | none | Track 67 records no Office Add-in manifest, taskpane, sideload smoke, or AppSource listing. | Requires Office Add-in manifest, sideload smoke, range provenance, reversible write plan, and accepted listing evidence. |
| LibreOffice Extensions | deferred | none | Track 68 records no `.oxt`, UNO bridge, install smoke, or LibreOffice Extensions listing. | Requires `.oxt` package, Writer range mapping, install/uninstall smoke, reversible write plan, and accepted listing evidence. |
| Homebrew, Scoop, winget, npm, PyPI | deferred | none | Release-status docs record no maintained package-manager wrapper or manifest. | Requires stable binary layout, checksum automation, and thin wrapper or manifest policy before submission. |
| Chocolatey | n/a | none | Release-status docs record the .NET ecosystem mismatch. | Not targeted unless the distribution strategy changes. |

## Private Submission Boundary

Private account-side submissions are not represented as completed repo proof.
They can be added only after the submitted artifact, account action, and public
listing state are recorded with a URL, date, version or artifact id, and install
metadata.
