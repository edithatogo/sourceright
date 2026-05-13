# 09 — Final review

Review all uncommitted changes.

Check:
1. no accidental overwrite of existing source;
2. no stale overlay files copied into root;
3. no live network calls in benchmarks/demos;
4. no SOTA/production overclaims;
5. benchmark wording is honest;
6. plugin/provider status is explicit;
7. demos are safe and sample-only;
8. docs build;
9. Rust checks pass.

Run:
```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo check --locked
cargo run --bin sourceright -- bench
```

Then summarize:
- changed files;
- checks;
- remaining risks;
- recommended commit grouping;
- launch readiness.
