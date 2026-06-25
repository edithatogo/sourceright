# Sourceright Conductor Tracks

Sourceright work is organised as numbered Conductor tracks. Tracks 00-12 establish the academic-reference verification product; track 15 adds reporting as a first-class audit surface; tracks 13-14 reserve explicit roadmap space for legal citation verification and broader claim/source/provenance work; track 16 covers journal workflow integrations; tracks 17-29 cover the runtime, provider, sync, benchmark, recency-evidence, CLI, release, CI, MCP distribution, docs, and performance-hardening work that were deferred from the ZIP audit and SOTA review; tracks 30-32 extend the roadmap into a Starlight/Astro docs site, measurable QA hardening, and publication/provenance automation; tracks 33-35 finish the broader operational roadmap with live public publishing, measurable coverage evidence, and the final docs cutover; tracks 36-40 define the examiner-grade audit hardening phase needed to move from structured reference triage toward final-verifier confidence; tracks 41-47 split the remaining governance work into contract, GitHub automation, registry completion, stale branch triage, external proof, plugin delivery, and anti-overclaim gates; tracks 48-56 break the plugin/provider/demo/benchmark/registry roadmap into implementation lanes; tracks 57-64 cover external publication maturity, mature host plugins, expanded normalisers, SOTA plugin handling, and GitHub-side governance; tracks 65-69 turn AI-client, editor, office-suite, and marketplace packaging into explicit host contracts; track 70 keeps prepared/deferred release surfaces auditable between live publication checks; track 71 adds parallel arXiv submission-platform adapter contracts after shared dependencies are updated; tracks 72-81 convert the remaining prepared/deferred external submissions into hardened, approval-gated contracts, with arXiv split into granular requirements, current-platform hardening, legacy-platform hardening, and upstream submission evidence; track 82 adds the self-improving submission-readiness inventory, workflow, and repo-health loop; tracks 83-85 extend the journal roadmap to Janeway with a separate reconnaissance, packaging, and disposable-smoke lane before any publication claim; tracks 86-87 add a proprietary journal-platform matrix and a self-improving adapter registry/workflow lane.

| Track | Status | Purpose |
| --- | --- | --- |
| 00 Public repo infrastructure | completed | Public GitHub repo, license, CI/CD, Pages, security, release plumbing. |
| 01 Product roadmap | completed | Product boundaries, v1 academic workflow, later legal and claim/provenance roadmap. |
| 02 Legacy audit | completed | Audit imported `humanizer-next` behavior, fixtures, and reusable ideas. |
| 03 Reference intake | completed | Extract references from text, Markdown, DOCX-adapter text, PDF text layers, and OCR-gated scans. |
| 04 CSL canonical model | completed | Use CSL JSON as the canonical reference data model. |
| 05 Verification sidecar | completed | Store provenance, provider matches, confidence, conflicts, and review state. |
| 06 Academic providers | completed | Normalize Crossref, DOI resolver, DataCite, OpenAlex, PubMed/NCBI, and ORCID provider evidence. |
| 07 Standardisation cleaning | completed | Normalize fields, identifiers, duplicates, names, containers, and style-independent reference data. |
| 08 Conflict resolution | completed | Deterministic provider merge, unresolved-conflict preservation, and explainable conflict reports. |
| 09 In-text citation reconciliation | completed | Match body citations to references and report missing/uncited/order issues. |
| 10 Manual agent review | completed | Agent/subagent manual verification queues after automated checks. |
| 11 Export suite | completed | XML, ENW, RIS, BibLaTeX, and YAML outputs with validation. |
| 12 CLI and MCP | completed | CLI command family and MCP server tools/resources/prompts. |
| 13 Legal citations | completed | Separate legal citation model with jurisdiction/provider hints and legal review issues. |
| 14 Claim/source/provenance | completed | Claim/source graph, citation links, and document-level provenance issue reports. |
| 15 Reference reporting | completed | Reference integrity and AI-related citation-error reports. |
| 16 Journal workflow integrations | completed | OJS-first and platform-neutral editorial submission screening contracts. |
| 17 MCP server runtime | completed | Actual local MCP server transport for existing read-only MCP contracts. |
| 18 MCP write tools audit dry-run | completed | Write-capable MCP tools gated by dry-run plans, schema validation, audit logs, and explicit apply. |
| 19 Runtime plugin loading | completed | Safe runtime plugin discovery, manifest validation, and capability loading. |
| 20 Live provider adapters | completed | Opt-in live provider adapters that write sidecar evidence without mutating canonical CSL. |
| 21 Citation manager live sync | completed | Zotero-first live sync with preview, audit logs, conflicts, and explicit apply. |
| 22 Benchmark runner automation | completed | Executable fixture-backed benchmark runner with deterministic baseline comparison. |
| 23 Provider-backed recency evidence | completed | Retraction, correction, preprint, superseded-guideline, and age evidence surfaced in policy/reporting. |
| 24 CLI entrypoint integration | completed | First-class CLI/help/docs integration for the bench and citation-sync entrypoints plus a final repo-wide consistency pass. |
| 25 Release and registry readiness | completed | Crates.io, GitHub Release, package metadata, and registry-readiness gates before public publication. |
| 26 CI supply-chain hardening | completed | Stricter CI gates for dependency risk, docs, command smoke tests, coverage, linting, and release provenance. |
| 27 MCP distribution | completed | Registry-ready MCP metadata and packaging for official MCP registry, Smithery, Glama, and local stdio installs. |
| 28 Docs and discoverability | completed | Docs build validation, command references, install pages, and a decision on mdBook versus Starlight/Astro. |
| 29 Performance and robustness | completed | Profiling, deterministic benchmark thresholds, fuzzing, and larger fixture stress coverage. |
| 30 Starlight Astro docs site | completed | Replace mdBook with a Starlight/Astro documentation site with CI, search, and deployment parity. |
| 31 Quality assurance hardening | completed | Coverage thresholds, mutation testing, property tests, load tests, edge tests, and end-to-end verification. |
| 32 Publishing governance and provenance | completed | Release provenance, attestation, dependency governance, and registry publication automation. |
| 33 Public release and registry submission | completed | Execute the live crates.io, GitHub Release, GHCR, official MCP registry, Smithery, and Glama publication sequence. |
| 34 Coverage verification and reporting | completed | Make the 85 percent coverage floor measurable, reproducible, and visible in hooks, CI, and reports. |
| 35 Public docs cutover and launch | completed | Finalise the Starlight/Astro docs migration, redirects, and archive cutover for the public site. |
| 36 Document extraction hardening | completed | Real DOCX/PDF extraction, OCR diagnostics, provenance spans, and messy manuscript fixtures. |
| 37 Live core provider verification | completed | Opt-in live Crossref, DataCite, OpenAlex, PubMed/NCBI, and DOI-resolution evidence with caching and sidecar-only writes. |
| 38 Citation matching disambiguation | completed | Institutional authors, same-author same-year citations, `et al.` variants, numeric styles, and low-noise ambiguity handling. |
| 39 URL archive integrity | completed | URL reachability, DOI landing pages, redirects, archive evidence, and broken-link reporting. |
| 40 Low-noise writeback suggestions | completed | Dry-run CSL/citation-manager writeback plans with thresholds, explanations, audit logs, and explicit apply semantics. |
| 41 Security, publication, and contract governance | completed | Dependency-alert closure, quiet Renovate automation, publication inventory, and canonical feature/design contract docs. |
| 42 GitHub automation and alert operations | completed | Verify GitHub-side Copilot, Dependabot/code-scanning alert state, installed apps, and low-noise automation settings. |
| 43 Publication registry completion | completed | Close the accepted/prepared/deferred registry gaps for Glama, Smithery, GHCR evidence, and future package managers. |
| 44 Branch triage and stale-work closure | completed | Triage unmerged branches against current `main`, merging, superseding, or archiving with evidence. |
| 45 External proof suites | completed | Add opt-in proof suites for installed CLI, MCP stdio, OJS, citation managers, live providers, and registries. |
| 46 Plugin and provider roadmap delivery | completed | Convert every planned plugin/provider manifest into an owned implementation, fixture, or explicit deferral track. |
| 47 Contract evidence and overclaim gates | completed | Enforce the requirements/design contracts so docs, releases, and tracks cannot overclaim completion. |
| 48 Public API provider adapters | completed | Implement or explicitly defer Unpaywall, OpenCitations, arXiv, Europe PMC, and related public-provider adapters with fixtures. |
| 49 Licensed BYO-key provider adapters | completed | Implement Dimensions, Scopus, and Web of Science contracts with auth, cache, privacy, and opt-in live gates. |
| 50 Repository record provider adapters | completed | Split repository-record evidence into Zenodo, OSF, Figshare, Dataverse, and institutional-repository lanes. |
| 51 Citation-manager adapter proof | completed | Separate Zotero preview/apply/audit sync from EndNote ENW/RIS handoff proof. |
| 52 Non-provider pipeline plugins | completed | Implement local bibliographic matching, recency/retraction policy checks, claim-source relevance, and DOCX/PDF extraction plugin contracts. |
| 53 CourtListener legal provider | completed | Implement CourtListener as a legal-citation provider without touching academic CSL boundaries. |
| 54 Demo public surface proof | completed | Prove GitHub Pages and Streamlit demos separately with static render, browser, and server smoke gates. |
| 55 Benchmark robustness contract | completed | Keep benchmark, stress, robustness, and metrics artifacts first-class evidence gates. |
| 56 MCP registry release binding | completed | Validate `server.json`, OCI labels, release artifacts, and official MCP Registry publication binding. |
| 57 Smithery distribution readiness | completed | Prepare and validate the Smithery MCPB/local package path while keeping public listing claims gated. |
| 58 Mature Zotero plugin | completed | Deliver a ready Zotero plugin or adapter package with `.xpi`, preview/apply/audit semantics, tests, and public distribution notes. |
| 59 Other citation-manager integrations | completed | Harden EndNote reference-checking handoff plus Mendeley, Paperpile, JabRef, RefWorks, and CSL-compatible import/export decisions. |
| 60 Mature OJS plugin | completed | Deliver an installable OJS/PKP plugin path with platform-neutral screening, permissions, packaging, and Plugin Gallery readiness. |
| 61 Streamlit app publication and hardening | completed | Make the Streamlit demo deployable, synthetic-data-only, smoke-tested, and clearly separated from the static Pages demo. |
| 62 Expanded normaliser/provider catalogue | completed | Assess and track additional scholarly, economics, grey literature, repository, and search-provider normalisers. |
| 63 Plugin packaging and supply-chain maturity | completed | Define no-submodule policy, signing, provenance, versioning, compatibility, deprecation, and sandbox/network gates for plugins. |
| 64 GitHub-side governance additions | completed | Configure or document required GitHub-side settings for branch protection, Copilot, Renovate, code scanning, coverage, labels, and releases. |
| 65 AI client MCP packaging | completed | Document and prove Claude, Codex, GitHub Copilot, and generic MCP-client packaging boundaries without overclaiming host plugins. |
| 66 VS Code extension packaging | completed | Define a thin VS Code/editor package contract that reuses CLI/MCP diagnostics and stays preview-first for writes. |
| 67 Microsoft Word add-in packaging | completed | Define Office Add-in packaging, document-range provenance, sideload/AppSource boundaries, and reversible dry-run editing semantics. |
| 68 LibreOffice extension packaging | completed | Define LibreOffice `.oxt`/UNO packaging, Writer range mapping, and local install smoke boundaries. |
| 69 Marketplace submission evidence | completed | Record accepted, prepared, and deferred marketplace evidence for host packages and client directories. |
| 70 Release surface refresh cadence | completed | Add a repeatable local audit cadence for prepared/deferred release surfaces without turning stale metadata into acceptance claims. |
| 71 arXiv submission platform adapters | completed | Add fixture-backed current and legacy arXiv submission-platform adapter contracts with shared dependency gates and parallel lanes. |
| 72 submission requirements contracts | completed | Define host-by-host requirements, evidence gates, and approval boundaries before external registry, marketplace, plugin, or upstream submissions. |
| 73 MCP directory submission hardening | in_progress | Mature Official MCP Registry refresh, Smithery, and Glama submission evidence with bundle, metadata, smoke, and listing gates. Server-card fix implemented 2026-06-10; browser listing verification recorded; GitHub Pages dot-path 404 blocker open. |
| 74 citation-manager publication hardening | in_progress | Harden Zotero and EndNote publication paths with package decisions, smoke proof, import/export checks, and exact plugin-claim boundaries. Spec, plan, and test matrix defined. |
| 75 journal-platform publication hardening | in_progress | Mature OJS/PKP package submission and keep arXiv upstream work on separate granular tracks. Gallery search shows no match; OJS plugin skeleton exists. |
| 76 AI client extension publication hardening | in_progress | Define Claude, Codex, Copilot, Gemini CLI, and Qwen CLI extension/package paths or no-package decisions. Package decisions contract, transcript smoke, and docs updates defined. |
| 77 VS Code and Open VSX publication hardening | in_progress | Turn the deferred VSIX/Open VSX contract into package, install-smoke, and listing evidence when demand justifies it. Currently deferred - activate only when demand confirmed. |
| 78 arXiv upstream requirements reconnaissance | in_progress | Search and document arXiv/submit-ce and arXiv/arxiv-submission-core contribution, schema, testing, security, and submission requirements. Requirements matrix documented with handoff to Tracks 79/80. |
| 79 arXiv submit-ce maturity hardening | in_progress | Raise the current arXiv submit-ce adapter to upstream-ready maturity, stability, and testing evidence before submission. journal-screen smoke recorded 2026-06-10; integration hook proposed on issue #72. |
| 80 arXiv submission-core maturity hardening | in_progress | Raise the legacy arXiv arxiv-submission-core adapter to migration-safe maturity, stability, and testing evidence before submission. Repo confirmed legacy/inactive per maintainer; evidence packet and migration path documented. |
| 81 arXiv upstream submission and acceptance | in_progress | Create and track arXiv upstream issues/PRs only after requirements and hardening gates pass and approval is recorded. Issues filed: submit-ce #72 (open), submission-core #88 (closed/legacy). |
| 82 self-improving submission and health loop | in_progress | Keep submission readiness machine-readable, workflow-checked, and tied to a repo-health target of at least 9.5 before external claims. Inventory schema, readiness script, and policy test defined. |
| 83 Janeway platform reconnaissance | in_progress | Map Janeway plugin hooks, packaging, install surfaces, and integration options before any plugin or service claim. |
| 84 Janeway plugin package hardening | in_progress | Turn the Janeway contract into a packageable adapter skeleton with explicit permissions, metadata, and fixture-backed checks. |
| 85 Janeway install and smoke evidence | in_progress | Prove disposable-instance Janeway install and screening smoke, or record blockers, before any publication claim. |
| 86 Proprietary journal-platform matrix | in_progress | Map ScholarOne, Editorial Manager, eJournalPress, and Manuscript Manager integration patterns, API access, and test environments. |
| 87 Self-improving platform registry | in_progress | Build the agentic registry, skill/workflow hooks, and track-synthesis loop that keeps platform evidence current. |
| 88 Open source documentation and maintenance hardening | planned | Audit and harden Sourceright public GitHub documentation, contributor experience, release hygiene, community-health files, and maintenance posture. |
