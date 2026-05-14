# Publishing Plan

Sourceright should publish one Rust core and keep wrappers thin.

The current release path is staged rather than auto-published from a dirty
tree. Local changes must be committed, pushed, and green in GitHub Actions
before any external registry publication.

The full registry completion table, including accepted, prepared, deferred,
and not-applicable registries, is documented in
[Release Status](release-status.md).

## CLI

The CLI should publish through:

- GitHub Releases with platform binaries.
- SHA-256 checksums.
- Release notes generated from tags.
- crates.io after `cargo publish --dry-run --locked` passes.
- Later Homebrew, Scoop, and Winget manifests after binary layout is stable.

The crate metadata is prepared for crates.io and docs.rs. Real crates.io
publication is handled by the manual `Publish crate` workflow, which should be
protected by the `crates-io` environment and a `CARGO_REGISTRY_TOKEN` secret.
The release workflow also runs `cargo package --locked`,
`cargo publish --dry-run --locked`, `cargo deny check advisories bans sources`,
and `cargo tree -d --locked --target x86_64-unknown-linux-gnu` before any GitHub release is cut.
The duplicate check ignores the known `wit-bindgen` split that comes from the
WASI support crates; any other duplicate should still fail the gate.

## MCP

The first MCP distribution should be `sourceright mcp` in the same Rust binary.

The official MCP Registry currently supports package metadata for npm, PyPI,
NuGet, OCI, and MCPB package types. Because Sourceright is a Rust CLI, the
registry-ready path is the OCI image declared in `server.json`; crates.io is
still useful for Rust installation but is not the MCP registry package target.

The release workflow builds and pushes `ghcr.io/edithatogo/sourceright-mcp`
on version tags. The `Publish MCP registry metadata` workflow now follows the
release workflow completion so it can submit `server.json` after the image
exists. Manual dispatch remains available for controlled retries.
The Docker image carries matching Open Containers metadata so registry scans
can tie the container back to the source repository and declared MCP server
name.

### Smithery

Smithery supports two publication modes relevant to Sourceright:

- URL publishing for hosted MCP servers that expose Streamable HTTP.
- MCPB bundle publishing for local stdio servers.

Sourceright has chosen the MCPB/local path for Smithery because the current MCP
runtime is `sourceright mcp` over stdio. The prepared bundle contract is:

- `smithery/mcpb/manifest.template.json` — MCPB manifest v0.3 template for the
  local stdio server.
- `scripts/build-smithery-mcpb.ps1` — stages a `.mcpb` bundle from a supplied
  release binary and writes platform-specific manifest fields.
- `tests/smithery_distribution_policy.rs` — keeps the manifest and docs aligned
  with the prepared-not-accepted status.

Track 57 owns the Smithery publication path. It must choose and validate either
Streamable HTTP publishing or MCPB/local packaging before release notes or docs
claim Smithery availability. Prepared metadata is not accepted-listing evidence.
The current state is MCPB-prepared, not Smithery-accepted.

### Glama

Glama indexing is driven by repository scanning, and ownership for org-hosted repos
is typically asserted via `glama.json`.

Add `glama.json` at repository root and keep the repository publicly discoverable
with license metadata so Glama can complete install-readiness checks.

Current Glama requirements to track:

- `glama.json` present with valid schema and maintainer handle.
- Public `LICENSE`.
- MCP metadata discoverable from repository files and release artifacts.

An npm launcher package is a later convenience layer only if MCP clients
benefit from `npx` installation. It should invoke the Rust binary rather than
reimplementing reference verification.

## Documentation

The current documentation stack is the Starlight/Astro site under
`docs-site/`, built by GitHub Pages. `docs/src/` remains the archival Markdown
source, but the public docs target is now the Astro build.

Track 30 owns the Starlight/Astro migration and deployment parity work. Keep
the archival Markdown source aligned with the public site content.

The operational sequence for live release work is documented in
[Release Runbook](release-runbook.md), while the coverage floor and docs
cutover notes are captured in [Coverage Reporting](coverage-reporting.md) and
[Docs Cutover](docs-cutover.md).

Tag creation with a `v*.*.*` release tag automatically starts the crate publish
workflow, and the MCP registry workflow follows the release workflow
completion. The manual dispatch entries remain for controlled retries.

Documentation and contributor checks also run typo validation through
`typos.toml`, so new public text should be reviewed with the same bar as code.

## Release Gates

Release candidates should pass:

- Formatting, clippy, tests, and locked builds.
- Crate packaging.
- Crates.io publish dry run.
- GitHub Pages docs build.
- Security scanning.
- Dependency policy checks.
- Fixture-based benchmark checks.
- MCP metadata and OCI image checks before registry submission.
- Glama metadata validity (`glama.json`) and MCP distribution checks.
- Duplicate-dependency checks via `cargo deny check advisories bans sources`
  with `deny.toml` skipping the known `wit-bindgen` split, plus
  `cargo tree -d --locked --target x86_64-unknown-linux-gnu` as a secondary
  duplicate scan.
