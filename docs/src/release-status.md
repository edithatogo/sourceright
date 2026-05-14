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
| Glama | prepared | — | `glama.json` present at repository root with valid schema and maintainer handle `edithatogo`. | 2026-05-13 | No accepted external listing recorded. Glama requirements: valid `glama.json`, public `LICENSE`, MCP metadata discoverable from repo files and release artifacts. Manual verification or Glama API check required for accepted status. |
| Smithery | prepared | 0.1.20 | `smithery/mcpb/manifest.template.json` and `scripts/build-smithery-mcpb.ps1` define the MCPB/local stdio package path. | 2026-05-14 | No accepted Smithery listing recorded. Publish only after a concrete `.mcpb` bundle is built from a release binary and the Smithery listing is verified. |
| Claude Desktop client config | prepared | — | `docs/src/mcp.md` and `examples/mcp-clients/claude-desktop.json` document local stdio `mcpServers.sourceright` launch for `sourceright mcp`. | 2026-05-14 | Requires local client transcript smoke before claiming verified client setup. No Claude, Codex, or Copilot marketplace package exists. |
| Codex MCP client config | prepared | — | `docs/src/mcp.md`, `examples/mcp-clients/codex-config.toml`, and `examples/mcp-clients/codex-mcp.json` document local CLI/MCP workflow configuration. | 2026-05-14 | Requires local Codex configuration proof for the target build before claiming verified Codex setup. No Claude, Codex, or Copilot marketplace package exists. |
| Generic MCP client config | prepared | — | `docs/src/mcp.md`, `examples/mcp-clients/generic-mcp-client.json`, `generic-stdio.json`, and `smoke-requests.jsonl` document protocol-level stdio launch and dry-run evidence. | 2026-05-14 | Requires transcript smoke per named client before claiming client-specific compatibility. |
| GitHub Copilot coding-agent prep | prepared | — | `.github/copilot-instructions.md`, setup workflow, security-remediation issue template, and `examples/mcp-clients/github-copilot-coding-agent.md` are present; MCP client packaging remains separate. | 2026-05-14 | Requires GitHub-side entitlement/settings verification before enabled claims; separate package evidence is required for extension claims. No Claude, Codex, or Copilot marketplace package exists. |
| Zotero | prepared | — | `docs/src/zotero-plugin-install.md`, `plugins/manifests/citation-manager.zotero.toml`, and `fixtures/providers/zotero/` document the CLI/Web API adapter, preview/apply/audit semantics, and fixture-backed proof. | 2026-05-15 | No Zotero `.xpi` or Plugin Gallery listing is claimed. Plugin marketplace claims require a package decision, install proof, and accepted listing evidence. |
| OJS/PKP | prepared | — | `plugins/ojs/sourceright/`, `fixtures/journal/ojs-submission.json`, and `tests/ojs_plugin_packaging_policy.rs` document the generic-plugin source skeleton and fixture-backed screening path. | 2026-05-15 | No PKP Plugin Gallery acceptance is claimed. Accepted status requires live OJS compatibility proof and Gallery listing evidence. |

### Deferred Registries

| Registry | Status | Version | URL / Evidence | Blocking Requirement | Revisit Trigger |
|----------|--------|---------|---------------|---------------------|-----------------|
| Homebrew | deferred | — | No Homebrew formula maintained. | Binary layout must be stable before formula submission. Need to decide whether to maintain a custom tap or submit to homebrew-core. | Binary release layout stabilised and CI publishing automation in place. |
| Scoop | deferred | — | No Scoop manifest maintained. | Scoop manifest requires stable binary download URL pattern and SHA-256 checksums. | Same gate as Homebrew — stable binary layout with checksum automation. |
| winget | deferred | — | No winget manifest maintained. | winget manifest submission requires GitHub Release binary URLs, checksums, and Microsoft Partner Center account for manifest PRs. | Binary release cadence stable and GitHub Actions automation for manifest generation ready. |
| npm launcher | deferred | — | No npm package maintained. | An npm launcher package is a convenience layer only — it should invoke the Rust binary rather than reimplement verification. Requires stable binary paths. | MCP clients that benefit from `npx` installation demand proven. Create thin npm wrapper that downloads and invokes the Rust binary. |
| PyPI launcher | deferred | — | No PyPI package maintained. | Similar reasoning to npm: a PyPI launcher would download and invoke the Rust binary. No native Python code is planned. | User demand from Python-based MCP clients or data-science workflows. |
| VS Code Marketplace / Open VSX | deferred | — | No VSIX extension package maintained. | Requires Track 66 package, diagnostics mapping, workspace-trust docs, and install smoke. | Stable CLI/MCP diagnostics and clear editor demand. |
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

This page is linked from [Operations Status](operations-status.md).
For the full publishing plan see [Publishing Plan](publishing.md).
