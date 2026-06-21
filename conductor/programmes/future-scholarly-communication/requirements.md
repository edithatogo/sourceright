# Future Scholarly Communication Requirements

## Scope

These requirements describe the target state for Sourceright extensions and for a possible open Transforming Health publishing platform. They cover machine-readable submission packages, journal and article requirement contracts, reporting checklist contracts, agentic editorial workflows, preprint-first dissemination, peer review, reproducibility, contribution credit, and queryable knowledge graphs.

## MoSCoW requirements

### Must

- Represent every submission as a versioned research package, not only as a manuscript file.
- Support machine-readable manuscript, reference, data, code, protocol, ethics, reporting, peer-review, contribution, and dissemination metadata in JSON, YAML, and XML-compatible forms.
- Preserve a human-readable manuscript view for current journal, preprint, indexing, and archiving expectations.
- Source journal and article requirements from official author instructions, editorial policies, reporting policies, data policies, AI policies, ethics policies, and submission platform requirements.
- Convert sourced requirements into an explicit contract with provenance, freshness, confidence, and human review status.
- Assess each submission against the journal-article contract and produce MoSCoW recommendations.
- Optionally implement author-approved recommendations through dry-run plans before any writeback.
- Detect relevant reporting checklist candidates from study design, method, population, intervention, data type, article type, and jurisdiction.
- Source reporting checklists from authoritative sources, combine compatible checklists, deduplicate overlapping items, and preserve item-level provenance.
- Assess the submission against the reporting checklist contract and produce MoSCoW recommendations.
- Produce a machine-readable checklist artifact suitable for authoring tools, journal workflows, reviewers, and public transparency.
- Keep agentic workflows configurable from fully human to agent-assisted to agent-comparative to fully automated screening, while preserving editorial accountability.
- Ensure all agent outputs are auditable, attributable to tool and model versions, and separable from human decisions.
- Support preprint-first dissemination after verification workflows have completed the configured minimum safety and integrity checks.
- Support public review spaces for pre-publication review, including human, invited, community, and agent-generated reviews under explicit policy.
- Index peer review, editorial input, human verification, translation, code review, data curation, statistical review, and reproducibility work as citable contributions.
- Support contribution credit with contributor identity, role, provenance, conflict-of-interest metadata, verification status, and anti-gaming controls.
- Maintain a queryable knowledge graph that represents the research project over time, including changes in question, evidence, analysis, interpretation, dissemination, review, and decision status.
- Provide privacy, confidentiality, consent, ethics, governance, and embargo controls appropriate for health research.
- Separate clinical or policy-relevant status labels from preprint, exploratory, unreviewed, reviewed, accepted, retracted, superseded, and living-update states.
- Preserve export compatibility with JATS XML, Crossref and DataCite metadata, CSL JSON, BibLaTeX, RIS, ENW, RO-Crate, and repository deposit metadata.
- Provide evidence gates before claiming support for any manuscript platform, preprint server, checklist source, or journal.

### Should

- Generate multiple presentation outputs from the same package, including preprint, blog summary, graphical abstract, slide deck, audio script, video script, long-form article, short-form article, conversational summary, and monograph-like expansion.
- Allow readers to chat with the article using retrieval over article text, data, code, reviews, limitations, author responses, and dissemination outputs.
- Allow readers to declare their positionality, prior knowledge, profession, information need, language, and preferred format before receiving a summary.
- Support reproducibility tiers: full computational rerun, partial rerun, independent replication feasible, conceptual replication feasible, not reproducible from supplied materials, and not applicable.
- Rerun analyses where code, data, environment, and permissions are sufficient.
- Assess why reproduction is blocked when data, code, environment, licenses, compute, or privacy permissions are incomplete.
- Support integration with GitHub, GitLab, Zenodo, OSF, Dataverse, Figshare, institutional repositories, Jupyter, Quarto, Colab, Binder, Code Ocean-like capsules, WorkflowHub, RO-Crate, and Software Heritage.
- Support linked external dissemination outputs such as preprint deposits, Substack or blog posts, press releases, news coverage, conference presentations, podcasts, videos, policy briefs, and social media threads.
- Maintain provenance for external dissemination so that changes across outputs are modeled as state changes rather than contradictions.
- Provide configurable reviewer policies: fully human, human with agent briefing, agent-assisted review, parallel human and agent review, public community review, and agent-only screening for non-decision tasks.
- Support reviewer matching, conflict checks, diversity checks, reviewer briefing packs, review synthesis, decision-letter drafting, and post-publication monitoring as bounded agentic tasks.
- Provide adversarial and bias testing for reviewer matching, triage, review synthesis, and automated recommendations.
- Provide multilingual translation workflows with automated translation, human verification, and language-specific contribution credit.
- Provide public APIs and event streams for indexing systems, knowledge graphs, registries, funders, institutions, and evidence synthesis tools.
- Support funder and payer commissioning workflows where governments, companies, charities, or health systems can fund protocols, datasets, analyses, reviews, updates, or platform improvements.
- Recognize platform-building contributions such as plugin development, workflow design, translation modules, data connectors, and validation tools.

### Could

- Provide modular economic incentives for reviewing, editing, verification, translation, reproducibility work, dataset curation, software packaging, and platform development.
- Support portable contributor profiles connected to ORCID, GitHub, LinkedIn, Google Scholar, ResearchGate, Bluesky, Mastodon, and other public profiles, subject to consent and policy.
- Use zero-knowledge or privacy-preserving attestations for sensitive credentials, conflicts, or governance checks.
- Support decentralized attestations or ledgers for process milestones, reviewer compensation, and contribution credit, while keeping scholarly records off speculative token mechanics.
- Provide interactive reader modes such as clinician view, policy view, methods reviewer view, patient/community view, technical reproducibility view, and education view.
- Provide living evidence and living protocol modes for systematic reviews, guidelines, methods resources, and fast-moving health topics.
- Support market-like commissioning or bounties for unresolved verification tasks, translations, replications, and data reuse projects.
- Support monograph generation where the project has accumulated enough evidence, commentary, and dissemination outputs.

### Won't, for now

- Make final editorial accept or reject decisions without accountable human editorial oversight.
- Claim clinical practice guidance status for a preprint or unreviewed output.
- Publish confidential reviewer reports, rejected manuscripts, private author data, or sensitive health data without explicit policy and consent.
- Treat agentic review as equivalent to expert human peer review unless a journal explicitly configures and discloses that policy.
- Use unverifiable social profile data for conflict-of-interest, expertise, or credit scoring without consent and human appeal pathways.
- Implement speculative token economics as the default incentive model.
- Claim platform support without official-source reconnaissance, fixture-backed contracts, and evidence gates.

## Additional bleeding-edge recommendations

- Add an article package graph that can be queried by agents and humans through the same API.
- Add a checklist composition engine with item-level provenance and deduplication.
- Add a journal-requirements crawler with official-source provenance, stale-source warnings, and human approval gates.
- Add an agentic workflow engine with policy profiles for human-only, agent-assisted, agent-comparative, and automated screening modes.
- Add a contribution ledger that records editorial, review, verification, translation, reproducibility, data, software, and dissemination work.
- Add model-card-like disclosures for every agent used in screening, review, translation, summarization, and reproducibility assessment.
- Add prompt-injection and hidden-instruction screening for manuscripts and reviewer interactions.
- Add public event logs for article lifecycle events, including submissions, reviews, versions, corrections, retractions, updates, external deposits, and dissemination artifacts.
- Add reproducibility evidence artifacts that record what was rerun, what failed, and what remains independently reproducible.
- Add reader-directed output generation so editors choose the validated knowledge package, while readers choose the presentation layer.
