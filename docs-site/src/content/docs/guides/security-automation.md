---
title: Security Automation
description: Quiet dependency, supply-chain, and Copilot cloud-agent automation for Sourceright.
---

Sourceright uses quiet-by-default automation for dependency and supply-chain
maintenance. The goal is fast remediation with reviewable pull requests, not a
steady stream of account notifications.

## Active Automation

| Surface | Status | Contract |
| --- | --- | --- |
| Renovate | Active | Groups routine minor, patch, pin, and digest updates by ecosystem: Rust crates, GitHub Actions/MCP release automation, and docs-site Node modules. Major updates stay manual. Dependabot PR creation disabled to prevent duplicate bot noise. |
| Dependabot alerts | Active in GitHub | Alerts identify vulnerable dependencies. Dependency updates are handled by Renovate to avoid duplicate bot PRs. API check on 2026-05-14 returned 12 fixed alerts and no open alerts. |
| Dependency review | Active on pull requests | Blocks or reports risky dependency changes before merge. |
| CodeQL | Active | Uploads SARIF for Rust security analysis. Current code-scanning API output returned no open CodeQL alerts. |
| OpenSSF Scorecard | Active | Reports supply-chain posture through SARIF without changing code. API check on 2026-05-14 returned one open Scorecard alert (`VulnerabilitiesID`) and 29 fixed Scorecard alerts. |
| Release Drafter | Active | Maintains a draft changelog from merged PR labels and refreshes release notes on `main` pushes and `v*.*.*` tag pushes. |
| Copilot cloud agent | Prepared | Repository instructions, setup steps, and a security-remediation issue template are present. Requires GitHub Copilot entitlement at org/repo level to activate `copilot-swe-agent[bot]` assignment. |

## Alert Inventory

Alert checks combine local repository inspection with read-only GitHub API
queries from the `edithatogo` token available on 2026-05-14. These checks did
not modify repository settings or alert state.

### Dependabot alerts

- **Status**: Enabled in GitHub repository settings.
- **Read-only API result**: `gh api repos/edithatogo/sourceright/dependabot/alerts`
  returned 12 fixed alerts and no open alerts on 2026-05-14. All observed
  Dependabot alerts were for `astro` in `docs-site/package.json`.
- **Source**: GitHub's native dependency graph and Dependabot alerting for
  `Cargo.lock` and `docs-site/package-lock.json`.
- **PR creation**: Disabled — all dependency update PRs come through Renovate
  to avoid duplicate bot noise.
- **Local evidence**: `cargo audit` and `npm audit` are run in the security
  workflow. These are not equivalent to Dependabot's advisory database but
  provide offline coverage.
- **Threshold**: Critical and high severity alerts should be remediated within
  the monthly Renovate window. Emergency out-of-band PRs are an option via the
  security-remediation issue template.

### Code-scanning alerts (CodeQL + Scorecard)

- **CodeQL**: Active on push and pull_request to `main`. Results are uploaded
  as SARIF and visible in GitHub Security > Code scanning alerts. The current
  API query returned no open CodeQL alerts.
- **OpenSSF Scorecard**: Active on push and pull_request to `main`. Results are
  uploaded as SARIF for supply-chain posture.
- **Read-only API result**: `gh api repos/edithatogo/sourceright/code-scanning/alerts`
  returned 30 Scorecard alerts on 2026-05-14: one open `VulnerabilitiesID`
  alert and 29 fixed `PinnedDependenciesID` alerts.
- **Remediation path**: CodeQL or Scorecard findings can be filed as
  security-remediation issues and optionally delegated to `copilot-swe-agent[bot]`.

### Secret scanning

- **Status**: GitHub-side setting to verify in the repository Security tab. No
  repo-local configuration is required for public repositories.

### Rust developer tooling

On Windows workstations where the MSVC host linker is unavailable or blocked,
use the local GNU validation wrapper instead of plain `cargo`:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File scripts\verify-local-windows-gnu.ps1
```

The wrapper selects `stable-x86_64-pc-windows-gnu`, writes build artifacts to an
explicit `C:\tmp` target directory, and runs format, clippy, tests, locked
check, plugin validation, benchmark smoke, and the example report smoke. This
keeps validation out of OneDrive-locked `target/` directories and avoids the
MSVC `link.exe` path that can fail before Sourceright code is compiled.

## Installed GitHub Apps And Marketplace Integrations

The current environment token can read repository alerts, workflows, branch
protection, and environments. It cannot enumerate GitHub App installations:
`GET /user/installations` returned HTTP 403 because it requires a token
authorized to a GitHub App, and `GET /repos/edithatogo/sourceright/installation`
returned HTTP 401. The following observations are therefore based on local
inspection plus the GitHub workflow and alert APIs:

| App / Integration | Presence | Source of evidence |
| --- | --- | --- |
| Renovate (Mend) | Prepared locally; app install not API-confirmed | `renovate.json` present; full GitHub App installation inventory requires repository settings or a GitHub App-authorized token. |
| GitHub Copilot cloud agent | Prepared | `.github/copilot-instructions.md`, `.github/workflows/copilot-setup-steps.yml`, `.github/ISSUE_TEMPLATE/copilot_security_remediation.yml` present. Requires org-level Copilot entitlement. |
| Dependabot (GitHub-native) | Confirmed | Dependabot alert API is readable and returned 12 fixed, zero open alerts on 2026-05-14. PR creation is configured through Renovate rather than Dependabot update PRs. |
| CodeQL (GitHub-native) | Confirmed | `.github/workflows/security.yml` runs CodeQL analysis. |
| OpenSSF Scorecard action | Confirmed | `.github/workflows/security.yml` runs Scorecard with SARIF upload; code-scanning API returned one open Scorecard alert on 2026-05-14. |

**Note**: A full inventory requires GitHub UI navigation to **Settings > GitHub
Apps** or **Settings > Integrations > Installed GitHub Apps** with an
admin-level token.

## Copilot Cloud-Agent Path

The repo does not run Copilot from the CLI. GitHub Copilot coding agent is a
GitHub-side feature that works from issues and pull requests when enabled for
the repository or organization.

Repo-local preparation lives in:

- `.github/copilot-instructions.md`
- `.github/workflows/copilot-setup-steps.yml`
- `.github/ISSUE_TEMPLATE/copilot_security_remediation.yml`

To delegate a security task, create a focused "Copilot security remediation"
issue and assign it to `copilot-swe-agent[bot]` if that assignee is available
in GitHub. The setup workflow gives the agent Rust, Node, cargo cache, locked
dependency fetches, and docs-site dependency installation.

Copilot-generated dependency or security PRs must still pass normal branch
protection and CI. The repo does not auto-create scheduled Copilot tasks or
notification-heavy reminder workflows.

## Branch Protection

The following required checks should be configured in the GitHub repository
settings under **Settings > Branches > Branch protection rules** for the
default branch (`main`):

| Check | Required | Notes |
| --- | --- | --- |
| `CI` | Yes | Runs `cargo fmt --check`, `cargo clippy`, `cargo test`, `cargo check --locked`, docs build, and docs-site TypeScript `tsc --noEmit`. |
| `Security` | Yes | Runs CodeQL, Scorecard, Dependabot review, and cargo/npm audit steps. |
| `Pages` | Yes | Docs-site build and deployment check. |
| `release-dry-run` | Recommended | Validates release packaging and runs `scripts/verify-release-surface-refresh.ps1` without publishing. |
| `Coverage` | Recommended | Runs `cargo llvm-cov` summary-only with minimum 85% branch coverage. |
| `Robustness` | Recommended | Runs fixture-backed benchmark and stress tests. |

**Bypass rules**: Allow repository admins and CODEOWNERS to bypass if
necessary, but log all bypass events.

Read-only API check on 2026-05-14 showed branch protection currently requires:
`Rust ubuntu-latest`, `Rust macos-latest`, `Rust windows-latest`, `Docs build`,
`CodeQL`, `Cargo audit`, and `Dependency review`, with one approving review and
admin enforcement disabled. It did not show `Pages`, `Coverage`, `Robustness`,
or release dry-run as required checks. The release dry-run gate now also checks
release-surface evidence boundaries before publication wording can change.
Changing those settings remains an
account-side admin task outside this repo-local track.

### Merge Queue

Enable GitHub Merge Queue for the protected `main` branch once the required
checks above are stable. In **Settings > Branches**, edit the `main` rule,
enable **Require merge queue**, keep required checks aligned with CI, Security,
Quality, Pages, Coverage, Robustness, and release dry-run, and start with small
queue batches while the Rust and docs checks are long-running.

## Labels And Milestones

Repository labels and milestones are GitHub-side settings and are not encoded
in repo files. Verify or create labels for `security`, `dependencies`,
`registry`, `plugin`, `provider`, and `external-proof` work. Milestones should
separate technical-preview hardening from registry/publication readiness so
issue queues do not imply production, legal-compliance, or registry support
before the evidence ledger permits those claims.

## Coverage Reporting

Decision: **No third-party coverage service (Codecov, Coveralls) is used.**

Coverage is computed locally via `cargo llvm-cov --summary-only --branch` in the
`.github/workflows/coverage.yml` workflow on a weekly scheduled trigger
(Tuesdays at 04:37 UTC) and on `workflow_dispatch`. The summary is uploaded as
a CI artifact (`coverage-summary`) rather than posted to a public dashboard.

**Rationale**: Public coverage history is not needed at this stage. The
`--fail-under-branches 85` threshold enforces coverage quality in CI without
exposing intermediate data to external services. If a public coverage badge is
desired later, the existing artifact can be consumed by a badge service or a
new workflow step can upload to Codecov.

## Release Drafter

Release Drafter is configured through `.github/release-drafter.yml` and the
`.github/workflows/release-drafter.yml` workflow. Add `skip-changelog` to PRs
that should be excluded from draft release notes.

## Release Environment Protection

Production releases (`crates.io` publish, MCP registry publish) should be gated
by GitHub Environments configured in the repository settings:

| Environment | Required reviewers | Protected branches | Notes |
| --- | --- | --- | --- |
| `publish-crate` | CODEOWNERS | `main` | Requires CI, Security, and release-dry-run checks. |
| `publish-mcp` | CODEOWNERS | `main` | Requires CI, Security, and release-dry-run checks. |

Environment secrets (`CARGO_REGISTRY_TOKEN`, `MCP_REGISTRY_TOKEN`) are stored
at the environment level, not the repository level. Deployment branches must
match `main` and must pass all required status checks before the environment
gate can be approved.

Read-only API check on 2026-05-14 found three environments: `crates-io`,
`github-pages`, and `mcp-registry`. Only `github-pages` reported a protection
rule through that endpoint. Reviewer and secret configuration still require
admin UI verification.

### OIDC Publishing Trust Gates

Future crates.io and npm publication should prefer OpenID Connect trusted
publishing over long-lived repository secrets where the target registry supports
it. Keep `id-token: write` only on publishing jobs, protect `crates-io`, `npm`,
and `mcp-registry` environments with reviewers, restrict trust to
`edithatogo/sourceright` and `refs/tags/v*.*.*`, and remove long-lived tokens
after trusted publishing is proven.

## CodeRabbit PR Review App

Install the CodeRabbit GitHub App only on this repository first, grant the
minimum PR-review permissions, run it on pull requests only, keep human
approval required, and configure claim-boundary context around
`conductor/requirements.md`, `conductor/evidence-ledger.json`, and
`docs/src/feature-contract-matrix.md`.

## Notification Posture

Repository files cannot change personal GitHub email preferences. This repo
therefore avoids adding scheduled issue creation, broad bot assignment, or
default watcher notifications. Renovate uses grouped PRs, limited concurrency,
and no automerge assignees.

## GitHub App And Plugin Checks

Local `gh extension list` returned no installed GitHub CLI extensions in the
current environment. The GitHub REST endpoints for listing user or repository
app installations were not available to the current token type, so installed
Marketplace apps must be verified in GitHub repository settings if that
inventory is needed.
