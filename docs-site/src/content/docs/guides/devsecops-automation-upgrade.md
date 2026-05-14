---
title: DevSecOps Automation Upgrade
description: CI/CD, security, Renovate, Release Drafter, and GitHub UI setup guidance.
---

This summary records the CI/CD and code-health automation changes for the Rust
and TypeScript surfaces.

## Structural changes

| Area | Repository change |
| --- | --- |
| Rust CI | `.github/workflows/ci.yml` runs `cargo fmt --all --check`, `cargo clippy --all-targets -- -D warnings`, tests, locked check, command smokes, `cargo-deny`, `cargo-machete`, Taplo, and `typos --config typos.toml`. |
| Rust caching | Rust jobs use `Swatinem/rust-cache@v2` pinned by commit SHA. |
| TypeScript CI | The docs-site job runs `npm run typecheck`, which executes `tsc --noEmit` against the strict Astro `tsconfig.json`. |
| Coverage | `.github/workflows/coverage.yml` runs `cargo llvm-cov --branch --fail-under-branches 85` and uploads `coverage-status.md`. |
| Renovate | `renovate.json` groups routine Rust crates, GitHub Actions/MCP release automation, and docs-site Node modules separately while keeping majors manual. |
| Release notes | `.github/release-drafter.yml` and `.github/workflows/release-drafter.yml` maintain a draft changelog by PR label and refresh on `main` and `v*.*.*` tag pushes. |

## GitHub UI settings

These settings cannot be fully enforced from repository files.

### Merge queue

Enable **Require merge queue** on the protected `main` branch and require the
checks for CI, Security, Quality, Pages, Coverage, Robustness, and release
dry-run. Start with small queue batches because Rust, docs, and coverage checks
are intentionally strict.

### OIDC publishing

Create protected environments for `crates-io`, `npm`, and `mcp-registry`.
Configure registry-side trusted publishers to accept only
`edithatogo/sourceright`, the intended workflow file, protected environments,
and `refs/tags/v*.*.*`. Keep `id-token: write` only on publishing jobs and
remove long-lived publish tokens after trusted publishing is verified.

### CodeRabbit

Install the CodeRabbit GitHub App on `edithatogo/sourceright` only, grant the
minimum pull-request review permissions, keep human approval required, and add
review guidance that treats `conductor/requirements.md`,
`conductor/evidence-ledger.json`, and `docs/src/feature-contract-matrix.md` as
claim-boundary sources.
