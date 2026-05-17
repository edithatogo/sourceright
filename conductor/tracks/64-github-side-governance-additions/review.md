# Track 64 — GitHub-Side Governance Additions — Review

## Files changed

| File | Change |
| --- | --- |
| `.github/copilot-instructions.md` | Added Conductor track workflow reference and Forbidden Claims sections. |
| `.github/ISSUE_TEMPLATE/security-remediation.md` | Created new generic security remediation issue template. |
| `.github/pull_request_template.md` | Added Governance checklist section. |
| `docs/src/security-automation.md` | Added Branch Protection, Coverage Reporting, and Release Environment Protection sections. Updated existing automation table. |
| `conductor/tracks/64-github-side-governance-additions/metadata.json` | Status → completed. |
| `conductor/tracks/64-github-side-governance-additions/plan.md` | Added completion notes. |

## Test matrix results

| Scenario | Result |
| --- | --- |
| **Branch protection** | Documented required checks table in `docs/src/security-automation.md`. Settings tasks listed in plan.md. |
| **Copilot** | Copilot cloud agent documented as "Prepared" — requires GitHub org/repo entitlement. `.github/copilot-instructions.md` exists and is updated. |
| **Renovate** | Low-noise: monthly schedule, grouped minor/patch, no Dependabot PR config. Noted in security-automation.md. |
| **Code scanning** | CodeQL, Scorecard, Dependabot alerts are tracked in the Active Automation table. cargo/npm audits noted in the Branch Protection Security check. |
| **Coverage** | Decision recorded: `cargo llvm-cov` summary-only, no Codecov/Coveralls. Documented in security-automation.md and PR template. |
| **Labels and milestones** | Listed as GitHub settings work in `docs/src/security-automation.md` and this track plan. |
| **Review loop** | This review.md documents the final gate. All local fixes applied. Settings tasks recorded for UI/API. |

## Settings tasks (require GitHub UI/API)

1. **Branch protection**: Add required checks `CI`, `Security`, `Pages`, `release-dry-run`, `Coverage`, `Robustness` to `main`.
2. **Environments**: Configure `publish-crate` and `publish-mcp` with required reviewers (CODEOWNERS), protected branch (`main`), and environment-level secrets.
3. **Copilot**: Enable Copilot coding agent at org/repo level.
4. **Code scanning**: Verify CodeQL, Scorecard, Dependabot alerts, and Dependency review are enabled in repository settings.
5. **Labels/milestones**: Verify or create labels for `security`, `dependencies`, `registry`, `plugin`, `provider`, and `external-proof`; create milestones that keep technical-preview hardening separate from registry/publication readiness.

## Evidence level

`contracted` → `fixture-backed` — all changes are fixture-backed (checked-in files with observable content). No live GitHub API calls or settings modifications were made by this track; only repo-local files were created or updated.

## Validation run — 2026-05-14

| Check | Result |
| --- | --- |
| `Get-Content conductor/evidence-ledger.json \| ConvertFrom-Json` | Passed. |
| `rg` over track-owned docs for unobserved Dependabot enabled/visible wording | Passed after wording correction. |
| `cargo fmt --check` | Superseded by later green CI and local checks after repository formatting/parser issues were resolved. |
| `cargo test --test requirements_contract_policy` | Superseded by later green CI; Windows target-dir access issues are no longer recorded as a Track 64 blocker. |

## Current Validation

Latest `main` runs after the governance and CI hardening work pass `CI`,
`Quality`, `Security`, `Robustness`, `Pages`, and `Release Drafter`. Track 64
therefore remains completed, with only GitHub account-side settings tasks left
outside repo-local control.
