# Benchmarks

The `sourceright-bench/` directory contains the checked-in benchmark manifest,
fixtures, and baselines for deterministic local execution.

The benchmark surface is a technical preview. It is fixture-backed,
deterministic, and intended for local regression and stress checks rather than
live-provider evaluation.

Benchmark runs do not use live providers, citation-manager APIs, or
journal-system credentials by default.

## Claim Boundary

**Fixture-backed regression benchmarks only. Not comparable to external
benchmark suites.**

All benchmark results in this repository are produced against self-authored,
checked-in fixtures. They measure internal correctness and latency regressions
for the core CLI surfaces (CSL validation, sidecar validation, reference
reporting, citation reconciliation, review queue, export, legal citation,
provenance, and journal screening).

These benchmarks do **not**:

- Assert state-of-the-art (SOTA) performance relative to external tools.
- Claim comparability with academic benchmark suites (e.g., those used in
  NLP/IR or citation-matching publications).
- Measure live-provider response quality, throughput under real network
  conditions, or end-to-end pipeline latency against arbitrary datasets.

Any future SOTA or external comparability claim must be backed by a separate,
published evaluation with independently verifiable methodology and a dedicated
evidence entry in the conductor evidence ledger.

Run the benchmark suite with:

```text
cargo run --bin sourceright -- bench
```

The runner reads `sourceright-bench/tasks.yaml`, executes the matching core
surfaces against the checked-in fixtures, and compares the results against the
baselines in `sourceright-bench/baselines/` when a task includes `correctness`
in `measures`.

Each task can include `latency` in `measures` to emit execution timing. The
`performance` section applies enforceable latency ceilings only when `latency` is
explicitly present.

After installation the same suite is available as:

```text
sourceright bench
sourceright bench --json
```

Default CI uses benchmark correctness as the smoke gate. Scheduled and manual
robustness jobs run `tasks-stress.yaml` with enforceable latency budgets for
selected paths.

The checked-in stress fixture and manifest exercise larger CSL, sidecar, report,
and export inputs without credentials.

The `Coverage` workflow runs `cargo llvm-cov` on a schedule or manually. The
`Robustness` workflow runs bounded parser fuzzing (`validate_csl` and
`validate_sidecar`) and a scheduled stress benchmark pass. Metrics from stress runs
are uploaded as artifacts for trend review and do not block CI by default.

For manual execution:

```text
cargo run --locked --bin sourceright -- bench --manifest sourceright-bench/tasks-stress.yaml --json
```
