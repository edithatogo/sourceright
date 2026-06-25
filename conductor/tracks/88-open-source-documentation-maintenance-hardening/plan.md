# Track 88 Plan

## Phase 1: Confirm Audit Baseline

- Re-read README, docs installation, release status, release notes,
  contributing, security, code of conduct, issue templates, pull request
  template, workflows, and Conductor track status.
- Record every stale or inconsistent public statement with file path, line
  reference, replacement intent, and evidence source.
- Confirm which community-health files already exist and which are missing.
- Acceptance: the audit table is specific enough that an implementer can edit
  without re-discovering the problem.

## Phase 2: README and Docs Front Door

- Move install, docs-site, and quick trial guidance into the top README flow.
- Reconcile README feature status with completed Conductor tracks and release
  evidence.
- Normalize installation docs, release notes, and release status so they do
  not disagree about crates.io, docs.rs, GitHub Releases, MCP Registry, or
  remaining hardening work.
- Acceptance: a new user can understand maturity, install path, and first
  command from README plus one docs link.

## Phase 3: Community Health and Governance

- Add or strengthen support, changelog, governance, maintainer, security, and
  conduct documentation.
- Define security reporting scope, supported versions, response expectations,
  non-vulnerability support routing, conduct scope, and enforcement process.
- Explain how Conductor tracks map to public governance and contribution flow
  without making internal track details mandatory user reading.
- Acceptance: GitHub community-health expectations are covered and routed.

## Phase 4: Issue and Pull Request Workflow

- Upgrade bug reports to require version, operating system, command, minimal
  repro workspace, affected surface, privacy confirmation, and expected versus
  actual behavior.
- Upgrade feature requests to capture user value, evidence boundary, contract
  boundary, affected public interface, and maintenance cost.
- Add or update issue-template configuration so users are routed to support,
  security, discussions, or feature requests correctly.
- Acceptance: new issues collect enough information for maintainers to triage
  without repeated clarification.

## Phase 5: Maintenance Automation and Status

- Reconcile README badges with workflows that matter for PRs, docs, security,
  pages, release, coverage, and publication.
- Review docs workflow pinning, Node version, install mode, and submodule
  policy against main CI.
- Verify Dependabot or equivalent update policy, stale issue policy, release
  checklist, changelog discipline, and branch-protection assumptions.
- Acceptance: public maintenance docs match the actual automation and do not
  overstate what CI gates.

## Phase 6: Policy Tests and Validation

- Add policy checks for stale wording such as prepared-versus-accepted release
  status and remaining-hardening-versus-completed-track drift.
- Add markdownlint and link checks for changed docs where practical.
- Run `cargo fmt --check`, `cargo check --locked`, and
  `cargo clippy --all-targets -- -D warnings` where practical.
- Acceptance: documentation changes are guarded by repeatable checks rather
  than one-off review.

## Deferred Work

- Live GitHub branch-protection changes are deferred because they require
  repository admin access.
- Publishing a release is deferred until documentation and validation changes
  are complete.
- Product roadmap expansion is deferred unless the documentation audit reveals
  a public claim that requires product work to support it.
