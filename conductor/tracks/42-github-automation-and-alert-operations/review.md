# Track 42 — GitHub Automation And Alert Operations — Review

## Status

**Completed** — all owned paths updated, docs mirrored, evidence documented.
Read-only GitHub API checks were run on 2026-05-14; no account-side GitHub
settings or alert states were modified.

## Files reviewed

| Path | Action |
| --- | --- |
| `.github/` directory | Inspected — all files present from Track 64 and earlier |
| `.github/copilot-instructions.md` | Verify exists (Track 64) — present |
| `.github/ISSUE_TEMPLATE/security-remediation.md` | Track 64 created — present, **not overwritten** |
| `.github/ISSUE_TEMPLATE/copilot_security_remediation.yml` | Verify exists (Track 64) — present |
| `.github/workflows/copilot-setup-steps.yml` | Verify exists (Track 64) — present |
| `renovate.json` | Confirmed quiet configuration — monthly grouped PRs, majors manual, no automerge assignees |
| `docs/src/security-automation.md` | Updated — added read-only alert counts, branch-protection/environment observations, installed-app limitation, and enhanced Copilot notes |
| `docs-site/src/content/docs/guides/security-automation.md` | Updated — mirror parity with docs/src |

## Test matrix results

| # | Test | Result |
|---|------|--------|
| 1 | Copilot setup files present | ✅ `.github/copilot-instructions.md`, `ISSUE_TEMPLATE/security-remediation.md`, `ISSUE_TEMPLATE/copilot_security_remediation.yml`, `workflows/copilot-setup-steps.yml` all present |
| 2 | Alert inventory recorded | ✅ Dependabot API returned 12 fixed, zero open alerts; code-scanning API returned one open Scorecard `VulnerabilitiesID` alert and 29 fixed Scorecard `PinnedDependenciesID` alerts |
| 3 | Installed apps inventory recorded | ✅ Local/workflow inventory recorded; full GitHub App enumeration deferred because `GET /user/installations` returned HTTP 403 and `GET /repos/edithatogo/sourceright/installation` returned HTTP 401 |
| 4 | Renovate quietness confirmed | ✅ Monthly schedule, grouped non-major PRs, majors manual, `dependencyDashboard: false`, `assignAutomerge: false`, `prConcurrentLimit: 1`, `prHourlyLimit: 1` |
| 5 | Notification posture documented | ✅ No scheduled issues, no broad bot assignment, no automerge assignees, grouped PRs documented |
| 6 | No overwrite of Track 64 files | ✅ `security-remediation.md` and `copilot_security_remediation.yml` left untouched |

## Changes made

1. **docs/src/security-automation.md** — Added read-only Dependabot/code-scanning counts, branch-protection and environment observations, installed-app API limitation, and Copilot entitlement requirement.
2. **docs-site/src/content/docs/guides/security-automation.md** — Mirrored all doc changes for site parity.
3. **conductor/tracks/42-github-automation-and-alert-operations/metadata.json** — Status remains `"completed"`.
4. **conductor/evidence-ledger.json** — Track 42 evidence level remains `"fixture-backed"` with allowed claims tightened to observed evidence.

## Evidence

- All `.github/` Copilot and issue-template files confirmed present (local inspection).
- `renovate.json` verified quiet-by-default: monthly schedule, grouped minors/patches, majors manual, no dashboard.
- `gh api repos/edithatogo/sourceright/dependabot/alerts` returned 12 fixed alerts and no open alerts on 2026-05-14.
- `gh api repos/edithatogo/sourceright/code-scanning/alerts` returned one open Scorecard alert and 29 fixed Scorecard alerts on 2026-05-14.
- `gh api repos/edithatogo/sourceright/branches/main/protection` returned one required approving review and required checks: `Rust ubuntu-latest`, `Rust macos-latest`, `Rust windows-latest`, `Docs build`, `CodeQL`, `Cargo audit`, and `Dependency review`.
- `gh api repos/edithatogo/sourceright/environments` returned `crates-io`, `github-pages`, and `mcp-registry`; only `github-pages` reported a protection rule through that endpoint.
- Both doc surfaces now contain alert inventory, installed-app limitation, branch/environment observations, notification posture, and Renovate quietness confirmation.

## Deferred / blocked

- Full GitHub App/Marketplace enumeration remains unavailable from this token type: `GET /user/installations` returned HTTP 403 and `GET /repos/edithatogo/sourceright/installation` returned HTTP 401.
- Copilot entitlement/agent assignment, installed Marketplace app confirmation, branch-protection changes, environment reviewer/secret verification, labels, and milestones remain GitHub-side admin tasks outside this repo-local implementation.
- The latest Copilot Setup Steps run could not be fetched by workflow ID in this environment; the workflow itself is listed as active by `gh api repos/edithatogo/sourceright/actions/workflows`.
