# Track 78 — arXiv Upstream Requirements Evidence

Date: 2026-06-09

## Inventory alignment

| Surface | Inventory id | Submission packet |
| --- | --- | --- |
| arXiv submit-ce | `arxiv-submit-ce` | `conductor/submission-packets/arxiv-upstream.md` |
| arXiv submission-core | `arxiv-submission-core` | same |

## Official requirements sources (searched 2026-05-18)

| Repository | Source | Local impact |
| --- | --- | --- |
| `arXiv/submit-ce` | https://github.com/arXiv/submit-ce | Current submission-platform code; align screening output to `sourceright.journal_screening.v1` without mutating arXiv state |
| `arXiv/arxiv-submission-core` | https://github.com/arXiv/arxiv-submission-core and https://arxiv.github.io/arxiv-submission-core/ | Legacy event/domain model; migration-safe mapping with review/warning on unknown events |

## Local adapter evidence (Track 71)

| Lane | Manifest | Fixture | CLI smoke |
| --- | --- | --- | --- |
| submit-ce | `plugins/manifests/journal.arxiv-submit-ce.toml` | `fixtures/journal/arxiv-submit-ce-submission.json` | `journal-screen --platform arxiv-submit-ce` |
| submission-core | `plugins/manifests/journal.arxiv-submission-core.toml` | `fixtures/journal/arxiv-submission-core-submission.json` | `journal-screen --platform arxiv-submission-core` |

## Submission path decision

Issue-first upstream engagement for both repositories. The first issue asks
whether maintainers want external integration, an in-repository module, or no
upstream change. Pull requests follow only after maintainer direction.

## Claim boundary

Requirements reconnaissance only. No upstream GitHub issue, pull request, or
module acceptance is claimed.
