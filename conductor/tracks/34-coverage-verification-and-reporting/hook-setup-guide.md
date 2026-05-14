# Pre-Commit Hook Setup Guide

This track maintains two hook surfaces that enforce the 85% coverage floor:
a checked-in Git hook (`.githooks/pre-commit`) and a `pre-commit` framework
config (`.pre-commit-config.yaml`). Both invoke the same verification script
(`scripts/verify.ps1`) with `-CoverageMinimum 85`.

## Platform Support Matrix

| Platform     | `.githooks/pre-commit`                | `.pre-commit-config.yaml`             |
|--------------|---------------------------------------|---------------------------------------|
| **Linux**    | Requires `pwsh` or `powershell`       | Requires `pwsh` + `pre-commit` pkg    |
| **macOS**    | Requires `pwsh` or `powershell`       | Requires `pwsh` + `pre-commit` pkg    |
| **Windows**  | Requires `pwsh` or `powershell`       | Requires `pwsh` + `pre-commit` pkg    |

Both hook surfaces ultimately run:

```
pwsh -NoProfile -ExecutionPolicy Bypass -File scripts/verify.ps1 -CoverageMinimum 85
```

## Option 1: Git Hook (`core.hooksPath`)

The checked-in `.githooks/pre-commit` script is a plain shell script that
delegates to `pwsh` or `powershell`. It is NOT auto-installed.

### Installation

Run once per clone from the repo root:

```bash
git config core.hooksPath .githooks
```

### Verification

```bash
git config core.hooksPath
# Expected output: .githooks

ls -la .githooks/pre-commit
# Or dry-run:
bash .githooks/pre-commit
```

### What Happens on Commit

1. Shell script runs.
2. Locates `pwsh` or `powershell` (preferring `pwsh`).
3. Calls `scripts/verify.ps1 -CoverageMinimum 85`.
4. Runs: `cargo fmt --check`, `cargo clippy`, `cargo test --locked`,
   then `cargo llvm-cov --branch --fail-under-branches 85`.
5. Any failure blocks the commit.

### Uninstalling

```bash
git config --unset core.hooksPath
```

## Option 2: `pre-commit` Framework

The `.pre-commit-config.yaml` defines a `local` hook for the `pre-commit`
framework (installed separately via `pip install pre-commit`).

### Installation

```bash
pip install pre-commit
pre-commit install
```

### Verification

```bash
pre-commit run --all-files
pre-commit run sourceright-quality-gate --all-files
```

### What Happens on Commit

1. `pre-commit` intercepts `git commit`.
2. Matches `sourceright-quality-gate` (local, `language: system`).
3. Runs `pwsh ... scripts/verify.ps1 -CoverageMinimum 85`.
4. Failure blocks the commit.

### Caveat

`pre-commit install` writes `.git/hooks/pre-commit`, which conflicts with
`git config core.hooksPath .githooks`. Use one or the other, not both.

## Option 3: Manual (No Hook Installation)

```powershell
pwsh -NoProfile -ExecutionPolicy Bypass -File scripts/verify.ps1 -CoverageMinimum 85
```

Skip coverage for faster iteration:

```powershell
pwsh -NoProfile -ExecutionPolicy Bypass -File scripts/verify.ps1 -SkipCoverage
```

## The 85% Coverage Floor

All surfaces assert the same threshold: **85% branch coverage**.

| Surface                         | How asserted                                             |
|---------------------------------|----------------------------------------------------------|
| `.githooks/pre-commit`          | `-CoverageMinimum 85`                                    |
| `.pre-commit-config.yaml`       | `-CoverageMinimum 85` in `entry`                         |
| `.github/workflows/coverage.yml`| `--fail-under-branches 85`                               |
| `scripts/verify.ps1`            | `[int]$CoverageMinimum = 85` default                     |
| `CONTRIBUTING.md`               | "Keep coverage above 85 percent"                         |
| `README.md`                     | "Coverage stays gated above 85 percent"                  |

`tests/coverage_policy.rs` asserts all six surfaces to prevent drift.

## Windows-Specific Notes

- PowerShell is available by default on modern Windows.
- `cargo llvm-cov` may not produce a fresh report on Windows (see
  `docs/src/coverage-reporting.md`). Use `-SkipCoverage` locally and rely on
  CI (`ubuntu-latest`) for the authoritative gate.

## Troubleshooting

| Symptom                              | Likely Cause                       | Fix                                      |
|--------------------------------------|------------------------------------|------------------------------------------|
| `pwsh` not found                     | PowerShell not on PATH             | Install PowerShell 7+                    |
| Hook coverage step fails             | `cargo-llvm-cov` not installed     | `cargo install cargo-llvm-cov --locked`  |
| Hook not firing on commit            | hooksPath not set / pre-commit not installed | `git config core.hooksPath` or `pre-commit run` |
| llvm-cov error on Windows            | Known platform limitation          | Use `-SkipCoverage`; CI gate on ubuntu   |
