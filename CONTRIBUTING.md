# Contributing

Sourceright is organised through Conductor tracks. Before making substantial changes, read `conductor/tracks.md` and the track folder that owns the behavior you want to change.

## Development checks

Run these before opening a pull request:

```powershell
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo check --locked
cargo run --bin sourceright -- bench
cargo package --locked
cargo deny check advisories bans sources
```

## Pull requests

- Keep changes scoped to one track where possible.
- Add fixtures for reference parsing, verification, export, or review behavior.
- Keep CSL JSON clean and place provenance, confidence, conflicts, and manual-review state in the sidecar model.
- Do not add live network requirements to unit tests; use mocks or recorded fixtures.
- Run `cargo publish --dry-run --locked` before any crates.io publication.
- Keep docs and command help aligned when adding public CLI or MCP surfaces.

## Legacy material

`legacy/humanizer-next/` is included as provenance and regression material. Do not make production runtime behavior depend on legacy JavaScript unless a Conductor track explicitly approves the porting boundary.
