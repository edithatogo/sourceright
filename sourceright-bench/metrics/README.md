# Metrics

Scheduled and manual robustness runs execute `sourceright-bench/tasks-stress.yaml` and
write outputs under `sourceright-bench/metrics/` for trend review:

- `robustness-stress-<UTC timestamp>.json` is the full `sourceright.benchmark_run.v1` payload.
- `robustness-stress-<UTC timestamp>.csv` is a flattened task summary for quick
  diffing, with `id`, `baseline`, `passed`, and `duration_ms`.

`sourceright-bench/tasks-stress.yaml` is configured for latency-focused gating:
tasks include `measures: [latency]` and enforce performance ceilings only where marked.

Keep historical files for a few weeks and inspect regressions in `duration_ms`
before enforcing tighter gates.
