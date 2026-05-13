# Benchmark maturity plan

## Current interpretation

The benchmark exists as a deterministic scaffold under `sourceright-bench/`.
It appears to include:
- tasks manifest;
- fixtures;
- baselines;
- metrics directory;
- stress manifest;
- a CLI surface through `sourceright bench`.

However, `tasks.yaml` currently marks `runner_status: scaffold_only`. That is incompatible with strong public benchmark claims unless clarified.

## Immediate fixes

1. Decide what `sourceright bench` currently executes.
2. Update manifest language:
   - if it is runnable: `runner_status: internal_regression_runner`;
   - if partially runnable: `runner_status: partial_internal_runner`;
   - if scaffold only: keep `scaffold_only`, but document what `sourceright bench` actually does.
3. Add a benchmark status report:
   - `docs/src/benchmark-results.md` or docs-site equivalent;
   - latest fixture count;
   - baseline file count;
   - tasks executed;
   - tasks not yet executed;
   - latency budgets if any;
   - known limitations.
4. Add command examples:
   - `cargo run --bin sourceright -- bench`
   - `cargo run --bin sourceright -- bench --json`
5. Add CI integration if not already present:
   - fast benchmark smoke in PRs;
   - stress benchmark scheduled/manual.

## Benchmark taxonomy

Use two tracks:

### Track A — Internal deterministic regression benchmark

Purpose:
- catch regressions;
- validate artifacts;
- test CLI/core surfaces;
- stay offline and deterministic.

Tasks:
- CSL validation;
- sidecar validation;
- reference report;
- review queue;
- citation reconciliation;
- export suite;
- legal citation report;
- provenance report;
- journal screening;
- policy report;
- plugin manifest validation.

Metrics:
- pass/fail;
- snapshot drift;
- schema validity;
- latency budget;
- fixture coverage.

### Track B — External comparative benchmark

Purpose:
- support stronger claims later.

Do not claim this exists until implemented.

Targets:
- Crossref-only baseline;
- OpenAlex-only baseline;
- GROBID + Crossref baseline;
- CheckIfExist-style multi-provider cascade;
- HalluCiteChecker-style existence detection;
- local fixture-only matcher.

Metrics:
- reference extraction F1;
- field parsing F1;
- identifier recovery recall;
- top-1/top-3 match accuracy;
- false positive rate;
- abstention quality;
- confidence calibration;
- review-queue reduction;
- latency/cost.

## Suggested docs wording

Use now:

> Sourceright includes a deterministic fixture-backed regression benchmark harness.

Avoid for now:

> Sourceright is externally benchmarked SOTA.
