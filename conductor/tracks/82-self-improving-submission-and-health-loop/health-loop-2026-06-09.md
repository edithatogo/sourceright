# Track 82 — Submission and Repo-Health Loop

Date: 2026-06-09

## Loop

```text
submission-requirements.json + manifest.json
        ↓
verify-submission-readiness.ps1 (local / CI)
        ↓
submission_contracts_policy (cargo test)
        ↓
maintainer updates inventory / packets / track evidence
```

## Verified on 2026-06-09

| Check | Result | Command / artifact |
| --- | --- | --- |
| 14 submission surfaces inventoried | Pass | `verify-submission-readiness.ps1` |
| 7 submission packets indexed | Pass | `manifest.json` |
| Repo health target ≥ 9.5 | Pass | `repo_health_target: 9.5` |
| Blocked surfaces reported | Pass | smithery, glama, ojs-pkp, arxiv-* |
| CI workflow wired | Pass | `.github/workflows/submission-readiness.yml` |
| Policy tests bind controls | Pass | `submission_contracts_policy` |

## Blocked surface summary

Surfaces with inventory blockers remain blocked until owning tracks record URL
evidence and clear blockers before `submission_ready` gates flip.

## Self-improvement rules

1. Add inventory rows before claiming a new host surface.
2. Add or update submission packets when local validation steps change.
3. Run the readiness verifier before changing external-submission wording.
4. Do not add host-specific agents or skills without a stable package contract
   (`agent-workflow.md`).
5. Do not raise `submission_ready`, `submitted`, or `publicly_accepted` while
   blockers remain.

## Deferred

- Subjective live repo-health scoring from external services
- Automatic promotion of new agents/skills per host
