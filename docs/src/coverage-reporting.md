# Coverage Reporting

Coverage is treated as a hard quality floor, not an informal dashboard number.

## Policy

- The repository target is above 90 percent line coverage.
- The same floor is referenced in CI, the pre-commit hook, and contributor
  guidance.
- Coverage checks should remain deterministic and should not become flaky
  release blockers.

## Local Reporting

Use the shared verification script where possible:

```powershell
pwsh -NoProfile -ExecutionPolicy Bypass -File scripts/verify.ps1 -CoverageMinimum 90
```

On supported coverage toolchains, the script runs formatting, linting, tests,
and the coverage gate together so the reported number reflects the same state
the CI gate checks.

## Caveat

The Windows toolchain in this environment cannot currently produce a fresh
`cargo llvm-cov` report, so the repo documents the threshold and enforces it in
workflow configuration even when the numeric report itself must be generated in
another supported environment.
