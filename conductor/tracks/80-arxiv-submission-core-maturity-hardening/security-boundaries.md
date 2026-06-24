# arXiv submission-core Security and Privacy Boundaries

Date: 2026-06-09

## Scope

Local, read-only journal screening for synthetic or exported legacy
`arxiv-submission-core` event/domain payloads. Default CI does not call live arXiv
systems or legacy submission-core services.

## Boundaries

| Boundary | Policy |
| --- | --- |
| Legacy submission state | No mutation of submission-core domain models or event history |
| arXiv state | No paper submission and no writeback to arXiv systems |
| Canonical CSL | `references.csl.json` is never silently overwritten by screening output |
| Verification sidecar | Provider evidence stays in `references.verification.json`; screening reads only |
| Credentials | No live arXiv or legacy-service credentials in fixtures or default tests |
| PII | Fixtures use synthetic titles, DOIs, event IDs, and submission IDs only |
| Network | Default CI uses fixture-backed `journal-screen`; legacy platform smoke is opt-in |

## Migration safety

Unknown legacy event types degrade to `screened_with_warnings`, not silent
success. Malformed events degrade to `screened_with_errors`. Screening never
claims upstream acceptance of a legacy event mapping.

## Upstream engagement

Maintainer issue/PR bodies remain drafts until Track 81 approval. External
submission is blocked in `conductor/submission-requirements.json`.
