# Contributing

Sourceright work is planned through Conductor tracks. Start with `conductor/tracks.md` and the track folder that owns the behavior you want to change.

Before opening a pull request, run:

```text
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo check --locked
```

Keep provider tests deterministic and avoid live network requirements in unit tests.
