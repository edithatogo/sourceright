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

## MCP

The first MCP distribution should be `sourceright mcp` in the same Rust binary.

The official MCP Registry currently supports package metadata for npm, PyPI,
NuGet, OCI, and MCPB package types. Because Sourceright is a Rust CLI, the
registry-ready path is the OCI image declared in `server.json`; crates.io is
still useful for Rust installation but is not the MCP registry package target.

The release workflow builds and pushes `ghcr.io/edithatogo/sourceright-mcp`
on version tags. The manual `Publish MCP registry metadata` workflow uses
GitHub OIDC and `mcp-publisher` to submit `server.json` after the image exists.

Smithery URL publishing requires Streamable HTTP, so Sourceright should use a
local MCPB bundle there until an HTTP transport exists. Glama can index an
open-source repository when it can build, run, and introspect the MCP server.

An npm launcher package is a later convenience layer only if MCP clients
benefit from `npx` installation. It should invoke the Rust binary rather than
reimplementing reference verification.

## Documentation

The current documentation stack is mdBook, built by GitHub Pages. Starlight on
Astro is a reasonable later product-site option, but it should replace mdBook
only if the migration includes navigation, deployment, and CI parity. Until
then, mdBook remains the source of truth.

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
