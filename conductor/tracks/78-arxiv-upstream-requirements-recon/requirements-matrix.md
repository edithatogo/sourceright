# arXiv Upstream Requirements Matrix

Date: 2026-06-09 (matrix frozen 2026-05-18; closure review 2026-06-09)

## `arXiv/submit-ce`

| Requirement | Sourceright impact | Status |
| --- | --- | --- |
| Repository-first contribution path | Prepare an issue/PR body against `arXiv/submit-ce`; do not submit until approval. | Contracted |
| Current-platform screening only | Keep output as `sourceright.journal_screening.v1`; do not submit papers or mutate arXiv state. | Contracted |
| Fixture and schema evidence | Use synthetic source-bundle fixtures and schema checks before upstream engagement. | Contracted |
| Default CI safety | No live credentials or arXiv writes in default tests. | Contracted |
| Maintainer burden | Provide rollback, security boundary, and local test commands in the issue/PR. | Contracted |

## `arXiv/arxiv-submission-core`

| Requirement | Sourceright impact | Status |
| --- | --- | --- |
| Legacy event/domain compatibility | Map legacy submission events into journal-screening output without changing legacy state. | Contracted |
| Migration safety | Unknown legacy events must degrade to review/warning behavior, not silent success. | Contracted |
| Fixture breadth | Cover accepted, held, rejected, malformed, and unknown-event cases before submission. | Contracted |
| Default CI safety | No live credentials or arXiv writes in default tests. | Contracted |
| Maintainer burden | Provide compatibility matrix, security boundary, and local test commands. | Contracted |

## Submission Path Decision

Use issue-first upstream engagement for both repositories unless maintainers
request a pull request. The first issue should ask whether the maintainers want
an external integration, an in-repository module, or no upstream change.

## Blockers

- Maintainer-ready issue/PR bodies must be approved before browser submission.
- No acceptance claim is allowed without public maintainer evidence.
