# arXiv upstream monitor

Date: 2026-06-10

## Correction

`arXiv/arxiv-submission-core` is **legacy/inactive** (maintainer @dginev on #88). The
correct active intake surface is **`arXiv/submit-ce`**.

## Issues

| Repo | Issue | State | Notes |
| --- | --- | --- | --- |
| submit-ce | https://github.com/arXiv/submit-ce/issues/72 | Open | Follow-up posted with local smoke + hook proposal: https://github.com/arXiv/submit-ce/issues/72#issuecomment-4662113521 |
| arxiv-submission-core | https://github.com/arXiv/arxiv-submission-core/issues/88 | Closed | Redirect comment: https://github.com/arXiv/arxiv-submission-core/issues/88#issuecomment-4662122861 |

## Local proof

`journal-screen-smoke-2026-06-10.md` — `screened_with_warnings` on synthetic submit-ce fixture.

## Integration hook (proposed)

1. Post source extraction: external read-only `journal-screen` on exported bundle JSON.
2. Complement `submit_ce/ui/filters/tex_filters.py` citation warnings with structured CSL screening.

Awaiting maintainer guidance on preferred insertion point before opening a PR.
