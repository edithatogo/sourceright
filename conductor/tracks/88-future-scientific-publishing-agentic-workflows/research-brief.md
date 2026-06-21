# Research Brief: Future Scientific Publishing and Agentic Editorial Workflows

## Purpose

This brief consolidates the research direction for reimagining the scientific paper and makes the submission-management layer explicit. It guides Sourceright's journal workflow roadmap for editorial screening, agentic workflow support, structured validation reports, and post-publication monitoring.

## Core thesis

The future scientific paper should be treated as a structured research object and as a workflow object. It is not only something to read after acceptance. It is something to submit, validate, review, update, query, execute, link, and monitor over time.

For Sourceright, this means journal integration should not stop at citation checking. Citation integrity, source provenance, reporting completeness, recency signals, retraction signals, data and code links, AI-use declarations, and reviewer briefing outputs should become parts of an auditable submission package.

## Design requirements

A post-PDF scholarly record should be:

1. **Modular**: claims, methods, data, code, reviews, corrections, and author contributions can be addressed separately.
2. **Executable where relevant**: computational findings can be rerun or inspected in preserved environments.
3. **Machine-actionable**: metadata and claims can flow into knowledge graphs, syntheses, policy tools, and review systems.
4. **Versioned and auditable**: readers can see status, provenance, corrections, and update history.
5. **Operationally structured at submission**: a journal receives a research package, not only a manuscript file.
6. **Human-governed**: agents support checks and synthesis, but editorial accountability remains human.

## Future paper models

### Option 1: Living evidence record

A living evidence record is a maintained web-native article whose recommendations, evidence tables, and conclusions are updated when relevant new evidence appears.

**Conceptual framework**

The article becomes a maintained evidence service. The stable scholarly object is not one final narrative, but a versioned record with update triggers, evidence surveillance, changelogs, and status labels.

**Required infrastructure**

- Structured authoring for evidence summaries and recommendations.
- Version metadata and persistent identifiers.
- Update triggers and standing editorial groups.
- Links to trials, systematic reviews, datasets, corrections, and prior versions.
- Public changelogs and supersession labels.

**Rigor mechanisms**

- Predefined update rules.
- GRADE or equivalent evidence appraisal where applicable.
- Human evidence-editor approval.
- Conflict-of-interest refresh at update points.
- Clear distinction between living evidence, preprints, and clinically actionable guidance.

**Sourceright role**

Sourceright can monitor cited evidence, flag retractions or corrections, check broken links, surface superseded guideline signals, and generate update-readiness reports for living articles.

### Option 2: Executable research object

An executable research object packages narrative, code, data, workflow, environment, and outputs so computational claims can be inspected and rerun.

**Conceptual framework**

The article becomes a runnable representation of the research workflow rather than only a description of it.

**Required infrastructure**

- Containerized or otherwise reproducible environments.
- Code, data, and environment packaging.
- Data and software citations.
- Sandbox access for reviewers.
- Preservation of environment metadata and build logs.

**Rigor mechanisms**

- Run-on-submit checks.
- Verification that key figures and tables regenerate.
- Reviewer access to a controlled execution environment.
- Versioned code and data references.
- Clear separation between reproducibility, replication, and scientific validity.

**Sourceright role**

Sourceright can validate data and software citations, link code and dataset identifiers to article claims, surface missing provenance, and include reproducibility-readiness signals in editorial reports.

### Option 3: Refereed preprint network paper

A refereed preprint network paper treats the public preprint, peer reviews, author responses, journal assessment, and curation signal as the complete scholarly object.

**Conceptual framework**

Dissemination and certification are separated. The journal becomes a curator and evaluator of already visible research rather than the sole gateway to public release.

**Required infrastructure**

- Preprint server links.
- Review transfer metadata.
- Public peer-review records where policy allows.
- Version relationships between preprint, reviewed preprint, accepted article, and version of record.
- Status labels for health and clinical-adjacent research.

**Rigor mechanisms**

- Transparent review provenance.
- Structured editorial assessments.
- Clear version control.
- Review reuse policies and conflict checks.
- Explicit warning labels where unreviewed preprints should not guide clinical practice.

**Sourceright role**

Sourceright can ingest preprints, compare versions, link reviews to cited claims, check references across revisions, and generate curation packets for editors.

### Option 4: Machine-actionable claim graph

A claim graph makes the paper a human-readable view over structured assertions, evidence links, methods, populations, interventions, outcomes, provenance, reviews, and corrections.

**Conceptual framework**

The unit of scholarly exchange shifts from the whole document to claims with provenance.

**Required infrastructure**

- Structured article markup.
- Claim extraction and author validation.
- Persistent identifiers for related objects.
- APIs for article components.
- Knowledge graph export formats.

**Rigor mechanisms**

- Claim-level provenance.
- Citation and source verification.
- Human validation of extracted claims.
- Change alerts when upstream evidence is corrected or retracted.
- Clear versioning of claims and relationships.

**Sourceright role**

Sourceright's claim/source/provenance model can become a validation layer for claim graphs, especially for source linkage, citation integrity, and recency-aware evidence alerts.

### Option 5: Community-governed research ledger

A community-governed research ledger records research workflow states, funding milestones, review attestations, governance decisions, and update events.

**Conceptual framework**

The paper becomes one state transition in a broader governed research workflow.

**Required infrastructure**

- Identity and role verification.
- Governance records.
- Reviewer compensation or acknowledgement workflows where appropriate.
- Off-chain storage for substantive scholarly objects.
- Legal, financial, and compliance safeguards.

**Rigor mechanisms**

- Conflict checks.
- Anti-gaming controls.
- Reviewer qualification checks.
- Human oversight of governance outcomes.
- Separation between scholarly validation and speculative incentive systems.

**Sourceright role**

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

Agents can monitor broken links, code or data access, cited retractions, corrections, PubPeer-style comments, replication attempts, new linked evidence, and update triggers for living records.

## Governance requirements

- Human authority: editors decide, reviewers review, and authors remain responsible for submissions.
- Role-specific AI policy: author, reviewer, editor, staff, and production uses need separate rules.
- Confidentiality: manuscripts should not be uploaded to third-party AI systems unless policy, consent, and security requirements are satisfied.
- Audit logs: every agentic action should record inputs, tool version, output, confidence, human override, and downstream use.
- Bias and robustness audits: reviewer matching, triage, and decision support should be tested for bias, manipulation, and hidden prompt attacks.
- Appeal pathway: authors should be able to challenge agent-generated errors in scope, completeness, conflict, and validation checks.

## Transitional roadmap for journals

### Phase 1: Structured current submissions

Require ORCID, CRediT, funding metadata, AI-use declarations, reporting checklist selection, data and code availability statements, ethics metadata, and persistent links as machine-readable fields.

### Phase 2: Pre-submission and intake agents

Deploy author-facing readiness checks and editorial intake checks. Start with completeness, reporting guideline fit, citation integrity, and metadata quality.

### Phase 3: Technical validation

Add similarity, citation, recency, link, data, code, image, and reporting checks. Treat outputs as editor-facing evidence, not decisions.

### Phase 4: Reviewer matching and briefing

Use agents to propose reviewers, flag conflicts, generate briefing packs, and summarize technical checks. Require editor confirmation or override.

### Phase 5: Structured review and decision support

Move reviewer reports into structured forms aligned to article type and method. Use agents to synthesize reviews and draft decision letters while preserving human accountability.

### Phase 6: Post-publication monitoring

Monitor corrections, retractions, broken links, new evidence, replication attempts, and reader commentary. Apply this first to living evidence, computational papers, and digital health papers with policy or clinical-adjacent implications.

## Implications for Sourceright

Sourceright should position journal integration as a validation layer for future research objects. The strongest near-term implementation path is:

1. Structured submission package readiness checks.
2. Citation and reference integrity validation.
3. Source provenance and claim-link validation.
4. Provider-backed recency, correction, and retraction evidence.
5. Editor and author report artifacts.
6. Agent-ready JSON outputs for workflow adapters.
7. Post-publication monitoring reports for published records.

The product boundary should remain explicit. Sourceright supports scientific rigor by improving evidence traceability, citation integrity, and workflow auditability. It does not determine whether a manuscript should be accepted, whether a clinical recommendation is valid, or whether a paper was written by AI.

## Source anchors

This brief is grounded in the research synthesis covering living evidence systems, executable research objects, reviewed preprints, machine-actionable metadata, peer-review metadata, and AI-governed editorial workflows. Key public sources include Crossref documentation on manuscript submission systems, peer-review metadata, Crossmark, and versioning; JATS and JATS4R article markup; ORCID and CRediT identity and contribution infrastructure; FAIR principles; FORCE11 data and software citation principles; NISO peer review terminology; ICMJE recommendations on AI in publishing; EQUATOR reporting guidelines; Review Commons, eLife, F1000Research, PCI, and PREreview review models; Whole Tale, Jupyter Book, Stencila, RO-Crate, and Code Ocean executable research infrastructure; ALEC, MAGICapp, WHO, and Cochrane living evidence models; and DeSci examples such as ResearchHub and DeSci Foundation.
