# Benchmarks

The `sourceright-bench/` directory contains the checked-in benchmark manifest,
fixtures, and baselines for deterministic local execution.

Benchmark runs are fixture-backed and do not use live providers,
citation-manager APIs, or journal-system credentials by default.

Run the benchmark suite with:

```text
cargo run --bin sourceright -- bench
```

The runner reads `sourceright-bench/tasks.yaml`, executes the matching core
surfaces against the checked-in fixtures, and compares the results against the
baselines in `sourceright-bench/baselines/`.

After installation the same suite is available as:

```text
sourceright bench
sourceright bench --json
```

Default CI should use benchmark correctness as a smoke gate. Timing and larger
stress fixtures belong in scheduled or manual profiling jobs until thresholds
are stable enough to avoid noisy PR failures.

The `Coverage` workflow runs `cargo llvm-cov` on a schedule or manually. The
`Robustness` workflow runs a bounded `cargo-fuzz` smoke target over CSL input
parsing. Both remain outside default PR CI until their runtime and signal are
stable.
