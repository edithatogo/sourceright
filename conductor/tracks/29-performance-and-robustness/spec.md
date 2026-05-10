# Performance And Robustness Spec

## Goal

Move Sourceright from correctness-only fixture checks toward measured performance, fuzzing, and larger stress coverage.

## Scope

- Keep benchmark outputs deterministic.
- Add timing and fixture-size metrics without making them brittle default gates.
- Run fuzzing on parser-heavy inputs.
- Add larger reference-list fixtures for stress checks.

## Outputs

- Benchmark metric guidance.
- Scheduled/manual fuzz and performance workflows.
- Larger fixture coverage.

## Boundaries

Performance gates should not fail default PRs on noisy timing alone. Default CI may run smoke benchmarks; scheduled/manual jobs can run heavier profiling and fuzzing.
