# 02 — Benchmark maturity implementation

Implement the smallest changes needed to make the benchmark status honest and useful.

Tasks:
1. Inspect `sourceright-bench/tasks.yaml`, `tasks-stress.yaml`, baselines, fixtures, and `src/bench.rs`.
2. Determine what `cargo run --bin sourceright -- bench` actually executes.
3. Update docs/manifest language so the benchmark is accurately described:
   - internal deterministic regression benchmark;
   - not an external SOTA benchmark.
4. Add or update a benchmark results/interpretation page.
5. Ensure `sourceright bench --json` is documented if supported.
6. Add tests only if they are deterministic and offline.
7. Run:
   - `cargo test`
   - `cargo run --bin sourceright -- bench`
   - docs/schema validation if relevant.

Do not add live provider calls.
Do not add ML/NLP dependencies.
Do not claim external comparative benchmarking.
