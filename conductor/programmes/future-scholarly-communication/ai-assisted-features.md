# AI-Assisted Feature Gap Analysis

## Purpose

This document records AI-assisted features suggested in current research, platform practice, and the Transforming Health design discussion that are not yet fully represented in Sourceright.

## Already represented in Sourceright

- Citation integrity reports.
- Reference validation and sidecar evidence.
- Review queues for manual verification.
- Journal screening adapter contracts.
- OJS, Janeway, proprietary platform, and arXiv submission-platform tracks.
- MCP-facing tool surfaces.
- Dry-run and explicit apply patterns for write-capable workflows.
- Self-improving platform registry concept.

## AI-assisted features to add

| Feature | Status | Recommendation |
| --- | --- | --- |
| Journal requirement sourcing agent | Missing | Add Track 91 to source official instructions, create contracts, assess submissions, and produce MoSCoW recommendations. |
| Reporting checklist discovery agent | Missing | Add Track 92 to detect study design, source checklists, deduplicate items, assess gaps, and optionally implement changes. |
| Machine-readable research package builder | Missing | Add Track 89 to emit JSON, YAML, JATS XML, CSL JSON, and RO-Crate-compatible packages. |
| Agentic intake triage | Partial | Extend journal-screening from citation integrity to scope, article type, ethics, AI use, data, code, and checklist readiness. |
| Reviewer matching recommender | Missing | Add bounded, auditable reviewer recommendations with conflict checks, diversity checks, and human override. |
| Reviewer briefing packs | Missing | Generate structured briefs from manuscript, references, checklists, reproducibility evidence, and prior reviews. |
| Agentic peer review | Missing | Support policy-labeled agent reviews as separate contributions, never hidden as human review. |
| Parallel human-agent review | Missing | Support comparison workflows where human and agent reviews are both recorded and compared. |
| Review synthesis and decision-letter drafting | Missing | Add as decision-support only, with editor attribution and audit logs. |
| Hidden prompt and prompt-injection screening | Missing | Add screening for manuscript text, supplementary files, code comments, and reviewer-facing instructions. |
| Reproducibility rerun agent | Missing | Add Track 94 to rerun code where possible and classify reproducibility tiers. |
| Replication feasibility assessment | Missing | Add structured assessment where data or code are incomplete. |
| Knowledge graph builder | Missing | Add Track 89 to represent evolving questions, claims, evidence, reviews, updates, and dissemination outputs. |
| Multi-format output generator | Missing | Add Track 95 for preprint, blog, presentation, audio, video, short and long form, and monograph views. |
| Chat with article | Missing | Add reader-configurable retrieval and summarization over the article package. |
| Positionality-aware summaries | Missing | Let users request summaries based on role, interest, language, and prior knowledge, with safety controls. |
| Multilingual translation workflow | Missing | Add translation plus human verification as credited workflow steps. |
| Public contribution ledger | Missing | Add Track 96 for peer review, editorial input, verification, translation, code, data, and platform contributions. |
| Funder and payer commissioning | Missing | Add optional commissioning contracts for protocols, datasets, analyses, reviews, and platform improvements. |
| External dissemination tracker | Missing | Track blog posts, Substack, press releases, news media, conference outputs, social media, and preprint deposits as graph nodes. |
| Post-publication monitoring agent | Partial | Extend recency and retraction evidence to comments, reviews, code breakage, new evidence, and living updates. |
| Model and tool cards for editorial agents | Missing | Require every agent to publish scope, limitations, version, validation evidence, and policy profile. |
| Anti-gaming analytics | Missing | Monitor contribution inflation, reciprocal reviewing, conflict evasion, citation gaming, and agent-generated low-value work. |

## AI-assisted features identified by others but not yet included

- AI-generated official review proposals, as explored in emerging AI-era publishing designs, should be supported only as labeled agent review or comparative review.
- MCP-accessible publishing platforms for human and AI scientists should be monitored because they indicate a direction toward agent-native scholarly infrastructure.
- OpenReview-derived review datasets and specialized reviewer models show that AI review and rebuttal support is now a practical research area, but Sourceright should keep it policy-labeled and human-governed.
- AI-assisted author self-evaluation before submission should be incorporated as a pre-submission readiness workflow.
- AI-assisted rebuttal drafting and review response analysis should be added as optional author tools with disclosure and human responsibility boundaries.
- AI-assisted contribution extraction should infer possible CRediT and extended roles, but contributors must approve roles before publication.
- AI-assisted evidence surveillance should be used for living evidence records, but update decisions should remain governed by explicit methods and editors.

## Bleeding-edge recommendations

1. Add a contract-native workflow engine where each agent reads and writes typed artifacts.
2. Add side-by-side human and agent review comparison metrics.
3. Add formal evaluation suites for hallucination, hidden prompt compliance, citation accuracy, checklist accuracy, and review usefulness.
4. Add reproducibility certificates that distinguish rerun success from independent replication.
5. Add open graph APIs so external indexers can query not only papers, but claims, reviews, updates, code, datasets, and contributions.
6. Add contribution credit for human verification of AI outputs.
7. Add economic and governance models for rewarding high-value editorial and verification work without enabling review markets to dominate editorial judgement.
8. Add persona-aware but evidence-bounded reader interfaces so summaries change presentation, not evidentiary status.
9. Add a living project timeline that answers how the research question, evidence, analysis, and interpretation changed over time.
10. Add platform-building as a publishable and creditable contribution type.

## Safety boundaries

- Agent review must be labeled as agent review.
- Agent summaries must cite or link to retrieved package evidence.
- Agentic writeback must be dry-run first.
- Agentic recommendations must be appealable.
- Reviewer matching must expose conflict checks and editor overrides.
- Clinical or policy-facing outputs must display evidence status and limitations.
- Confidential manuscripts must not be sent to external AI systems unless policy, consent, and contracts allow it.
