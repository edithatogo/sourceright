# Track 70 Test Matrix

| Scenario | Acceptance | Evidence | Default-CI or opt-in-live |
| --- | --- | --- | --- |
| Refresh guide exists | Source docs and docs-site mirror describe the cadence. | `tests/public_surface_refresh_policy.rs` | Default-CI |
| Accepted surfaces require evidence | Guide requires public URL, version or artifact id, date, and install metadata before accepted claims. | `tests/public_surface_refresh_policy.rs` | Default-CI |
| Prepared/deferred surfaces stay bounded | Smithery, Glama, Zotero, OJS, VS Code, Word, LibreOffice, and package-manager rows remain non-accepted unless proof is recorded. | `tests/public_surface_refresh_policy.rs` | Default-CI |
| Local refresh verification | Script checks accepted/prepared/deferred evidence markers and docs-site parity without network access. | `scripts/verify-release-surface-refresh.ps1` and `tests/public_surface_refresh_policy.rs` | Default-CI |
| Release checklist integration | Release checklist invokes the release-surface verifier before printing remaining release gates. | `scripts/release-check.ps1` and `tests/public_surface_refresh_policy.rs` | Default-CI |
| Release dry-run integration | Release dry-run workflow invokes the release-surface verifier and triggers on relevant evidence/doc changes. | `.github/workflows/release-dry-run.yml`, `tests/quality_policy.rs`, and `tests/public_surface_refresh_policy.rs` | Default-CI |
| Release docs integration | Release runbook and publishing docs mention the release-surface verifier in source and Starlight docs. | `tests/publish_policy.rs`, `tests/release_status_policy.rs`, and docs build | Default-CI |
| Feature contract parity | Feature contract matrix describes the fixture-backed release-surface refresh cadence in source and Starlight docs. | `tests/requirements_contract_policy.rs` and docs build | Default-CI |
| PR claim checklist | Pull request template prompts maintainers to run the verifier when release-surface wording changes. | `.github/pull_request_template.md` and `tests/quality_policy.rs` | Default-CI |
| Evidence ledger | Track 70 has allowed claims and blockers recorded in the evidence ledger. | `conductor/evidence-ledger.json` and `tests/public_surface_refresh_policy.rs` | Default-CI |
| Release governance sequencing | Implementation-order and release-channel docs include Track 70 as the release-surface wording gate. | `conductor/implementation-order.md`, `conductor/release-channels.md`, and `tests/requirements_contract_policy.rs` | Default-CI |
| Security automation docs | Security automation and DevSecOps docs describe release dry-run as packaging plus release-surface evidence validation. | `docs/src/security-automation.md`, `docs-site/src/content/docs/guides/security-automation.md`, `docs/src/devsecops-automation-upgrade.md`, `docs-site/src/content/docs/guides/devsecops-automation-upgrade.md`, and `tests/quality_policy.rs` | Default-CI |
| Live external refresh | Maintainer may verify external listings and update evidence rows. | Manual evidence URLs and dates in Track 69/release-status docs | opt-in-live |
| `$conductor-review` gate | Review checks stale-claim wording and docs parity before closure. | Track plan and review notes | Default-CI |
