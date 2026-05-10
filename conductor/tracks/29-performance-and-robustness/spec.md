# Performance And Robustness Spec

## Goal

Move Sourceright from correctness-only fixture checks toward measured performance, fuzzing, and larger stress coverage.

## Scope

- Keep benchmark outputs deterministic.
- Add timing and fixture-size metrics without making them brittle default gates.
- Run fuzzing on parser-heavy inputs.
- Add larger reference-list fixtures for stress checks.

## Outputs

- Benchmark metric guidance and explicit metric interpretation for correctness-vs-latency
  tasks.
- Scheduled/manual fuzz and performance workflows.
- Larger fixture coverage for parser, sidecar, reference-report, and export paths.

## Measurement Model

- `correctness` tasks compare against checked-in baselines.
- `latency` tasks record `duration_ms` and can be paired with `performance` to
  enforce a maximum duration.
- `performance` gates are only applied when `latency` is included in `measures`.

## Boundaries

Performance gates should not fail default PRs on noisy timing alone. Default CI may run smoke benchmarks; scheduled/manual jobs can run heavier profiling and fuzzing.
