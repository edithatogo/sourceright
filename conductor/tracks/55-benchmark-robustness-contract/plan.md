# Benchmark Robustness Contract Plan

1. Inventory `tasks.yaml`, `tasks-stress.yaml`, baselines, fixtures, and metrics.
2. Ensure robustness workflow output is reproducible and labelled.
3. Add gates for baseline drift, stress results, and coverage artifact linkage.
4. Update benchmark docs and overclaim policy.
5. Run benchmark/robustness checks and `$conductor-review`.
6. Apply local fixes automatically.

## Progress

- [x] Inventory completed — 15 tasks in tasks.yaml, 4 stress tasks in
      tasks-stress.yaml; baselines, fixtures, and metrics directories all
      populated.
- [x] robustness.yml updated — added push/PR triggers on `main`, labelled
      artifact upload keyed by `${{ github.sha || github.run_id }}`, benchmark
      smoke output saved to `bench-artifacts/benchmark-results.json`.
- [x] benchmarks.md updated — added "Claim Boundary" section with explicit
      "Fixture-backed regression benchmarks only. Not comparable to external
      benchmark suites." wording; clear list of what benchmarks do NOT assert.
- [x] coverage-reporting.md verified — already documents 85 % coverage floor,
      local script invocation, and CI artifact publication path.
- [x] metadata.json status set to "completed".
- [x] review.md created with track summary and check results.
- [x] evidence-ledger.json updated with fixture-backed entry for track 55.
