# Sourceright Benchmark Harness

This directory is a benchmark scaffold, not a Rust workspace crate. It records
deterministic benchmark tasks for the currently implemented CLI/core surfaces.

No task should require live provider calls, API keys, ML/NLP dependencies, or
network access. Fixtures should be small, public-domain or self-authored, and
checked in with enough expected output to make changes reviewable.

The manifest includes both baseline regression cases and a deterministic
larger-workspace stress slice so that parser, sidecar, report, and export paths
can be exercised without live providers.

## Layout

- `tasks.yaml`: benchmark task definitions and expected commands.
- `fixtures/`: source inputs for benchmark tasks.
- `baselines/`: expected output snapshots and acceptance notes.
- `metrics/`: measured results written by future benchmark runners.
- `tasks-stress.yaml`: scheduled/manual stress suite with larger deterministic fixtures.

## Expected Output Convention

Each task should declare:

- the Sourceright surface being exercised;
- the fixture input;
- the expected output or baseline file;
- whether the task measures correctness, latency, or both;
- any known limitations.

Scheduled/manual robustness gates use `tasks-stress.yaml`. Those tasks are intended
for larger inputs and enforceable latency budgets where flagged.

Future runners should write metrics as JSON or CSV and keep correctness
baselines separate from timing results.
