# Benchmark Robustness Contract Test Matrix

| Scenario | Acceptance |
| --- | --- |
| Fixture benchmark | `sourceright bench --json` remains deterministic. |
| Stress benchmark | Stress tasks produce labelled metrics artifacts. |
| Robustness workflow | Failures are triaged against fixture drift or real regressions. |
| Claim boundary | Docs avoid SOTA/external comparability claims without external evidence. |
| Review loop | `$conductor-review` runs and local fixes are applied. |
