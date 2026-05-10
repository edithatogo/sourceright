# Benchmark Runner Automation Spec

## Goal

Turn `sourceright-bench/` from a scaffold into an executable benchmark runner.

## Scope

- Define benchmark task manifests.
- Run deterministic fixture-backed benchmark tasks.
- Compare generated outputs against committed baselines.
- Cover academic references, legal citations, claim/source/provenance, and journal screening flows.
- Avoid live providers in default benchmark execution.

## Outputs

- Executable benchmark runner.
- Benchmark task manifests.
- Fixture inputs and expected baselines.
- Baseline comparison reports.
- Benchmark documentation.

## Boundaries

Benchmarks must be deterministic and local by default. They must not call live providers, citation-manager APIs, or journal systems unless a separate opt-in smoke path is added later.

Benchmark assertions should compare Sourceright outputs, not assert claim truth.
