# Track 91 - Journal-Article Requirements Contract Workflow

## Goal

Create a workflow that sources a target journal's submission and article requirements, converts them into a machine-readable contract, assesses the submission against the contract, produces MoSCoW recommendations, and optionally implements approved recommendations through dry-run first writeback.

## User outcome

An author or editor can choose a target journal and article type, then receive a sourced, auditable readiness assessment before submission. Where safe, the workflow can also create an implementation plan and apply approved changes.

## Required workflow

1. Identify target journal, publisher, article type, section, submission platform, and intended output state.
2. Source official requirements from author instructions, editorial policy, data policy, ethics policy, AI policy, reporting policy, preprint policy, figure policy, references policy, and submission system requirements.
3. Record source URLs, retrieval date, source type, freshness, and confidence.
4. Create `sourceright.journal_article_requirements.v1`.
5. Require human approval before using a newly sourced contract as a gate.
6. Assess the submission package against each requirement.
7. Classify findings as pass, fail, partial, not applicable, or unknown.
8. Assign MoSCoW priority to each gap.
9. Produce author-facing and editor-facing reports.
10. Create an implementation plan for approved changes.
11. Optionally implement changes through dry-run first, explicit apply, and reversible outputs.

## Requirement categories

- Scope fit.
- Article type fit.
- Manuscript structure.
- Word count and abstract requirements.
- Title page, highlights, key points, graphical abstract, or lay summary.
- Reference style and limits.
- Figures, tables, supplementary materials, and image integrity.
- Data availability and repository requirements.
- Code availability and reproducibility requirements.
- Protocol, registration, and ethics requirements.
- Consent and patient or public involvement requirements.
- AI-use disclosure.
- Conflict of interest and funding disclosure.
- Reporting checklist requirements.
- Preprint and prior dissemination policy.
- License, copyright, fees, and open access requirements.
- Submission platform metadata requirements.

## Outputs

- Journal-article contract in JSON or YAML.
- Requirements assessment in JSON, YAML, and Markdown.
- MoSCoW recommendation report.
- Dry-run implementation plan.
- Optional patch or author task list.
- Audit log showing sources, tool versions, model versions where relevant, and human approvals.

## Out of scope

- Scraping paywalled or restricted author instructions without permission.
- Claiming target journal acceptance likelihood.
- Submitting to the journal automatically.
- Applying changes without explicit approval.

## Claim boundary

The workflow assesses readiness against sourced requirements. It does not guarantee acceptance and does not replace editorial judgment.

## Completion criteria

- Contract schema is documented.
- Workflow phases are documented.
- MoSCoW assessment rules are documented.
- Dry-run and apply boundary is documented.
- Test matrix covers sourcing, contract creation, assessment, recommendations, and optional implementation.
