# arXiv Submission Platform Adapters Spec

## Goal

Add Sourceright-side submission platform contracts for the current arXiv `submit-ce` platform and the legacy `arxiv-submission-core` platform.

## User outcome

An implementer can build arXiv submission modules later from stable Sourceright contracts instead of inventing arXiv-specific screening semantics.

## Scope

- Add explicit `journal-screen` platform values for `arxiv-submit-ce` and `arxiv-submission-core`.
- Register separate journal plugin manifests for the current and legacy arXiv lanes.
- Add fixture-backed submissions for both platform generations.
- Document the distinction between `provider.arxiv` metadata evidence and arXiv submission-platform screening.
- Keep upstream arXiv patches, live credentials, and platform writes out of this slice.

## Out of scope

- Pull requests to `arXiv/submit-ce` or `arXiv/arxiv-submission-core`.
- Live arXiv test-instance, credential, or API integration.
- Any writeback to arXiv systems or canonical CSL.

## Data contracts

- Inputs: submission source bundle or legacy submission event/domain payload plus canonical CSL and verification sidecar.
- Output: `sourceright.journal_screening.v1`.
- Platform values: `arxiv_submit_ce` and `arxiv_submission_core`.
- Provider evidence remains sidecar-only through `provider.arxiv`.

## Claim boundary

Allowed wording is "technical preview", "fixture-backed", and "planned submission-platform adapter". Do not claim upstream module acceptance, live arXiv integration, or arXiv-side writeback.

## Evidence level target

Fixture-backed.

## Parallelization plan

- Dependency slice: shared schema, enum, CLI/MCP parsing, registry entries, and policy tests.
- Lane A: `submit-ce` fixture, manifest, docs, and proof notes.
- Lane B: `arxiv-submission-core` fixture, manifest, docs, and proof notes.
- Lane C: shared schema/CLI/MCP tests and overclaim guards.
- Lane D: opt-in external proof-suite and future-upstream notes.

Only Lane C edits shared contracts. Platform lanes own separate fixtures and manifests to avoid overlapping writes.
