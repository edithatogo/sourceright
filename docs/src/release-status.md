# Release Status

The release-status artifact is the single human-readable summary emitted by the
tag-triggered publish workflows. It captures the release surface, tag, run URL,
and the primary evidence expected for a public release.

Each of the following workflows writes a `release-status.md` artifact:

- `release.yml`
- `publish-crate.yml`
- `publish-mcp-registry.yml`

Use the artifact together with the GitHub Release page, crate publication
result, MCP registry submission result, checksums, and attestations.

## Registry Completion Table

As of 2026-05-13, the latest verified public release surface is `v0.1.20`.

Registries are classified into four statuses:

- **Accepted** — publicly listed and installable; URL, version, date, and install metadata recorded.
- **Prepared** — metadata exists in the repository, but no accepted external listing is verified.
- **Deferred** — blocked by a documented requirement; revisit trigger is recorded.
- **N/A** — not applicable to this project; rationale documented.

### Accepted Registries

| Registry | Status | Version | URL / Evidence | Date | Install Metadata |
|----------|--------|---------|---------------|------|-----------------|
| GitHub Release | accepted | 0.1.20 | <https://github.com/edithatogo/sourceright/releases/tag/v0.1.20> | 2026-05-11 | Platform binaries + SHA-256 checksums attached to release. Release workflow publishes on `v*.*.*` tags. |
| crates.io | accepted | 0.1.20 | <https://crates.io/crates/sourceright> | 2026-05-11 | `cargo install sourceright`. Package metadata in `Cargo.toml`. `Publish crate` workflow is manual and token-gated. |
| docs.rs | accepted | 0.1.20 | <https://docs.rs/crate/sourceright/0.1.20> | 2026-05-11 | Auto-built from crates.io publish. `package.metadata.docs.rs` configures `all-features = true`. |
| Official MCP Registry | accepted | 0.1.20 | <https://registry.modelcontextprotocol.io/v0.1/servers?search=io.github.edithatogo/sourceright> | 2026-05-13 | Listed as active and latest. Server metadata from `server.json`. OCI image target: `ghcr.io/edithatogo/sourceright-mcp:0.1.20`. |

### Prepared Registries

| Registry | Status | Version | URL / Evidence | Date | Blocking Requirement |
|----------|--------|---------|---------------|------|---------------------|
| GHCR MCP image | prepared | 0.1.20 | Referenced by MCP Registry listing; direct GHCR package page requires package read permissions. | 2026-05-13 | GHCR package visibility must be confirmed after registry submission. Documented as MCP Registry indirection — the OCI image is published but the GHCR package listing is indirectly evidenced. |
| Glama | prepared | 0.1.20 | `glama.json` with valid schema and maintainer handle `edithatogo`; local verification in `conductor/tracks/73-mcp-directory-submission-hardening/glama-metadata-verification.md`. | 2026-06-09 | No accepted external listing recorded. Glama listing or API verification remains approval-gated. |
| Smithery | accepted | 0.1.20 | Listing https://smithery.ai/servers/edithatogo/sourceright (release `263ee636-5d24-4010-9dd9-e199d4f7b848`); install smoke in `smithery-install-smoke-2026-06-10.md`. | 2026-06-10 | Install via `npx -y @smithery/cli@latest run edithatogo/sourceright`. |
| Claude Desktop client config | prepared | 0.1.20 | Hardened in `conductor/tracks/76-ai-client-extension-publication-hardening/mcp-client-smoke-2026-06-09.md`; `claude-desktop.json` and MCP docs. | 2026-06-09 | No Claude plugin or Cowork marketplace package. Client transcript smoke remains opt-in. |
| Codex MCP client config | prepared | 0.1.20 | Hardened in Track 76 MCP smoke evidence; `codex-config.toml` and `codex-mcp.json`. | 2026-06-09 | No Codex app plugin package. Codex UI configuration proof remains opt-in. |
| Generic MCP client config | prepared | 0.1.20 | Track 76 MCP smoke evidence; `generic-mcp-client.json`, `generic-stdio.json`, and `smoke-requests.jsonl`. | 2026-06-09 | Requires transcript smoke per named client before claiming client-specific compatibility. No Claude, Codex, or Copilot marketplace package exists. |
| GitHub Copilot coding-agent prep | prepared | 0.1.20 | Hardened in Track 76; copilot instructions, setup workflow, and coding-agent example. | 2026-06-09 | No Copilot extension listing. Entitlement/settings verification remains required before enabled claims. |
| Gemini CLI extensions | prepared | — | No-package decision in Track 76; MCP/CLI configuration only until `gemini-extension.json` scaffold exists. | 2026-06-09 | No Gemini CLI extension package or registry listing is claimed. |
| Qwen CLI extensions | prepared | — | No-package decision in Track 76; MCP/CLI configuration only until Qwen extension schema is pinned. | 2026-06-09 | No Qwen CLI extension package or registry listing is claimed. |
| Zotero | prepared | 0.1.20 | CLI/Web API adapter hardened in `conductor/tracks/74-citation-manager-publication-hardening/zotero-adapter-hardening-2026-06-09.md`; install docs, manifest, and Zotero fixtures. | 2026-06-09 | No Zotero `.xpi` or Plugin Gallery listing is claimed. Live disposable-library smoke remains opt-in and approval-gated. |
| EndNote | prepared | 0.1.20 | ENW/RIS file handoff hardened in `conductor/tracks/74-citation-manager-publication-hardening/endnote-reparse-verification-2026-06-09.md`; golden fixtures and export docs. | 2026-06-09 | No EndNote plugin or live sync is claimed. File import in EndNote desktop is operator-verified, not default-CI proof. |
| OJS/PKP | prepared | 0.1.0 plugin | Hardened in `conductor/tracks/75-journal-platform-publication-hardening/ojs-fixture-smoke-2026-06-09.md`; plugin source, install archive, and fixture screening. | 2026-06-09 | No PKP Plugin Gallery acceptance is claimed. Live OJS install smoke and Gallery listing remain approval-gated. |
| VS Code Marketplace / Open VSX | prepared | 0.1.20 | Hardened in `conductor/tracks/77-vscode-open-vsx-publication-hardening/vsix-smoke-2026-06-09.md`; `extensions/vscode-sourceright/` and `dist/vscode-smoke/edithatogo.sourceright-0.1.20.vsix`. | 2026-06-09 | Local VSIX with isolated install/uninstall smoke; no Marketplace/Open VSX listing claimed. Requires public listing evidence before accepted claim. |

### Deferred Registries

| Registry | Status | Version | URL / Evidence | Blocking Requirement | Revisit Trigger |
|----------|--------|---------|---------------|---------------------|-----------------|
| Homebrew | deferred | — | No Homebrew formula maintained. | Binary layout must be stable before formula submission. Need to decide whether to maintain a custom tap or submit to homebrew-core. | Binary release layout stabilised and CI publishing automation in place. |
| Scoop | deferred | — | No Scoop manifest maintained. | Scoop manifest requires stable binary download URL pattern and SHA-256 checksums. | Same gate as Homebrew — stable binary layout with checksum automation. |
| winget | deferred | — | No winget manifest maintained. | winget manifest submission requires GitHub Release binary URLs, checksums, and Microsoft Partner Center account for manifest PRs. | Binary release cadence stable and GitHub Actions automation for manifest generation ready. |
| npm launcher | deferred | — | No npm package maintained. | An npm launcher package is a convenience layer only — it should invoke the Rust binary rather than reimplement verification. Requires stable binary paths. | MCP clients that benefit from `npx` installation demand proven. Create thin npm wrapper that downloads and invokes the Rust binary. |
| PyPI launcher | deferred | — | No PyPI package maintained. | Similar reasoning to npm: a PyPI launcher would download and invoke the Rust binary. No native Python code is planned. | User demand from Python-based MCP clients or data-science workflows. |
| Microsoft AppSource / Word add-in | deferred | — | No Office Add-in manifest or taskpane package maintained. | Requires Track 67 manifest, sideload smoke, document-range provenance, and reversible write plans. | Stable low-noise writeback and document-range provenance. |
| LibreOffice Extensions | deferred | — | No `.oxt`/UNO package maintained. | Requires Track 68 package, Writer range mapping, local install/uninstall smoke, and reversible write plans. | Stable office-document adapter contract and user demand. |

### Not Applicable

| Registry | Status | Rationale |
|----------|--------|-----------|
| Chocolatey | n/a | Chocolatey targets Windows-only .NET ecosystem. Sourceright is a Rust CLI that distributes via GitHub Releases, crates.io, and OCI images. Windows users can install via `cargo install` or download the GitHub Release binary. Chocolatey adds maintenance burden without meaningful user reach for a Rust tool. |

## Package-Manager Feasibility Decisions

| Manager | Decision | Rationale |
|---------|----------|-----------|
| Homebrew | Defer | Binary layout not yet stable. Formula generation could be automated in a future track. |
| Scoop | Defer | Same stability gate as Homebrew. Scoop manifests follow similar URL+checksum pattern. |
| Chocolatey | No | Not applicable — .NET ecosystem focus does not align with Rust CLI distribution. |
| winget | Defer | Requires Microsoft Partner Center account and manifest PR process. Defer until binary release cadence is stable and automated. |
| npm | Defer | Thin launcher wrapper only. No native Node.js code. Defer until MCP client demand materialises. |
| PyPI | Defer | Thin launcher wrapper only. No native Python code. Defer until Python ecosystem demand materialises. |

## Evidence Summary

All accepted registries have been verified manually by inspecting the listed URLs.
Prepared registries have repository metadata but no external acceptance confirmation.
Deferred registries have documented blocking requirements and revisit triggers.
The detailed marketplace evidence model is tracked in
`conductor/tracks/69-marketplace-submission-evidence/marketplace-evidence.md`.
Refresh cadence and promotion rules are tracked in
[Release Surface Refresh](release-surface-refresh.md).

This page is linked from [Operations Status](operations-status.md).
For the full publishing plan see [Publishing Plan](publishing.md).
