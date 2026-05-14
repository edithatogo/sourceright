---
name: Security remediation
about: File a security or dependency remediation issue
title: "security: "
labels: ["security"]
assignees: ""
---

## Finding

<!-- Paste the Dependabot alert, CodeQL finding, Scorecard recommendation,
cargo audit / npm audit output, or Renovate PR description. -->

## Required change

<!-- State the exact dependency, action, image, workflow permission, or policy
surface to change. Keep the fix narrowly scoped. -->

## Constraints

<!-- Note version/API limits, market-readiness wording restrictions, or repo
boundaries that apply. -->
- Provider evidence must stay separate from canonical CSL.
- CI, security policy, and release provenance checks must not be weakened.
- Do not add scheduled or notification-heavy automation.

## Validation

- [ ] `cargo fmt --check`
- [ ] `cargo clippy --all-targets -- -D warnings`
- [ ] `cargo test`
- [ ] `cargo check --locked`
- If docs-site files change: `npm audit --omit=dev` and `npm run build`
- If workflow files change: verify least-privilege permissions and SHA-pinned
  action versions.

## Governance checklist

- [ ] Branch protection rules are preserved or documented.
- [ ] CODEOWNERS review is not bypassed.
- [ ] Evidence-level claims in `conductor/evidence-ledger.json` are accurate.
- [ ] `$conductor-review` was run for the owning track or an explicit reason is
  recorded.
