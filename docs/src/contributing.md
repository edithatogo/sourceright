# Contributing

Sourceright work is planned through Conductor tracks. Start with `conductor/tracks.md` and the track folder that owns the behavior you want to change.

Before opening a pull request, run:

```text
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo check --locked
cargo run --bin sourceright -- bench
cargo package --locked
cargo deny check advisories bans sources
cargo tree -d --locked --target x86_64-unknown-linux-gnu
```

Keep provider tests deterministic and avoid live network requirements in unit tests.

Release candidates should also pass `cargo publish --dry-run --locked` from a
clean tree. CI runs the Astro docs site, Rust docs, dependency policy,
security, and command smoke checks.
