# Journal-Platform Submission Packet

## Surfaces

- OJS/PKP
- arXiv boundary cross-reference

## Requirements Evidence

| Surface | Source | Retrieved | Local impact |
| --- | --- | --- | --- |
| OJS/PKP | <https://docs.pkp.sfu.ca/dev/plugin-guide/> | 2026-05-18 | OJS packaging needs plugin metadata, install behavior, settings/permissions, compatibility proof, and Gallery-readiness evidence. |
| PKP Plugin Gallery | PKP community and plugin-gallery evidence path | 2026-05-18 | Accepted Gallery claims require public listing evidence; source skeleton alone is not enough. |

## Local Gates

- OJS package build.
- OJS compatibility matrix for supported LTS/current versions.
- Fixture smoke and optional disposable OJS instance smoke.
- Gallery submission draft with rollback/support notes.

## Blockers

- live or disposable instance compatibility proof missing
- PKP Plugin Gallery PR or accepted listing evidence missing

## arXiv Boundary

arXiv submission-platform work is not an OJS package. It is governed by Tracks
78-81 and the `arxiv-upstream.md` packet.

## Local package evidence (Track 75)

| Artifact | Verified |
| --- | --- |
| `conductor/tracks/75-journal-platform-publication-hardening/ojs-fixture-smoke-2026-06-09.md` | 2026-06-09 |
| `conductor/tracks/75-journal-platform-publication-hardening/ojs-compatibility-matrix.md` | 2026-06-09 |
| `conductor/tracks/75-journal-platform-publication-hardening/submission-drafts.md` | 2026-06-09 |

## Draft Submission Body

Local package, compatibility matrix, and fixture smoke are hardened. Gallery
submission drafts are ready for approval-gated publication. Live/disposable OJS
install proof and accepted Gallery listing evidence remain missing.

## Approval Gate

No PKP Plugin Gallery submission without explicit approval.
