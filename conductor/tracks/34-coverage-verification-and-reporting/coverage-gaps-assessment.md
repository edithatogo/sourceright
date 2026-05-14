# Coverage Gaps Assessment

## 1. What the Coverage Workflow Actually Measures

The `.github/workflows/coverage.yml` workflow runs `cargo llvm-cov` with:

```
cargo llvm-cov --locked --all-targets --summary-only --fail-under-lines 85
```

This measures **line coverage** across all targets (lib, bins, tests, benches,
examples). The `--summary-only` flag means the workflow emits an aggregate
percentage, not per-file detail. The output is piped through `tee` into
`coverage-report.txt` for artifact upload.

### What is covered:
- All Rust source in `src/`, `tests/`, `benches/`, and examples.
- Test code itself counts toward coverage (integration tests exercise the
  library and contribute positively).

### What is NOT covered:
- `legacy/humanizer-next/` JavaScript (excluded by `.gitignore`-level tooling).
- Shell scripts, PowerShell scripts, and CI workflow YAML.
- Documentation files (Markdown, CSL JSON fixtures).
- Build scripts (`build.rs`) — unless tests exercise them.

### Trigger cadence:
- Scheduled weekly (Tuesday 04:37 UTC).
- Manual via `workflow_dispatch`.
- NOT triggered on push or PR (unlike the main CI workflow).

### What a pass/fail means:
- **Pass:** `cargo llvm-cov` exits 0, meaning line coverage >= 85%.
- **Fail:** The step exits non-zero (coverage below 85%). The workflow job
  fails, and the badge (workflow status badge in README) goes red.

## 2. Windows Limitation

`docs/src/coverage-reporting.md` documents:

> The Windows toolchain in this environment cannot currently produce a fresh
> `cargo llvm-cov` report.

### Root cause:
`cargo llvm-cov` depends on LLVM profiling instrumentation (`-C
instrument-coverage`), which requires the LLVM tools component of the Rust
toolchain. On Windows, the `llvm-tools-preview` component may not be
available or may not produce correct profile data in all environments.

### Impact:
- The **coverage workflow** only runs on `ubuntu-latest`. Windows and macOS
  are not used for coverage measurement.
- The **pre-commit hook** (`scripts/verify.ps1`) may fail at the `cargo
  llvm-cov` step on Windows. Contributors can use `-SkipCoverage`.
- The **numeric report** (`coverage-report.txt` artifact) is always generated
  on Linux, so there is no cross-platform variance in the reported number.

### Mitigation:
- The Windows limitation is documented in `docs/src/coverage-reporting.md`.
- `CONTRIBUTING.md` does not mention the Windows limitation — this is a
  documentation gap that could confuse Windows contributors.
- The `--locked` flag ensures dependency consistency across platforms for
  non-coverage steps (fmt, clippy, test).

### Assessment:
This is an **acceptable documented limitation** for a Rust project in
technical preview. Full cross-platform coverage would require either:
- A different coverage tool (e.g., `tarpaulin` on Windows).
- Docker-based local coverage (WSL2 on Windows).
Neither is in scope for this track.

## 3. Coverage Badge Status

### Current state:

The README.md already includes a GitHub Actions workflow badge:

```markdown
[![Coverage](https://github.com/edithatogo/sourceright/actions/workflows/coverage.yml/badge.svg)](https://github.com/edithatogo/sourceright/actions/workflows/coverage.yml)
```

This is a **workflow status badge** — it shows whether the latest scheduled
(or manual) coverage workflow passed or failed. It does NOT display a
coverage percentage.

### What a percentage badge would need:

A coverage percentage badge (e.g., "85%" displayed in the README) would
require one of:

| Approach                        | Effort | Notes                                           |
|---------------------------------|--------|--------------------------------------------------|
| **Codecov / Coveralls**         | Medium | Requires an account, token, upload step in CI. Adds an external service dependency. |
| **shields.io JSON endpoint**    | Low    | Would require a JSON artifact with the percentage, then a dynamic shields.io URL. The workflow already uploads `coverage-report.txt` but not as JSON. |
| **GitHub Pages + static badge** | Medium | Generate an SVG or use shields.io with a static endpoint. Requires Pages setup. |
| **Custom SVG in workflow**      | Medium | Parse the coverage output, extract the percentage, generate an SVG badge, commit/push to a badge branch or use gist. |

### Assessment:

The current workflow status badge already provides a visual signal (green =
coverage >= 85%, red = coverage < 85%). A percentage badge would be
informative but is not required for the gating function — the gate is
binary (pass/fail). A percentage badge is a **nice-to-have enhancement**,
not a gap for track completion.

### Recommendation:
If a percentage badge is desired, the lowest-effort path is:
1. Add a step to parse `coverage-report.txt` and extract the percentage.
2. Write it to a `coverage-shield.json` artifact.
3. Use `shields.io/endpoint` with the raw artifact URL.

This would require the artifact to be publicly accessible or routed through
GitHub Pages. Defer to a future track or a dedicated PR.

## 4. Script Verification Status

### `scripts/coverage-status.ps1`:

```powershell
param(
    [int]$CoverageMinimum = 85,
    [string]$Outcome = "passed",
    [string]$OutputPath = "coverage-status.md"
)
```

This script generates a human-readable Markdown status file. It is invoked
by the CI workflow:

```yaml
- name: Generate coverage status
  run: pwsh -NoProfile -ExecutionPolicy Bypass -File scripts/coverage-status.ps1 -CoverageMinimum 85 -Outcome passed -OutputPath coverage-status.md
```

### What the policy tests verify:

`tests/coverage_status_policy.rs` asserts:
- The workflow file contains `coverage-status.md`.
- The workflow file contains `Generate coverage status`.
- The docs mention `single human-readable summary`.

### What is NOT verified:
- The script actually runs successfully under `pwsh` (no execution test).
- The output `coverage-status.md` is valid Markdown (no schema check).
- The `$runUrl` fallback logic (`local-run` vs. GitHub Actions URL) works.
- The encoding is correct (`-Encoding utf8`).

### Script correctness analysis:

Manual review of `scripts/coverage-status.ps1`:

| Aspect                | Status  | Notes                                           |
|-----------------------|---------|--------------------------------------------------|
| Parameter defaults    | OK      | `CoverageMinimum = 85` matches policy.          |
| GitHub Actions URL    | OK      | Uses standard `GITHUB_*` env vars with fallback. |
| Output format         | OK      | Emits Markdown headings and bullet list.         |
| Encoding              | OK      | Explicit `-Encoding utf8`.                       |
| Error handling        | Minimal | No `try/catch`, no validation of `$OutputPath`.  |
| Idempotency           | OK      | Overwrites `$OutputPath` each run.              |

### Assessment:
The script is simple and correct for its intended purpose. The lack of
execution testing is acceptable because:
- The CI workflow exercises it on every scheduled/manual run.
- If the script were broken, the workflow step would fail and the artifact
  would be missing, which is visible.
- Adding a PowerShell unit test (e.g., Pester) would require a separate test
  framework not currently in the repo's toolchain.

## Summary of Gaps

| Gap                              | Severity | Status                             |
|----------------------------------|----------|-------------------------------------|
| Pre-commit hook auto-install     | Medium   | Documented in CONTRIBUTING.md and hook-setup-guide.md. Manual step is acceptable. |
| Windows llvm-cov limitation      | Low      | Documented in coverage-reporting.md. Acceptable for technical preview. |
| CONTRIBUTING.md Windows gap      | Low      | CONTRIBUTING.md does not mention `-SkipCoverage` for Windows users. Minor docs gap. |
| Coverage percentage badge        | Low      | Workflow status badge already present. Percentage badge is nice-to-have, not blocking. |
| Script execution test            | Low      | CI exercises the script. Manual review confirms correctness. Adding Pester is out of scope. |
