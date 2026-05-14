# Coverage Verification and Reporting — Review

## Current State

**Status:** Planned → In Progress
**Priority:** High
**Dependencies:** 26-ci-supply-chain-hardening, 29-performance-and-robustness, 31-quality-assurance-hardening

## Evidence Found

### Required Artifacts — All Present

| Artifact | Path | Evidence |
|----------|------|----------|
| Coverage workflow | `.github/workflows/coverage.yml` | CI workflow exists, scheduled + manual trigger, `ubuntu-latest` runner, `--fail-under-lines 85` gate |
| Pre-commit hook | `.githooks/pre-commit` | Contains `CoverageMinimum 85` assertion |
| Pre-commit config | `.pre-commit-config.yaml` | Contains `CoverageMinimum 85` assertion |
| Verify script | `scripts/verify.ps1` | Contains `CoverageMinimum = 85` and `cargo llvm-cov` invocation |
| CONTRIBUTING.md | `docs/src/contributing.md` | References "85 percent floor" |
| README.md | Root | Contains "Coverage stays gated above 85 percent" |
| Coverage report docs | `docs/src/coverage-reporting.md` | Documents policy, local reporting, and caveat about Windows llvm-cov gap |
| Coverage status docs | `docs/src/coverage-status.md` | Documents the human-readable artifact emitted by the coverage workflow |
| Docs-site coverage status | `docs-site/src/content/docs/guides/coverage-status.md` | Frontmatter-titled page with same contract language |
| Benchmark docs coverage ref | `docs/src/benchmarks.md` | Mentions Coverage workflow runs `cargo llvm-cov` on schedule |
| Test: coverage_policy | `tests/coverage_policy.rs` | Asserts `--fail-under-lines 85` in CI, hook, pre-commit config, verify script, CONTRIBUTING, and README |
| Test: coverage_status_policy | `tests/coverage_status_policy.rs` | Asserts `ubuntu-latest` runner, `coverage-status.md` artifact, and docs-site parity |

### Coverage Workflow Details

- **Trigger:** `workflow_dispatch` + `schedule` (Tuesday 04:37 UTC)
- **Runner:** `ubuntu-latest`
- **Steps:** Checkout → Install Rust → Install `cargo-llvm-cov` → Run `cargo llvm-cov --locked --all-targets --summary-only --fail-under-lines 85` → Generate coverage status via `scripts/coverage-status.ps1` → Upload `coverage-report.txt` and `coverage-status.md` artifacts
- **Concurrency:** Grouped by ref, cancel-in-progress

## Gaps

1. **Pre-commit hook parity:** The `.githooks/pre-commit` file references `CoverageMinimum 85` but it is unclear whether this hook is auto-installed or requires manual `git config core.hooksPath`. The test asserts the string exists but does not verify hook invocation.
2. **Windows coverage caveat:** `coverage-reporting.md` documents that Windows cannot produce a fresh `cargo llvm-cov` report. This is a documented limitation but means local coverage verification on Windows requires a different environment.
3. **No coverage badge/visual:** The test matrix mentions "Coverage output is reproducible" and there is a status artifact, but there is no SVG badge or dashboard-style visualization in the README.
4. **Script verification:** `scripts/coverage-status.ps1` is referenced but not checked by the policy tests for correctness — only its output path is asserted.

## Completion Signal Assessment

The spec says: "Coverage can be measured repeatably, and the 85 percent floor is visible in the repo's developer and CI surfaces."

**Assessment:** The 85% floor IS visible in CI, the hook, the pre-commit config, verify script, CONTRIBUTING, and README. The coverage workflow is scheduled and manual-trigger-capable. The human-readable `coverage-status.md` artifact is emitted. Coverage-reporting.md documents the local workflow and caveats.

**Readiness:** Substantially complete. The remaining work items are:
- Verify pre-commit hook auto-install mechanism
- Confirm coverage-status.ps1 correctness under `pwsh`
- Evaluate whether Windows coverage gap needs resolution or just documentation (currently documented)
