# Security Boundaries — arXiv submission-core Adapter

> **⚠️ Legacy warning**: `arXiv/arxiv-submission-core` is **legacy/inactive** per
> maintainer @dginev on issue #88 (2026-06-10). This security boundaries document
> is provided for informational and migration purposes only. Users should migrate
> to the active `arXiv/submit-ce` surface (see Track 79).
>
> **Claim boundary**: This document describes the security boundaries of the
> Sourceright submission-core adapter. It does **not** claim that the adapter
> is arXiv-approved, arXiv-compatible, or actively integrated with arXiv's
> infrastructure.
>
> Evidence level: **contracted** — all claims are derived from reference-able
> sources.

## Scope

The submission-core adapter in Sourceright provides:

- A `JournalPlatform::ArxivSubmissionCore` enum variant (serialized as
  `arxiv_submission_core`) for platform identification.
- A synthetic submission fixture at
  `fixtures/journal/arxiv-submission-core-submission.json` for contract smoke
  testing.
- Read-only journal screening via `screen_journal_submission()`.

All operations are local, deterministic, and fixture-backed.

## Boundaries

### 1. Read-only screening — no arXiv submission state mutation

| Property | Detail |
|---|---|
| **What it does** | Reads fixture CSL + verification sidecar from local workspace; emits screening report to stdout |
| **What it does NOT do** | Creates, reads, updates, or deletes any arXiv submission state |
| **arXiv API endpoints touched** | None |
| **arXiv databases touched** | None |
| **Local file modification** | None — fixture files are read-only; report is stdout only |

The adapter performs **no** arXiv submission lifecycle operations. It does not
start submissions, accept policies, assert authorship, set licenses, upload
files, set categories, set metadata, confirm previews, or mark submissions as
deposited — all of which are required for actual arXiv submission.

### 2. No arXiv API calls — fixture-backed only

| Property | Detail |
|---|---|
| **arXiv API calls** | **Zero** — no HTTP requests to any arXiv endpoint |
| **arXiv credentials** | **None** — no API tokens, client IDs, client secrets, or passwords configured or required |
| **arXiv accounts** | **None** — no arXiv user accounts set up or used |
| **Network access** | **None** — all operations are local to the workspace directory |
| **Fixture dependency** | `fixtures/journal/arxiv-submission-core-submission.json` provides all input data |

The submission-core adapter is entirely self-contained. It does not connect to
any external service, including `https://arxiv.org/`,
`https://github.com/arXiv/arxiv-submission-core/`, or any arXiv API gateway,
OAuth2 endpoint, or database instance.

### 3. No writeback to canonical CSL data

| Property | Detail |
|---|---|
| **Canonical CSL** (`references.csl.json`) | **Not modified** — adapter reads fixture data only |
| **Verification sidecar** (`references.verification.json`) | **Not modified** — adapter reads fixture data only |
| **Review queue** (`review-queue.jsonl`) | **Not modified** — adapter does not interact with review queue |
| **Provider data** | **Not modified** — no provider writeback |
| **Screening report** | Emitted to stdout only; not persisted to any file |

This aligns with the Sourceright architectural principle that provider data
must never silently overwrite canonical CSL.

### 4. Platform enum scope

| Property | Detail |
|---|---|
| **Enum variant** | `JournalPlatform::ArxivSubmissionCore` |
| **Serialized form** | `arxiv_submission_core` |
| **Purpose** | Informational and migration documentation only |
| **Not wired to** | Any live arXiv API endpoint, authentication flow, or submission pipeline |
| **Supported CLI flags** | `--platform arxiv-submission-core` and `--platform arxiv_submission_core` |

The platform enum exists to:
- Document the legacy adapter's contract shape.
- Provide a migration reference for users switching from submission-core to
  submit-ce.
- Enable contract smoke tests against the synthetic fixture.

It is **not** wired to any live arXiv API endpoint. No arXiv credentials,
authentication tokens, or network configuration are required or used.

### 5. Legacy warning notice

The adapter carries the following warnings in all evidence documents:

1. **Legacy/inactive status**: Confirmed by maintainer @dginev on issue #88
   (2026-06-10). The repo is no longer under active development.
2. **No arXiv support**: arXiv will not provide bug fixes, security patches,
   or compatibility updates for the submission-core surface.
3. **Migration required**: Users must migrate to `arXiv/submit-ce` for any
   arXiv integration work.
4. **No live API claims**: The adapter makes no live API calls and holds no
   arXiv credentials. Claims are at evidence level `contracted`.

## Summary

| Boundary | Status | Risk |
|---|---|---|
| arXiv state mutation | **Prevented** — no mutation code paths exist | None |
| arXiv API calls | **Prevented** — no HTTP client configured | None |
| CSL writeback | **Prevented** — read-only fixture data only | None |
| Credential leakage | **Prevented** — no credentials stored or used | None |
| Network access | **Prevented** — local workspace only | None |
| Platform enum misuse | **Documented** — informational only, not wired to live API | Low (migration confusion) |
| Legacy status confusion | **Documented** — prominent warnings in all docs | Low (user education) |

## References

- Upstream issue: `https://github.com/arXiv/arxiv-submission-core/issues/88`
- Redirect comment: `https://github.com/arXiv/arxiv-submission-core/issues/88#issuecomment-4662122861`
- Track 78 requirements matrix:
  `conductor/tracks/78-arxiv-upstream-requirements-reconnaissance/requirements-matrix.md`
- Track 79 submit-ce maturity hardening:
  `conductor/tracks/79-arxiv-submit-ce-maturity-hardening/`
- Track 81 upstream submission and acceptance:
  `conductor/tracks/81-arxiv-upstream-submission-and-acceptance/`