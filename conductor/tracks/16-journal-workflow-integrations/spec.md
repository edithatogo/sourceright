# Journal Workflow Integrations Spec

## Goal

Make Sourceright usable by journals, publishers, and editorial offices as a citation-integrity screening service for submitted manuscripts.

## Scope

- Define a generic journal screening adapter contract that accepts manuscript files or extracted text and returns reference integrity outputs.
- Build an OJS-first integration plan because Open Journal Systems has an open plugin ecosystem and is suitable for public implementation.
- Define adapter plans for ScholarOne, Editorial Manager, eJournalPress, Manuscript Manager, and similar SaaS editorial systems where vendor or publisher access may be required.
- Return editor-facing and author-facing outputs without claiming that errors are AI-generated.
- Preserve manuscript privacy, auditability, and deterministic execution.

## Outputs

- `citation-integrity-report.json`.
- `citation-integrity-report.md`.
- Optional HTML/PDF report rendering.
- Editorial triage summary.
- Author-facing citation action checklist.
- Platform adapter contracts for OJS, generic webhooks, and enterprise systems.

## Boundaries

This track is an integration layer over Sourceright's core. It should not add new citation-verification logic directly into platform adapters.

OJS can be treated as the first public plugin target. ScholarOne, Editorial Manager, eJournalPress, and similar systems should start as adapter contracts and batch/webhook workflows unless API access and test credentials are available.

The screening report identifies citation-integrity risks and AI-related citation-error signals. It must not assert that a manuscript or citation was produced by AI.
