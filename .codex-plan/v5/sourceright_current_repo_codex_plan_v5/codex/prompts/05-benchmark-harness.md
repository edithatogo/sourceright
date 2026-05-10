Proceed with Slice 4: benchmark harness.

Add `sourceright-bench/` as a benchmark scaffold, not a Rust workspace crate yet.

Include:

```text
sourceright-bench/README.md
sourceright-bench/tasks.yaml
sourceright-bench/fixtures/
sourceright-bench/baselines/
sourceright-bench/metrics/
```

Benchmark implemented surfaces first:
- CSL validation
- sidecar validation
- reference reporting
- citation reconciliation
- review queue generation
- export correctness
- legal citation extraction
- provenance graph extraction
- journal screening JSON

Do not introduce ML/NLP dependencies yet.
Do not add live provider calls.
Do not turn this into a workspace crate yet.

Add clear fixture licensing notes and expected-output conventions.
