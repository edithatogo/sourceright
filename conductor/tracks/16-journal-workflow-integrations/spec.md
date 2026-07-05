# Journal Workflow Integrations Spec

## Goal

Make Sourceright usable by journals, publishers, and editorial offices as a citation-integrity screening service for submitted manuscripts.

Track 88 extends this completed integration contract with a strategic roadmap for future scientific papers and agentic editorial workflows. Track 16 remains the implemented screening surface; Track 88 defines the longer-term research-object and submission-management context.

## Scope

- Define a generic journal screening adapter contract that accepts manuscript files or extracted text and returns reference integrity outputs.
- Build an OJS-first integration plan because Open Journal Systems has an open plugin ecosystem and is suitable for public implementation.
- Define adapter plans for ScholarOne, Editorial Manager, eJournalPress, Manuscript Manager, and similar SaaS editorial systems where vendor or publisher access may be required.
- Return editor-facing and author-facing outputs without claiming that errors are AI-generated.
- Preserve manuscript privacy, auditability, and deterministic execution.
- Keep the adapter contract compatible with agentic pre-submission checks, structured intake, triage support, reviewer briefing packs, and post-publication monitoring.

## Outputs

- `citation-integrity-report.json`.
- `citation-integrity-report.md`.
- Optional HTML/PDF report rendering.
- Editorial triage summary.
- Author-facing citation action checklist.
- Platform adapter contracts for OJS, generic webhooks, and enterprise systems.
- Agent-ready JSON artifacts for later workflow automation.

## Agentic workflow boundary

Journal integrations may support agents that structure, check, route, summarize, and monitor submissions. They must not make autonomous accept, reject, clinical-use, or publication-support decisions.

The workflow boundary is:

- authors remain responsible for submitted content;
- reviewers remain responsible for reviews;
- editors remain accountable for editorial decisions;
- any agentic recommendation must be auditable and overridable;
- confidential manuscripts must stay inside the journal-controlled processing boundary unless explicit policy and permission allow another path.

## Boundaries

This track is an integration layer over Sourceright's core. It should not add new citation-verification logic directly into platform adapters.

OJS can be treated as the first public plugin target. ScholarOne, Editorial Manager, eJournalPress, and similar systems should start as adapter contracts and batch/webhook workflows unless API access and test credentials are available.

The screening report identifies citation-integrity risks and AI-related citation-error signals. It must not assert that a manuscript or citation was produced by AI.
