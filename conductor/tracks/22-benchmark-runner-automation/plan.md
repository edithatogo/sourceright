# Benchmark Runner Automation Plan

1. Audit `sourceright-bench/`.
   - Identify existing scaffold files and intended task categories.
   - Define the minimum executable benchmark contract.

2. Define benchmark manifests.
   - Include task id, input fixtures, expected outputs, command or library path, and comparison mode.
   - Separate academic, legal, claim/provenance, and journal workflow fixtures.

3. Implement the runner.
   - Execute tasks deterministically against fixtures.
   - Write structured result summaries.
   - Compare generated outputs against baselines.

4. Add baseline management.
   - Store expected outputs in reviewable files.
   - Report diffs clearly.
   - Avoid automatic baseline updates unless explicitly requested.

5. Add tests and documentation.
   - Add smoke benchmarks for each supported task family.
   - Document local execution and CI expectations.
   - Keep live-provider and external-system checks out of default benchmarks.
