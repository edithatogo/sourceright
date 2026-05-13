# 01 — Baseline quality gates

Proceed with baseline checks only.

Run the checks that are available in the current repo:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo check --locked
cargo run --bin sourceright -- bench
```

Also run docs/demo/schema validation commands if the repo already documents them.

Summarize:
1. pass/fail results;
2. failures that are pre-existing;
3. whether any failures block new work;
4. safest first implementation slice.

Do not make non-trivial source changes.
Do not commit.
