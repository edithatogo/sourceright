# DevSecOps Automation Upgrade

This summary records the CI/CD and code-health automation changes for the Rust
and TypeScript surfaces.

## Structural Changes

| Area | Repository change |
| --- | --- |
| Rust CI | `.github/workflows/ci.yml` runs `cargo fmt --all --check`, `cargo clippy --all-targets -- -D warnings`, tests, locked check, command smokes, `cargo-deny`, `cargo-machete`, Taplo, and `typos --config typos.toml`. |
| Rust caching | Rust jobs use `Swatinem/rust-cache@v2` pinned by commit SHA. |
| TypeScript CI | The docs-site job runs `npm run typecheck`, which executes `tsc --noEmit` against the strict Astro `tsconfig.json`. |
| Coverage | `.github/workflows/coverage.yml` runs `cargo llvm-cov --fail-under-lines 85` and uploads `coverage-status.md`. |
| Renovate | `renovate.json` groups routine Rust crates, GitHub Actions/MCP release automation, and docs-site Node modules separately while keeping majors manual. |
| Release notes | `.github/release-drafter.yml` and `.github/workflows/release-drafter.yml` maintain a draft changelog by PR label and refresh on `main` and `v*.*.*` tag pushes. |

## GitHub UI Settings

These settings cannot be fully enforced from repository files.

### Merge Queue

1. Open **Settings > Branches**.
2. Edit the `main` branch protection rule.
3. Enable **Require merge queue**.
4. Require the checks for CI, Security, Quality, Pages, Coverage, Robustness,
   and release dry-run.
5. Start with small queue batches because Rust, docs, and coverage checks are
   intentionally strict.

### OIDC Publishing

1. Create protected environments for `crates-io`, `npm`, and `mcp-registry`.
2. Require CODEOWNER or maintainer approval for each publishing environment.
3. Configure registry-side trusted publishers to accept only
   `edithatogo/sourceright`, the intended workflow file, protected
   environments, and `refs/tags/v*.*.*`.
4. Keep `id-token: write` only on publishing jobs.
5. Remove long-lived publish tokens once trusted publishing is verified.

### CodeRabbit

1. Install the CodeRabbit GitHub App on `edithatogo/sourceright` only.
2. Grant the minimum permissions needed for pull-request review comments.
3. Configure it for PRs only and ignore draft PRs if supported.
4. Keep human approval required; CodeRabbit should not be a required sole
   approver.
5. Add repository review guidance that treats `conductor/requirements.md`,
   `conductor/evidence-ledger.json`, and `docs/src/feature-contract-matrix.md`
   as claim-boundary sources.
