# GitHub-Side Governance Additions Plan

1. Inventory GitHub settings from API or UI.
2. Add repo-local files only where useful: PR checklist, issue templates,
   CODEOWNERS updates, workflow permissions, release environment docs.
3. Avoid duplicate bot noise: do not add Dependabot PR config if Renovate owns
   update PRs.
4. Decide on Codecov/Coveralls only if public coverage history is wanted.
5. Update security automation docs and track evidence.
6. Run `$conductor-review`.
7. Apply local fixes automatically; record settings requiring UI/admin action.

## Completion

All local file changes for track 64 are complete.

**Added/Updated files**:
- `.github/copilot-instructions.md` — added Conductor track workflow reference
  and Forbidden Claims section.
- `.github/ISSUE_TEMPLATE/security-remediation.md` — new generic security fix
  template with validation and governance checklist.
- `.github/pull_request_template.md` — added Governance checklist section with
  branch protection, code scanning, Renovate, coverage, and evidence ledger items.
- `docs/src/security-automation.md` — added Branch Protection table, Coverage
  Reporting decision (no Codecov/Coveralls), and Release Environment Protection
  sections. Updated existing tables with Duplicate bot avoidance note.

**Settings tasks (require GitHub UI/API — not automated in repo files)**:
- Branch protection rules: configure required checks `CI`, `Security`, `Pages`,
  `release-dry-run`, `Coverage`, `Robustness` on `main`.
- Environments `publish-crate` and `publish-mcp`: set required reviewers,
  protected branches, and environment-level secrets.
- Enable Copilot coding agent at org/repo level (if entitled).
- Verify Dependabot alerts, CodeQL, Scorecard, and Dependency review are
  enabled in repository settings.
- Verify or create labels for `security`, `dependencies`, `registry`,
  `plugin`, `provider`, and `external-proof`; create milestones that separate
  technical-preview hardening from registry/publication readiness.

**Evidence level**: `contracted` → `fixture-backed` (all changes are
fixture-backed — checked-in files with observable content).
