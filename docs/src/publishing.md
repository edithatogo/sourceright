# Publishing Plan

Sourceright should publish one Rust core and keep wrappers thin.

The current release path is staged rather than auto-published from a dirty
tree. Local changes must be committed, pushed, and green in GitHub Actions
before any external registry publication.

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

Smithery URL publishing requires Streamable HTTP, so Sourceright should use an
MCPB/local distribution path there until an HTTP transport exists.

When Smithery URL publishing is ready, set up a project configuration and expose
`sourceright mcp` under the required command surface. Until then, keep Smithery
readiness documented as a future distribution mode.

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
- Duplicate-dependency checks via `cargo tree -d --locked --target x86_64-unknown-linux-gnu`, with the known `wit-bindgen` split ignored.
