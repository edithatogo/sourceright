---
title: Benchmarks
description: Fixture-backed technical-preview benchmark guidance for Sourceright.
---

The benchmark surface is a technical preview. It is fixture-backed,
deterministic, and intended for local regression and stress checks rather than
live-provider evaluation.

- `sourceright bench` runs the checked-in `sourceright-bench/tasks.yaml`
  fixture suite.
- Benchmark runs do not use live providers, citation-manager APIs, or
  journal-system credentials by default.
- Use `sourceright-bench/tasks-stress.yaml` for larger fixture runs and
  scheduled or manual robustness jobs.
- Default CI uses benchmark correctness as the smoke gate.
- Treat benchmark drift as a quality signal, not a release blocker by default.
