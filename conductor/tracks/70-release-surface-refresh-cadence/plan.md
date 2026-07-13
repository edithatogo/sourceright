# Track 70 Plan

## 1. Discover With Subagents Where Useful

Skipped for this small additive slice. Existing Track 69 evidence, release-status pages, and host-packaging contracts are sufficient local source material.

## 2. Lock Spec, Plan, And Test Matrix

- [x] Add `metadata.json`, `spec.md`, `plan.md`, and `test-matrix.md`.
- [x] Register Track 70 in `conductor/tracks.md`.

## 3. Implement The Smallest Owned-Path Slice

- [x] Add a release-surface refresh guide to source docs.
- [x] Mirror the guide into the Starlight docs tree.
- [x] Link the guide from the mdBook summary and release-status pages.
- [x] Add the verifier to release runbook and publishing docs.
- [x] Add the verifier to the public feature contract matrix and Starlight mirror.
- [x] Add the verifier to security automation and DevSecOps docs.
- [x] Add a local verification script for release-surface evidence boundaries.
- [x] Wire the verification script into `scripts/release-check.ps1`.
- [x] Wire the verification script into `.github/workflows/release-dry-run.yml`.
- [x] Add the release-surface verifier to the PR template claim checklist.
- [x] Add Track 70 to `conductor/evidence-ledger.json`.
- [x] Add Track 70 to implementation order and release channel surfaces.
- [x] Add a deterministic policy test for the guide and claim boundaries.

## 4. Run Targeted Checks

- [x] `cargo fmt --check`
- [x] `cargo +stable-x86_64-pc-windows-gnu clippy --all-targets --target-dir C:\tmp\sourceright-target-track70 -- -D warnings`
- [x] `cargo +stable-x86_64-pc-windows-gnu test --target-dir C:\tmp\sourceright-target-track70`
- [x] `cargo +stable-x86_64-pc-windows-gnu check --locked --target-dir C:\tmp\sourceright-target-track70`
- [x] `cargo +stable-x86_64-pc-windows-gnu test --test public_surface_refresh_policy --target-dir C:\tmp\sourceright-target-track70`
- [x] `cargo +stable-x86_64-pc-windows-gnu test --test docs_site_parity --target-dir C:\tmp\sourceright-target-track70`
- [x] `npm run build` from `docs-site/`
- [x] `powershell -NoProfile -ExecutionPolicy Bypass -File scripts\verify-release-surface-refresh.ps1`
- [x] `powershell -NoProfile -ExecutionPolicy Bypass -File scripts\release-check.ps1 -ReleaseTag v0.1.20`
- [x] `cargo +stable-x86_64-pc-windows-gnu test --test quality_policy release_dry_run_checks_public_api_compatibility --target-dir C:\tmp\sourceright-target-track70`
- [x] `cargo +stable-x86_64-pc-windows-gnu test --test publish_policy --test release_status_policy --target-dir C:\tmp\sourceright-target-track70`
- [x] `cargo +stable-x86_64-pc-windows-gnu test --test quality_policy pull_request_template_keeps_release_surface_claim_gate_visible --target-dir C:\tmp\sourceright-target-track70`
- [x] `Get-Content conductor\evidence-ledger.json | ConvertFrom-Json`
- [x] `cargo +stable-x86_64-pc-windows-gnu test --test requirements_contract_policy --target-dir C:\tmp\sourceright-target-track70`
- [x] `cargo +stable-x86_64-pc-windows-gnu test --test quality_policy release_dry_run_checks_public_api_compatibility --target-dir C:\tmp\sourceright-target-track70`
- [x] `cargo +stable-x86_64-pc-windows-gnu test --test quality_policy release_dry_run_checks_public_api_compatibility --target-dir C:\tmp\sourceright-target-track70b`
- [x] `cargo +stable-x86_64-pc-windows-gnu test --test docs_site_parity --target-dir C:\tmp\sourceright-target-track70b`
- [x] `cargo +stable-x86_64-pc-windows-gnu clippy --all-targets --target-dir C:\tmp\sourceright-target-track70b -- -D warnings`
- [x] `cargo +stable-x86_64-pc-windows-gnu check --locked --target-dir C:\tmp\sourceright-target-track70b`
- [x] `cargo +stable-x86_64-pc-windows-gnu test --target-dir C:\tmp\sourceright-target-track70b`
- [x] `git diff --check` (only Windows line-ending warning for `scripts/release-check.ps1`)

## 5. Run `$conductor-review`

Review focus: stale-claim wording, docs parity, and whether any prepared surface is described as accepted.

## 6. Apply Local Review Fixes Automatically

Apply wording, parity, and test fixes locally before closing the track.

## 7. Progress Only After Findings Are Fixed Or Deferred

Any live external listing uncertainty remains deferred to opt-in live verification, not default CI.
