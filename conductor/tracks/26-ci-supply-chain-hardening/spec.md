# CI Supply-Chain Hardening Spec

## Goal

Raise CI from basic Rust validation to a release-quality supply-chain and quality gate.

## Scope

- Keep format, clippy, tests, locked checks, docs, security scanning, and dependency review.
- Add dependency policy checks and command smoke tests.
- Add optional coverage, semver, fuzzing, and typo/link validation gates where they are stable enough for CI.
- Preserve pinned GitHub Actions and minimum required permissions.

## Outputs

- Hardened CI workflow coverage.
- Dependency policy configuration.
- Contributor-facing command list for local validation.

## Boundaries

New gates must avoid live provider credentials and default network-dependent product tests. Dependency installation performed by CI is acceptable; runtime tests remain fixture-backed.
