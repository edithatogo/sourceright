Proceed with baseline checks only.

Run:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo check --locked
```

Then summarize:
1. Which checks pass or fail.
2. Any existing issues that must be fixed before adding new surfaces.
3. The safest first additive slice.

Do not modify files unless needed to make the baseline commands runnable. Ask before any non-trivial fix.
