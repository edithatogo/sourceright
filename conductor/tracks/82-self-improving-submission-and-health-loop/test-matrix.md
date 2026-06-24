| Scenario | Acceptance | Evidence | Default-CI or opt-in-live |
| --- | --- | --- | --- |
| Inventory exists | All requested submission surfaces have machine-readable rows. | `conductor/submission-requirements.json`, `tests/submission_contracts_policy.rs` | Default-CI |
| Readiness verifier | Script fails on missing surfaces, missing gates, or unsafe approval state. | `scripts/verify-submission-readiness.ps1` | Default-CI |
| CI workflow | Submission-contract changes run readiness validation. | `.github/workflows/submission-readiness.yml` | Default-CI |
| Repo health target | Inventory records `repo_health_target >= 9.5`. | Policy test | Default-CI |
| Verifier hardening | Native command failures in the Windows GNU verifier stop the script. | `scripts/verify-local-windows-gnu.ps1`, policy test | Default-CI |
