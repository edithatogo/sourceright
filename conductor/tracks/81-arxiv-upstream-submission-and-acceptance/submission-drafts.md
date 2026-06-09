# arXiv Upstream Submission Drafts

Date: 2026-06-09 (issues filed)

## Current state

Issue-first submissions filed on 2026-06-09 after explicit approval
(`approval-record-2026-06-09.md`). This is **submitted**, not **accepted**.

| Repository | Issue | Date |
| --- | --- | --- |
| `arXiv/submit-ce` | https://github.com/arXiv/submit-ce/issues/72 | 2026-06-09 |
| `arXiv/arxiv-submission-core` | https://github.com/arXiv/arxiv-submission-core/issues/88 | 2026-06-09 |

Draft bodies and local evidence remain in Track 79/80 evidence packets.
Live URLs: `conductor/submission-packets/live-evidence.json`.

## Approval checklist

- [x] Track 78 requirements matrix exists.
- [x] Track 79 evidence packet and drift/security artifacts exist.
- [x] Track 80 evidence packet and migration/security artifacts exist.
- [x] Track 81 readiness review recorded.
- [x] Explicit approval for external upstream submission recorded
  (`approval-record-2026-06-09.md`).
- [x] Upstream issue URL recorded after submission.

## `arXiv/submit-ce`

- Title and body: Track 79 `evidence-packet.md` — Maintainer Draft section
- Compatibility: `submit-ce-contract-snapshot.json`, variant fixtures
- Security: `security-boundaries.md` (no arXiv writes, no CSL mutation)
- **Submitted** — https://github.com/arXiv/submit-ce/issues/72
- Follow-up with local smoke + integration hook proposal: https://github.com/arXiv/submit-ce/issues/72#issuecomment-4662113521

## `arXiv/arxiv-submission-core`

- Title and body: Track 80 `evidence-packet.md` — Maintainer Draft section
- Compatibility: `submission-core-contract-snapshot.json`, event variant fixtures
- Security: `security-boundaries.md` (read-only screening, no writeback)
- **Submitted** — https://github.com/arXiv/arxiv-submission-core/issues/88

## Evidence record

| Repository | URL | Date | Artifact/branch | Status | Maintainer response |
| --- | --- | --- | --- | --- | --- |
| `arXiv/submit-ce` | https://github.com/arXiv/submit-ce/issues/72 | 2026-06-09 | issue-first draft | Submitted | Pending |
| `arXiv/arxiv-submission-core` | https://github.com/arXiv/arxiv-submission-core/issues/88 | 2026-06-09 | issue-first draft | Submitted | Pending |

## Acceptance boundary

An open issue or PR is only "submitted". Acceptance requires a merged PR,
maintainer acceptance comment, or another public acceptance URL.

## Rollback

- Remove any release-status or docs wording that implies upstream acceptance
  without URL evidence.
- Revert evidence-ledger `submitted` or `accepted` gates if external submission
  is withdrawn.
- Keep local adapters fixture-backed; do not claim arXiv state mutation.
