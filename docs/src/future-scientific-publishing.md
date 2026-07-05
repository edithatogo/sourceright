# Future Scientific Publishing

This roadmap note links Sourceright's journal workflows to a broader reimagining of the scientific paper. The paper of the future is treated as a structured research object and as an auditable workflow object. It is submitted, validated, reviewed, updated, queried, linked, and monitored over time.

The strategic details are tracked in `conductor/tracks/88-future-scientific-publishing-agentic-workflows/`.

## Design requirements

A post-PDF scholarly record should be:

- modular, so claims, methods, datasets, code, reviews, corrections, and author contributions can be addressed separately;
- executable where relevant, especially for computational and AI-heavy work;
- machine-actionable, so metadata and claims can flow into knowledge graphs, living reviews, policy tools, and review systems;
- versioned and auditable, so readers can see status, provenance, corrections, and update history;
- operationally structured at submission, so the journal receives a research package rather than only a manuscript file;
- human-governed, so agents can support checks and synthesis without replacing editorial accountability.

## Future paper options

### Living evidence record

A living evidence record is a maintained web-native article whose recommendations, evidence tables, and conclusions are updated when relevant new evidence appears.

Sourceright's role is to monitor cited evidence, flag retractions or corrections, check broken links, surface superseded guideline signals, and generate update-readiness reports for living articles.

### Executable research object

An executable research object packages narrative, code, data, workflow, environment, and outputs so computational claims can be inspected and rerun.

Sourceright's role is to validate data and software citations, link code and dataset identifiers to article claims, surface missing provenance, and include reproducibility-readiness signals in editorial reports.

### Refereed preprint network paper

A refereed preprint network paper treats the public preprint, peer reviews, author responses, journal assessment, and curation signal as the complete scholarly object.

Sourceright's role is to ingest preprints, compare versions, link reviews to cited claims, check references across revisions, and generate curation packets for editors.

### Machine-actionable claim graph

A claim graph makes the paper a human-readable view over structured assertions, evidence links, methods, populations, interventions, outcomes, provenance, reviews, and corrections.

Sourceright's claim/source/provenance model can become a validation layer for claim graphs, especially for source linkage, citation integrity, and recency-aware evidence alerts.

### Community-governed research ledger

A community-governed research ledger records research workflow states, funding milestones, review attestations, governance decisions, and update events.

Sourceright should treat this model as experimental. It can contribute audit artifacts and validation reports, but it should not depend on token mechanics or decentralized governance for core integrity claims.

## Agentic submission-management layer

Agentic workflows should be treated as the operational layer around future papers. They should not make editorial decisions. They should structure, check, route, summarize, and monitor submissions with full auditability.

### 1. Pre-submission guidance

Author-facing agents can check scope fit, article type, reporting guideline, ethics approval, trial registration, data availability, code availability, image-integrity readiness, conflict declarations, CRediT roles, ORCID identifiers, funding metadata, and AI-use declarations.

### 2. Structured submission intake

The submission should arrive as a structured package containing manuscript text, references, key claims, datasets, code, protocols, ethics metadata, checklists, author contributions, conflicts, AI-use declarations, and persistent identifiers.

### 3. Automated triage and routing

Agents can classify submissions by topic, method, article type, computational dependency, ethical risk, clinical relevance, reporting requirements, and editorial priority. They should identify missing components before human editorial review.

### 4. Reviewer selection support

Agents can propose reviewers based on expertise, methods knowledge, conflicts, availability, diversity of perspective, and prior review quality. Editors must retain authority to invite, exclude, or override reviewer suggestions.

### 5. Technical validation and integrity checks

Agents can run text similarity, citation validity, reference integrity, reporting checklist, image screening, statistical consistency, data link, code link, software environment, registration, ethics, and AI-use declaration checks.

### 6. Reviewer and editor briefing packs

Agents can generate structured packs summarizing article type, key claims, methods, datasets, code, ethics approvals, reporting status, missing items, prior versions, and review history. Briefing packs should distinguish extracted evidence from agent-generated synthesis.

### 7. Decision support

Agents can synthesize reviewer comments, identify agreement and disagreement, flag unresolved methodological concerns, and draft structured decision letters. The final decision remains attributable to the editor.

### 8. Post-publication monitoring

Agents can monitor broken links, code or data access, cited retractions, corrections, public comments, replication attempts, new linked evidence, and update triggers for living records.

## Governance requirements

- Editors retain authority over editorial decisions.
- Reviewers remain accountable for reviews.
- Authors remain responsible for submissions.
- AI policies should distinguish author, reviewer, editor, staff, and production use.
- Confidential manuscripts should not be uploaded to third-party AI systems unless policy, consent, and security requirements are satisfied.
- Every agentic action should be logged with input, tool version, output, confidence, human override, and downstream use.
- Reviewer matching, triage, and decision support should be tested for bias, manipulation, and hidden prompt attacks.
- Authors should be able to challenge agent-generated errors in scope, completeness, conflict, and validation checks.

## Transitional roadmap

1. Require ORCID, CRediT, funding metadata, AI-use declarations, reporting checklist selection, data and code availability statements, ethics metadata, and persistent links as machine-readable fields.
2. Deploy author-facing readiness checks and editorial intake checks for completeness, reporting guideline fit, citation integrity, and metadata quality.
3. Add similarity, citation, recency, link, data, code, image, and reporting checks as editor-facing evidence, not decisions.
4. Use agents to propose reviewers, flag conflicts, generate briefing packs, and summarize technical checks, with editor confirmation or override.
5. Move reviewer reports into structured forms and use agents to synthesize reviews and draft decision letters while preserving human accountability.
6. Monitor corrections, retractions, broken links, new evidence, replication attempts, and reader commentary after publication.

## Product boundary

Sourceright supports scientific rigor by improving evidence traceability, citation integrity, source provenance, and workflow auditability. It does not determine whether a manuscript should be accepted, whether a clinical recommendation is valid, or whether a paper was written by AI.
