---
title: Future scientific publishing
description: Roadmap context for future papers and agentic editorial workflows.
---

Sourceright's journal workflow roadmap treats the future scientific paper as both a structured research object and an auditable workflow object. The paper is submitted, validated, reviewed, updated, queried, linked, and monitored over time.

The detailed Conductor context is tracked in `conductor/tracks/88-future-scientific-publishing-agentic-workflows/`.

## Future paper options

### Living evidence record

A maintained web-native article with update triggers, evidence surveillance, changelogs, and status labels.

Sourceright can monitor cited evidence, flag retractions or corrections, check broken links, surface superseded guideline signals, and generate update-readiness reports.

### Executable research object

A package of narrative, code, data, workflow, environment, and outputs so computational claims can be inspected and rerun.

Sourceright can validate data and software citations, link code and dataset identifiers to article claims, surface missing provenance, and include reproducibility-readiness signals.

### Refereed preprint network paper

A scholarly object composed of a public preprint, peer reviews, author responses, journal assessment, and curation signal.

Sourceright can ingest preprints, compare versions, link reviews to cited claims, check references across revisions, and generate curation packets.

### Machine-actionable claim graph

A human-readable view over structured assertions, evidence links, methods, outcomes, provenance, reviews, and corrections.

Sourceright's claim/source/provenance model can validate source linkage, citation integrity, and recency-aware evidence alerts.

### Community-governed research ledger

A workflow record for funding milestones, review attestations, governance decisions, and update events.

Sourceright should treat this as experimental and contribute audit artifacts without depending on token mechanics for core integrity claims.

## Agentic submission-management layer

Agentic workflows should structure, check, route, summarize, and monitor. They should not make editorial decisions.

| Stage | Sourceright-facing role |
| --- | --- |
| Pre-submission guidance | Check scope, reporting guideline, data, code, ethics, contribution, funding, and AI-use readiness. |
| Structured intake | Receive manuscript text, references, claims, datasets, code, protocols, ethics metadata, checklists, conflicts, and persistent identifiers. |
| Triage and routing | Classify submissions by topic, method, article type, computational dependency, ethical risk, and reporting requirements. |
| Reviewer selection support | Propose reviewers while preserving editor authority and conflict checks. |
| Technical validation | Run citation, source, recency, link, data, code, reporting, and integrity checks. |
| Briefing packs | Summarize key claims, methods, datasets, code, reporting status, missing items, versions, and review history. |
| Decision support | Synthesize reviews and draft decision letters without taking the final decision. |
| Post-publication monitoring | Monitor broken links, data or code access, retractions, corrections, comments, replication attempts, and update triggers. |

## Governance boundary

Editors remain accountable for editorial decisions, reviewers remain accountable for reviews, and authors remain responsible for submissions. Agentic actions need audit logs, confidentiality boundaries, bias and manipulation checks, and appeal paths for agent-generated errors.

## Product boundary

Sourceright supports scientific rigor by improving evidence traceability, citation integrity, source provenance, and workflow auditability. It does not determine whether a manuscript should be accepted, whether a clinical recommendation is valid, or whether a paper was written by AI.
