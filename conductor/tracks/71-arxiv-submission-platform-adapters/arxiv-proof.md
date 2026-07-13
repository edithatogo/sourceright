# arXiv Submission Platform Fixture Proof

## Purpose

This proof defines the local, fixture-backed evidence for arXiv submission
platform screening. It does not prove live arXiv integration.

## Fixture commands

For each fixture:

```text
fixtures/journal/arxiv-submit-ce-submission.json
fixtures/journal/arxiv-submission-core-submission.json
```

1. Initialise a temporary Sourceright workspace.
2. Copy `csl_references` into `.sourceright/references.csl.json`.
3. Copy `verification_sidecar.references` into `.sourceright/references.verification.json` with schema version `sourceright.verification.v1`.
4. Run:

```text
sourceright journal-screen --platform arxiv-submit-ce --submission-id ARXIV-CE-2026-0001 --manuscript source-package.tar.gz .sourceright
sourceright journal-screen --platform arxiv-submission-core --submission-id ARXIV-LEGACY-2026-0001 --manuscript legacy-source.tar.gz .sourceright
```

## Required assertions

- Output schema is `sourceright.journal_screening.v1`.
- Platform serializes as `arxiv_submit_ce` or `arxiv_submission_core`.
- Status is `screened_with_warnings` for both synthetic fixtures.
- Output does not contain `AI-generated` or `AI authorship`.

## Overclaim guard

Do not claim upstream module acceptance, live arXiv integration, or arXiv-side
writeback. The only supported claim is fixture-backed technical-preview
screening through the shared Sourceright journal contract.
