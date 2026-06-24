# arXiv Upstream Submission Packet

## Surfaces

- `arXiv/submit-ce`
- `arXiv/arxiv-submission-core`

## Requirements Evidence

| Surface | Source | Retrieved | Local impact |
| --- | --- | --- | --- |
| `arXiv/submit-ce` | <https://github.com/arXiv/submit-ce> | 2026-05-18 | Contribution must be aligned to current submission-platform code, tests, and maintainer expectations before any issue or PR. |
| `arXiv/arxiv-submission-core` | <https://github.com/arXiv/arxiv-submission-core> and <https://arxiv.github.io/arxiv-submission-core/> | 2026-05-18 | Legacy submission work is event/domain-model oriented and needs migration-safe compatibility evidence. |

## Local Gates

- Track 78 requirements matrix complete.
- Track 79 current-platform fixture breadth, drift checks (`schema-drift-check-2026-06-09.md`), security boundaries, and no-writeback proof complete.
- Track 80 legacy event/domain fixture breadth, migration mapping checks (`migration-mapping-check-2026-06-09.md`), security boundaries, and no-writeback proof complete.
- Track 81 readiness review complete.

## Live evidence (2026-06-09)

| Surface | URL | Status |
| --- | --- | --- |
| `arXiv/submit-ce` | https://github.com/arXiv/submit-ce/issues/72 | submitted |
| `arXiv/arxiv-submission-core` | https://github.com/arXiv/arxiv-submission-core/issues/88 | submitted |

Recorded in `conductor/submission-packets/live-evidence.json`. Not accepted until
maintainer response or merged upstream work exists.

## Blockers

None.

## Draft `submit-ce` Issue/PR Body

Drafted in
`conductor/tracks/79-arxiv-submit-ce-maturity-hardening/evidence-packet.md`.

## Draft `arxiv-submission-core` Issue/PR Body

Drafted in
`conductor/tracks/80-arxiv-submission-core-maturity-hardening/evidence-packet.md`.

## Approval Gate

No arXiv issue or pull request without explicit approval after Tracks 78-80 are
green. Readiness review and submission drafts live in
`conductor/tracks/81-arxiv-upstream-submission-and-acceptance/`.
