# Publishing Plan

Sourceright should publish one Rust core and keep wrappers thin.

## CLI

The CLI should publish through:

- GitHub Releases with platform binaries.
- SHA-256 checksums.
- Release notes generated from tags.
- crates.io once the command and library surfaces stabilize.
- Later Homebrew, Scoop, and Winget manifests after binary layout is stable.

## MCP

The first MCP distribution should be `sourceright mcp` in the same Rust binary.

An npm launcher package is a later convenience layer only if MCP clients benefit from `npx` installation. It should invoke the Rust binary rather than reimplementing reference verification.

## Release Gates

Release candidates should pass:

- Formatting, clippy, tests, and locked builds.
- Crate packaging.
- GitHub Pages docs build.
- Security scanning.
- Fixture-based reference workflow checks once implemented.
