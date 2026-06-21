# Track 92 - Reporting Checklist Contract Workflow

## Goal

Create a workflow that determines whether reporting checklists exist for a manuscript, chooses the most appropriate checklist or combination, sources checklist items, combines and deduplicates them into a machine-readable contract, assesses the manuscript against that contract, produces MoSCoW findings, creates an implementation plan, and optionally implements approved changes.

## User outcome

An author or editor can receive an evidence-based reporting readiness assessment without manually searching multiple guideline sources or reconciling overlapping checklist items.

## Required workflow

1. Infer study design, article type, methods, population, setting, data type, intervention, and health domain from the research package.
2. Search authoritative reporting guideline sources and journal requirements.
3. Rank candidate checklists by relevance and explain why one checklist or a combination is recommended.
4. Source checklist items with version, URL, retrieval date, item IDs, and provenance.
5. Combine checklists where appropriate.
6. Deduplicate overlapping items while preserving source-item provenance.
7. Create `sourceright.reporting_checklist_contract.v1`.
8. Assess the manuscript against each checklist item.
9. Classify each item as pass, fail, partial, not applicable, or unknown.
10. Assign MoSCoW priorities to findings.
11. Report the initial outcome.
12. Create an implementation plan.
13. Optionally implement approved changes through dry-run first writeback or author tasks.

## Candidate checklist families

The workflow should be able to reason over, but not be limited to, CONSORT, SPIRIT, PRISMA, PRISMA-P, STROBE, RECORD, TRIPOD, STARD, CARE, CHEERS, AGREE, RIGHT, TIDieR, SQUIRE, GRIPP, ARRIVE, SRQR, COREQ, DECIDE-AI, CONSORT-AI, SPIRIT-AI, TRIPOD-AI, and future AI, digital health, equity, implementation, economics, and reproducibility guidelines.

## Outputs

- Checklist selection rationale.
- Deduplicated checklist contract in JSON or YAML.
- Item-level provenance table.
- Checklist assessment report.
- MoSCoW recommendation report.
- Implementation plan.
- Optional dry-run patch or author task list.

## Out of scope

- Claiming a checklist has been satisfied without evidence.
- Treating checklist completion as equivalent to study quality.
- Modifying manuscript text without explicit approval.
- Overriding journal-specific checklist requirements without editor approval.

## Claim boundary

The workflow assesses reporting completeness against sourced checklist items. It does not certify methodological quality, clinical validity, or publication readiness by itself.

## Completion criteria

- Checklist discovery and ranking are specified.
- Combination and deduplication rules are specified.
- Machine-readable checklist contract is documented.
- Assessment and MoSCoW reporting are documented.
- Dry-run and explicit apply boundaries are documented.
