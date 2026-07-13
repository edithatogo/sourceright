# arXiv submit-ce Security and Privacy Boundaries

Date: 2026-06-09

## Scope

Local, read-only journal screening for synthetic or exported `submit-ce` submission
metadata. This adapter does not integrate with live arXiv credentials in default
CI.

## Boundaries

| Boundary | Policy |
| --- | --- |
| arXiv state | No paper submission, no metadata mutation, no writeback to `submit-ce` |
| Canonical CSL | `references.csl.json` is never silently overwritten by screening output |
| Verification sidecar | Provider evidence stays in `references.verification.json`; screening reads only |
| Credentials | No arXiv API keys or maintainer tokens in fixtures or default tests |
| PII | Fixtures use synthetic titles, DOIs, and submission IDs only |
| Network | Default CI uses fixture-backed `journal-screen`; live upstream smoke is opt-in |

## Output contract

Screening emits `sourceright.journal_screening.v1` with editorial and author
summaries. It does not assert claim truth or AI-authorship conclusions.

## Upstream engagement

Maintainer issue/PR bodies remain drafts until Track 81 approval. External
submission is blocked in `conductor/submission-requirements.json`.
