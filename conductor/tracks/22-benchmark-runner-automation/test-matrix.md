# Benchmark Runner Automation Test Matrix

| Scenario | Expected result |
| --- | --- |
| Academic reference benchmark | Fixture input produces expected CSL, sidecar, report, and export outputs. |
| Legal citation benchmark | Fixture input produces legal citation outputs separate from academic CSL. |
| Claim/source benchmark | Fixture input produces provenance graph outputs without asserting claim truth. |
| Journal screening benchmark | Fixture submission produces expected editorial and author-facing report artifacts. |
| Baseline match | Runner exits successfully when outputs match baselines. |
| Baseline mismatch | Runner reports structured diffs and exits nonzero. |
| Missing fixture | Runner reports a clear task configuration error. |
| Default execution | Benchmarks require no live providers, citation-manager APIs, or journal-system credentials. |
