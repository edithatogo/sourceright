# Track 70 Release Surface Refresh Cadence

## Goal

Keep accepted, prepared, deferred, and not-applicable release surfaces auditable after the initial marketplace evidence pass.

## User Outcome

A maintainer can refresh public release-surface evidence without guessing which listings are accepted, which are only prepared, and which remain deliberately deferred.

## Scope

- Document a local-first refresh cadence for release surfaces.
- Tie the cadence to the existing marketplace evidence model and release-status pages.
- Require explicit verification dates and evidence URLs before any row moves to `accepted`.
- Keep client configuration, package skeletons, and local manifests separate from marketplace acceptance.
- Add a local verification script for the evidence table, release-status mirrors, and refresh guide.
- Add a policy test so the refresh guide and docs-site mirror stay present.

## Out Of Scope

- Submitting to public marketplaces or package managers.
- Claiming Smithery, Glama, Zotero, OJS, VS Code, Word, LibreOffice, Homebrew, Scoop, winget, npm, or PyPI acceptance without public proof.
- Running live registry checks in default CI.
- Changing the Rust verification engine.

## Data Contracts

- `conductor/tracks/69-marketplace-submission-evidence/marketplace-evidence.md` remains the detailed evidence table.
- `docs/src/release-status.md` remains the human-readable release status page.
- `docs/src/release-surface-refresh.md` documents the refresh workflow and escalation rules.
- `docs/src/release-runbook.md` and `docs/src/publishing.md` reference the verifier in the release sequence.
- `docs-site/src/content/docs/guides/release-surface-refresh.md` mirrors the public docs guide.
- `docs-site/src/content/docs/guides/release-runbook.md` and `docs-site/src/content/docs/guides/publishing.md` mirror the release sequence.
- `docs/src/feature-contract-matrix.md` and the Starlight mirror describe the fixture-backed refresh cadence.
- Security automation and DevSecOps docs describe release dry-run as both package and release-surface evidence validation.
- `scripts/verify-release-surface-refresh.ps1` performs deterministic local evidence checks without network access.
- `scripts/release-check.ps1` invokes the release-surface verifier as part of the release checklist.
- `.github/workflows/release-dry-run.yml` runs the release-surface verifier in CI-style release validation.
- `.github/pull_request_template.md` keeps the verifier visible during PR review for release-surface wording changes.
- `conductor/evidence-ledger.json` records Track 70 allowed claims and live-promotion blockers.
- `conductor/implementation-order.md` and `conductor/release-channels.md` sequence Track 70 as the release-surface wording gate.

## Claim Boundary

Prepared metadata is not public acceptance. A release surface can move to `accepted` only after a public URL, version or artifact id, verification date, and install metadata are recorded.

## Evidence Level Target

Contracted documentation plus deterministic policy tests. Live checks are opt-in and must not be required for default CI.

## Parallelization Plan

Future work can split by release-surface family: public registries, MCP client configurations, citation-manager packages, editor and office-suite extensions, and package-manager wrappers.
