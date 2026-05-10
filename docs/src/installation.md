# Installation

Sourceright ships as a single Rust binary. The intended public install path is
the same executable used for CLI and MCP server mode.

## Local build

```powershell
cargo install --path .
```

That produces a local `sourceright` command from the checked-out repository.

## Release binaries

GitHub Releases are the primary binary distribution surface. Release artifacts
include the platform binary and checksums, and they are validated by the
release gate before publication.

## Registry publication

The crate is prepared for crates.io publication, but the release process uses
`cargo package --locked` and `cargo publish --dry-run --locked` before any
manual publish step.

The MCP server is exposed through the same binary:

```powershell
sourceright mcp
```
