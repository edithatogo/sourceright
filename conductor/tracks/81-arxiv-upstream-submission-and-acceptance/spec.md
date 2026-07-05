# Track 81 — arXiv upstream submission and acceptance

## Goal

Create and track arXiv upstream issues/PRs only after requirements and hardening maturity, stability, and testing gates pass and approval is recorded.

## User outcome

arXiv repositories have submitted Sourceright integration proposals for local screening.

## Scope

- Issue-first submissions to `arXiv/submit-ce` and `arXiv/arxiv-submission-core`.
- Maintainer follow-up and response monitoring.
- Acceptance monitoring and live-evidence recording.
- Recording maintainer comments, PR merges, or equivalent acceptance URLs.
- Rollback documentation if a submission is withdrawn.

## Out of scope

- Code changes to arXiv repositories (no direct commits, PRs, or patches).
- arXiv API access (read-only GitHub Issues API is the interface).
- Hosted screening service or arXiv-side infrastructure.
- Non-arXiv journal platforms (OJS/PKP, institutional repositories, etc.).

## Data contracts

- **GitHub Issues API**: Read-only issue/comment retrieval for monitor updates.
- **live-evidence.json schema**: `conductor/submission-packets/live-evidence.template.json` defines the evidence entry structure for `submitted`/`accepted` states.
- **Approval-record format**: `approval-record-YYYY-MM-DD.md` documents the explicit go-ahead before external submission, referencing Track 78/79/80 readiness.

## Claim boundary

> **"submitted" is not "accepted"** until maintainer acceptance evidence exists (merged PR, maintainer acceptance comment, or equivalent public acceptance URL).

All evidence docs in this track must include the disclaimer: *fixture-backed, not arXiv-reviewed*.

## Evidence level target

**publicly-accepted** — the highest evidence level in `evidence-ledger.json`'s `evidence_levels` enum.

Achieved when at least one maintainer acceptance URL is recorded.

## Parallelization plan

- **`submit-ce` monitoring** and **`submission-core` monitoring** can run in parallel because the two repos are independent GitHub repositories and maintainer response cycles do not conflict.
- Within each repo, follow sequential: issue-filed → maintainer-responds → acceptance-recorded (or closed-without-acceptance).
