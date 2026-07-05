# Track 80 — Evidence Packet: arXiv submission-core Maturity Hardening

> **Claim boundary**: This evidence packet documents the legacy `arXiv/arxiv-submission-core`
> adapter's current state, security boundaries, and migration path. It does **not**
> claim that the adapter is actively maintained, arXiv-approved, or compatible
> with arXiv's current submission infrastructure. All claims are at evidence level
> **contracted** — derived from reference-able sources, not from live API calls
> or maintainer interviews.
>
> **Disclaimer**: Adapter is **legacy-documented** and has **not been reviewed or
> accepted by arXiv maintainers for continued use**. Users should migrate to the
> active `arXiv/submit-ce` surface (see Migration path section below).

## Current state

`arXiv/arxiv-submission-core` is **legacy/inactive** per maintainer @dginev on
issue #88 (2026-06-10). The correct active intake surface is
`arXiv/submit-ce`.

| Property | Value |
|---|---|
| **Repo** | `https://github.com/arXiv/arxiv-submission-core` |
| **Status** | ⛔ **Legacy/Inactive** — per maintainer @dginev on issue #88 |
| **Description** | arXiv-NG submission system (predecessor to submit-ce) |
| **Last release** | v0.6.1 (Jan 30, 2019) |
| **Last commit** | Archived — no active development |
| **Python version** | 3.6 (EOL) |
| **CI provider** | Travis CI (deprecated) |
| **Build system** | Pipenv (Pipfile.lock) |
| **Active replacement** | ✅ `arXiv/submit-ce` — see Track 79 |

## Issue record

| Item | Value |
|---|---|
| **Issue URL** | `https://github.com/arXiv/arxiv-submission-core/issues/88` |
| **Date filed** | 2026-06-09 |
| **Date closed** | 2026-06-10 |
| **Filed by** | Sourceright integration team (Track 81) |
| **Maintainer response** | @dginev confirmed repo is legacy/inactive; redirect to `submit-ce` |
| **Redirect comment** | `https://github.com/arXiv/arxiv-submission-core/issues/88#issuecomment-4662122861` |
| **Issue state** | Closed |

The issue was filed as part of Track 81 (upstream submission and acceptance).
The maintainer's response confirmed that `arXiv/arxiv-submission-core` is no
longer the active intake surface and directed all effort toward
`arXiv/submit-ce`.

## Contract snapshot

The submission-core adapter contract is frozen at:

### Platform enum

| Field | Value |
|---|---|
| **Enum variant** | `JournalPlatform::ArxivSubmissionCore` |
| **Serialized form** | `arxiv_submission_core` |
| **Source file** | `src/journal.rs` |
| **Status** | Informational / migration only — not connected to live API |

### Submission fixture

| Field | Value |
|---|---|
| **Fixture path** | `fixtures/journal/arxiv-submission-core-submission.json` |
| **Schema** | `schemas/ojs-submission-fixture.json` |
| **Submission ID** | `ARXIV-CORE-2026-0001` |
| **CSL references** | 2 (1 verified, 1 unverified) |
| **Verification sidecar** | Crossref provider candidate (confidence 0.95) |
| **Expected status** | `screened_with_warnings` |

### Screening report shape

| Field | Expected value |
|---|---|
| `schema_version` | `sourceright.journal_screening.v1` |
| `platform` | `arxiv_submission_core` |
| `status` | `screened_with_warnings` |
| `reference_report.summary.total_references` | 2 |
| `reference_report.summary.verified_references` | 1 |
| `reference_report.summary.review_queue_count` | 1 |
| `reference_report.summary.ai_risk_issue_count` | 1 |

### Requirements context

The Track 78 requirements matrix provides broader context for the submission-core
adapter across 6 dimensions:

`conductor/tracks/78-arxiv-upstream-requirements-reconnaissance/requirements-matrix.md`

Key findings for submission-core:
- **CONTRIBUTING.md**: ✅ Present — detailed Gitflow guide
- **CI**: Travis CI (deprecated), Python 3.6, Pipenv
- **Testing**: nose2 + pytest, coverage >=90%
- **Security**: OAuth2 client credentials flow, PyJWT 1.6.4, jsonschema 2.6.0
- **Submission**: RESTful API, MariaDB, Docker Compose, event-driven lifecycle

## Security boundaries

Full details in `security-boundaries.md`. Summary:

| Boundary | Detail |
|---|---|
| **Read-only screening** | `journal-screen` performs read-only screening on local fixture data. No arXiv submission state is created, read, updated, or deleted. |
| **No arXiv API calls** | The submission-core adapter makes **zero** arXiv API calls. All evidence is fixture-backed. No credentials, tokens, or arXiv accounts are configured or required. |
| **No CSL writeback** | The adapter does not write to canonical CSL data (`references.csl.json`) or the verification sidecar (`references.verification.json`). Screening reports are emitted to stdout only. |
| **Platform enum scope** | `JournalPlatform::ArxivSubmissionCore` exists for informational and migration documentation purposes. It is not wired to any live arXiv API endpoint. |
| **Legacy warning** | The adapter is documented as legacy/inactive per maintainer confirmation on issue #88. Users are directed to migrate to `arXiv/submit-ce`. |

## Migration path

Existing `arXiv/arxiv-submission-core` users should migrate to the active
`arXiv/submit-ce` surface:

### Step-by-step

1. **Acknowledge legacy status**: `arXiv/arxiv-submission-core` is
   legacy/inactive. Maintainer @dginev confirmed this on issue #88
   (2026-06-10). No further development, bug fixes, or support will be
   provided by arXiv for this surface.

2. **Review submit-ce maturity documentation**: Track 79 documents the
   `arXiv/submit-ce` adapter's maturity hardening evidence at:
   `conductor/tracks/79-arxiv-submit-ce-maturity-hardening/`

3. **Review upstream submission flow**: Track 81 handles upstream submission
   and acceptance for the active submit-ce surface:
   `conductor/tracks/81-arxiv-upstream-submission-and-acceptance/`

4. **File issues on the active repo**: All upstream issues should be filed on
   `https://github.com/arXiv/submit-ce` (not the legacy
   `arxiv-submission-core` repo).

5. **Update integration code**: Replace any `arxiv_submission_core` platform
   references with `arxiv_submit_ce`. The platform enum variant
   `ArxivSubmissionCore` remains available in `src/journal.rs` for
   transitional use but will not receive active development.

### Track cross-references

| Track | Purpose |
|---|---|
| **[Track 79](../79-arxiv-submit-ce-maturity-hardening/)** | Submit-ce adapter maturity hardening (active surface) |
| **[Track 81](../81-arxiv-upstream-submission-and-acceptance/)** | Upstream submission and acceptance flow |

## Claim boundary

> **"Legacy-documented" not "active-integration".** This evidence packet
> documents the legacy `arXiv/arxiv-submission-core` adapter's current state,
> security boundaries, and migration path. It does **not** claim that:
>
> - The adapter is actively maintained by arXiv.
> - The adapter is compatible with arXiv's current submission infrastructure.
> - The adapter has been reviewed or accepted by arXiv maintainers.
> - The adapter makes live API calls or mutates arXiv submission state.
>
> **Evidence level**: `contracted` — all claims are derived from reference-able
> sources (Track 78 requirements matrix, upstream GitHub issue #88, synthetic
> fixture file). No live API calls, credentials, or upstream acceptance evidence
> were used.

## Terminal evidence level

The terminal evidence level for this track is **contracted**. Unlike
Track 79 (submit-ce) which has an elevation path to `fixture-backed` and
potentially `opt-in-live-proven` via Track 81, the legacy submission-core
adapter has **no elevation path** to higher evidence levels because:

1. The upstream repo is confirmed legacy/inactive by the maintainer.
2. No live API calls can be made — the repo's submission infrastructure
   (Travis CI, Python 3.6, MariaDB, Kinesis) is deprecated.
3. All effort is redirected to `arXiv/submit-ce` (Track 79).

The adapter is documented for informational and migration purposes at evidence
level `contracted`. Any user needing arXiv integration should use the
`arXiv/submit-ce` surface documented in Track 79.
