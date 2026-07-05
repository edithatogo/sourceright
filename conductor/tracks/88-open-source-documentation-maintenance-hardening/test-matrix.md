# Track 88 Test Matrix

## Static Documentation Checks

- `markdownlint-cli2 conductor/tracks/88-open-source-documentation-maintenance-hardening/*.md`
  must pass for the track artifacts.
- Repository documentation changed by later phases should pass the same
  markdownlint profile used elsewhere in the repo.
- Links added to README, docs, and community-health files should be checked
  with the repo's existing link-check process or a documented manual pass.

## Community-Health Checks

- Confirm README, license, contributing, code of conduct, security, support,
  governance, maintainer, changelog, issue templates, and pull request template
  exist or are explicitly deferred with rationale.
- Confirm support and security routes do not conflict.
- Confirm issue templates route vulnerabilities away from public bug reports.

## Consistency Checks

- Compare README feature status against Conductor completed-track status.
- Compare installation docs against release status for crates.io, docs.rs,
  GitHub Releases, and registry publication wording.
- Compare release notes against current release-status evidence.
- Compare README badges against workflow files and actual gating policy.

## Contributor Dry Runs

- New user dry run: find install command, run first command, find full docs.
- Bug reporter dry run: file a complete bug report without maintainer
  follow-up for missing version, OS, command, repro, or privacy data.
- Feature requester dry run: classify evidence boundary, affected interface,
  and maintenance cost.
- Maintainer dry run: identify release checklist, changelog update path,
  dependency update policy, and branch-protection assumptions.

## Repository Checks

- `git status --short` before and after implementation.
- `cargo fmt --check`.
- `cargo check --locked`.
- `cargo clippy --all-targets -- -D warnings`.
- Targeted policy tests for documentation drift once implemented.
