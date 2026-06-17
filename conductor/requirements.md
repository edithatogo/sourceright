# Sourceright Requirements Contract

This is the Conductor-owned requirements document for Sourceright. It mirrors
the public feature matrix but is stricter about implementation evidence,
Conductor track ownership, review gates, and claim boundaries.

## Contract Rules

- The current repository is the source of truth.
- `references.csl.json` is canonical clean academic bibliographic data.
- `references.verification.json` stores provider evidence, provenance,
  confidence, conflicts, and review state.
- `review-queue.jsonl` is derived operational review work.
- Provider evidence must not silently overwrite canonical CSL.
- Legal citations remain separate from academic CSL.
- Claim/source/provenance work must not assert claim truth.
- Write-capable CLI, MCP, citation-manager, or plugin operations must be
  dry-run first, audit logged, schema checked, and explicit on apply.
- Default tests must be deterministic and fixture-backed; live providers,
  OJS, Zotero, registry, and hosted service tests are opt-in.
- A feature is not complete until its track lists evidence, tests, docs, and
  public-claim wording.
- Evidence levels and allowed claims are recorded in
  `conductor/evidence-ledger.json`.
- Track sequencing follows `conductor/implementation-order.md`.
- Host compatibility follows `conductor/plugin-compatibility-matrix.md`.
- Release channels follow `conductor/release-channels.md`.
- Secrets and live tests follow `conductor/secrets-and-live-tests.md`.
- Deprecation follows `conductor/deprecation-policy.md`.
- Architectural decisions are recorded under `conductor/adrs/`.

The downstream submission tracks are grouped as:

- Submission requirements contracts, which own the shared surface model.
- arXiv upstream submission contracts for tracks 78, 79, 80, 81.

## Submission Requirements Contracts

Submission requirements contracts define the approval-gated path for external
registries, marketplaces, plugin hosts, and upstream repositories. The shared
contract is tracked in `conductor/submission-contracts.md`, the inventory in
`conductor/submission-requirements.json`, and the packet families in
`conductor/submission-packets/`.

## Self-improving Submission Readiness

Self-improving submission readiness keeps the submission workflow machine
readable and workflow-checked. It is owned by track 82 and the repo-health
controls it defines.

## MoSCoW Matrix

| Requirement | MoSCoW | Track owner | Completion contract | Overclaim guard |
| --- | --- | --- | --- | --- |
| Canonical CSL model | Must | 04 | CSL stays clean and schema-valid. | Do not describe sidecar/provider data as canonical. |
| Verification sidecar | Must | 05 | Evidence, confidence, conflicts, and review state persist outside CSL. | Do not imply provider records overwrite bibliography entries. |
| Review queue | Must | 10 | Ambiguity and unresolved work are reproducible and partitionable. | Do not claim full automation where human review remains. |
| Provider evidence normalization | Must | 06, 20, 37 | Provider evidence is normalized and sidecar-only. | Do not claim live verification unless the provider adapter and opt-in smoke pass. |
| Citation reconciliation | Must | 09, 38 | In-text citations reconcile to references with ambiguity handling. | Do not claim source truth or AI authorship detection. |
| Export suite | Must | 11 | XML, ENW, RIS, BibLaTeX, and YAML outputs are deterministic. | Do not claim citation-manager sync when only file export is proven. |
| CLI contract | Must | 12, 24 | JSON outputs and help text are stable and tested. | Do not claim installed-user smoke without binary install proof. |
| MCP contract | Must | 12, 17, 18, 27 | Read contracts, distribution metadata, and write dry-runs are tested. | Do not claim hosted HTTP MCP until a transport track exists. |
| Security automation | Must | 26, 41, 42 | Renovate, Dependabot alerts, CodeQL, cargo audit, Scorecard, and Copilot prep are tracked. | Do not claim alerts are closed unless GitHub alert state is observed. |
| Publication inventory | Must | 25, 32, 33, 43 | Accepted, submitted, prepared, and deferred registries are separated. | Do not claim registry acceptance without public listing evidence. |
| Documentation parity | Must | 28, 30, 35 | Docs-site and source docs stay aligned. | Do not describe docs cutover as finished until redirect/archive checks pass. |
| Coverage and robustness evidence | Must | 31, 34 | Coverage, robustness, mutation/property/load claims are reproducible. | Do not use coverage badges as proof without current artifacts. |
| Self-improving submission readiness | Should | 82 | Keep submission readiness machine-readable and workflow-checked. | Do not auto-promote external submission claims without approval. |
| DOCX/PDF extraction hardening | Should | 36 | Real document fixtures produce provenance spans and OCR diagnostics. | Do not claim robust extraction until messy DOCX/PDF fixtures pass. |
| Live core providers | Should | 37 | Crossref, DataCite, OpenAlex, PubMed, DOI, and ORCID opt-in smokes pass. | Do not run live checks in default CI or hide rate-limit/cache behavior. |
| Citation disambiguation | Should | 38 | Institutional authors, same-author/year, styles, and ambiguity cases are tested. | Do not auto-correct ambiguous citations without review. |
| URL/archive integrity | Should | 39 | URL, DOI landing-page, redirect, and archive evidence is recorded. | Do not treat reachability as truth of the cited claim. |
| Low-noise writeback | Should | 40 | Dry-run plans are thresholded, explained, and auditable. | Do not write silently to CSL, Zotero, or external systems. |
| OJS proof | Should | 16, 45 | OJS fixture and optional live/test-instance smoke prove the journal contract. | Do not claim journal compatibility beyond tested adapters. |
| Citation-manager proof | Should | 21, 45 | Zotero/EndNote tests prove preview/apply/audit semantics. | Do not claim live sync from export-only paths. |
| Plugin/provider roadmap delivery | Should | 19, 20, 46 | Every registry entry has status, owner track, fixtures, and docs. | Do not show planned plugins as implemented. |
| Public API provider adapters | Should | 48 | Unpaywall, OpenCitations, arXiv, Europe PMC, and related public providers have fixtures and opt-in live smokes. | Do not treat one provider success as evidence for another provider. |
| Licensed provider adapters | Could | 49 | Dimensions, Scopus, and Web of Science have BYO-key auth, privacy, cache, and skip contracts. | Do not run licensed providers in default CI or imply bundled access. |
| Repository record adapters | Should | 50 | Zenodo, OSF, Figshare, Dataverse, and institutional records have separate evidence contracts. | Do not bundle repositories under one untested provider claim. |
| Citation-manager adapters | Should | 51 | Zotero sync and EndNote handoff have separate preview/apply/export proof. | Do not claim Zotero live sync from EndNote/RIS export proof. |
| Non-provider pipeline plugins | Should | 52 | Matching, recency/retractions, claim-source relevance, and extraction plugins have fixtures and contract gates. | Do not promote planned plugin manifests without behavior evidence. |
| CourtListener legal provider | Could | 53 | Legal-provider evidence writes legal reports only. | Do not merge legal citations into academic CSL. |
| Demo proof surfaces | Could | 54 | GitHub Pages and Streamlit demos have separate render/browser/server smoke gates. | Do not use demos as proof of live-provider behavior. |
| Benchmark robustness contract | Must | 55 | Fixture and stress benchmark artifacts remain reproducible and labelled. | Do not claim external benchmark comparability from local fixtures. |
| MCP registry release binding | Must | 56 | `server.json`, OCI labels, release artifacts, and MCP Registry listing align. | Do not claim registry acceptance before the public listing shows the version. |
| Smithery distribution | Should | 57 | Smithery has a validated Streamable HTTP or MCPB/local package path before public claims. | Do not claim Smithery availability from `server.json` alone. |
| Mature Zotero plugin | Should | 58 | Zotero has installable package/distribution notes, preview/apply/audit tests, and disposable-library proof. | Do not claim mature Zotero support from RIS/CSL export alone. |
| Other citation managers | Should | 59 | EndNote reference-checking handoff and other manager decisions have explicit adapters or deferrals. | Do not collapse all citation managers into the Zotero contract. |
| Mature OJS plugin | Should | 60 | OJS has installable PKP/OJS plugin packaging, permissions, fixtures, and optional test-instance proof. | Do not claim OJS plugin readiness from a generic report alone. |
| Streamlit app publication | Could | 61 | Streamlit is deployable, synthetic-data-only, server-smoked, and documented separately from static demos. | Do not treat demo availability as production service readiness. |
| Expanded normalisers | Should | 62 | Additional scholarly, economics, grey literature, repository, and search-provider normalisers are assessed and tracked. | Do not list a database/provider as supported without a manifest and evidence. |
| Plugin supply-chain maturity | Must | 63 | Plugins use provenance, signing/pinning, compatibility, status, deprecation, sandbox, and no-submodule policy gates. | Do not split plugins into submodules unless an independent lifecycle justifies it. |
| GitHub-side governance | Must | 64 | Branch protection, required checks, Copilot, Renovate, code scanning, coverage, labels, and release environments are verified or documented. | Do not claim settings are enabled unless GitHub state is observed. |
| AI client MCP packaging | Should | 65 | Claude, Codex, GitHub Copilot, and generic MCP clients have install/config contracts or explicit deferrals tied to MCP transcript proof. | Do not describe Sourceright as a Claude, Codex, or Copilot plugin until an installable host package or accepted listing exists. |
| VS Code extension packaging | Could | 66 | VS Code/editor integration has a VSIX-ready package or explicit deferral that reuses CLI/MCP diagnostics. | Development `.vscode` settings are not a VS Code extension. |
| Microsoft Word add-in packaging | Could | 67 | Word integration has an Office Add-in manifest/taskpane or explicit deferral with range provenance and reversible write plans. | DOCX extraction is not Word add-in support. |
| LibreOffice extension packaging | Could | 68 | LibreOffice integration has `.oxt`/UNO packaging or explicit deferral with Writer range provenance and local install smoke. | ODT/DOCX processing is not LibreOffice extension support. |
| Marketplace submission evidence | Should | 69 | Host marketplaces and directories record accepted, prepared, or deferred status with version/date/install metadata where applicable. | Prepared metadata is not public marketplace acceptance. |
| Branch inventory and stale-work closure | Should | 44 | Unmerged branches are triaged, merged, archived, or superseded with evidence. | Do not assume old branches are safe to merge. |
| Legal citation mode | Could | 13, 46 | Legal records use a separate model and public-provider strategy. | Do not claim legal filing compliance. |
| Claim/source provenance | Could | 14, 46 | Claim/source links are reported without truth scoring. | Do not claim fact checking or claim verification. |
| Additional registries | Could | 43 | Glama, Smithery, package managers, npm/PyPI wrappers have maintained manifests and acceptance evidence. | Do not publish wrappers that reimplement Rust core behavior. |
| Additional journal platforms | Could | 45 | New adapters reuse the OJS/platform-neutral screening contract. | Do not fragment journal semantics per platform. |
| Automatic final verification | Won't for now | 47 | Excluded until tracks 36-40 and external proof suites pass. | Use "technical preview" and "structured triage" wording. |
| AI detector | Won't for now | 47 | Excluded by policy and tests. | Never equate citation errors with AI authorship. |

## Completion Evidence Levels

| Level | Meaning | Required evidence |
| --- | --- | --- |
| Contracted | Track exists with spec, plan, test matrix, and owner paths. | Conductor track and matrix row. |
| Scaffolded | CLI/schema/docs/test shell exists but does not prove behavior. | Passing contract tests and explicit limitation wording. |
| Fixture-backed | Deterministic local fixtures prove behavior. | CI/local test names and fixture paths. |
| Opt-in live proven | Live service smoke works with explicit opt-in and no default secrets. | Smoke script, redacted run notes, and cache/rate-limit policy. |
| Publicly accepted | External registry/platform shows the artifact. | Public URL, version, date, and acceptance status. |

## Optional Future Roadmap Requirements

These requirements are explicitly optional and do not block the mature stable
release. They should not be described as implemented, supported, or committed
until a future Conductor track promotes them into the main MoSCoW matrix with
evidence.

| Optional requirement | Horizon | Rationale | Promotion trigger | Claim boundary |
| --- | --- | --- | --- | --- |
| Hosted team workspace | Future | Multi-user teams may want shared project state, review assignments, and audit history. | Repeated pilot demand plus an access-control and data-residency design. | Do not claim SaaS, collaboration, or institutional hosting support. |
| Web UI for review queues | Future | Reviewers may prefer browser triage over JSONL/CLI workflows. | Stable queue schema, role model, and accessibility design. | Do not claim full editorial platform support. |
| Browser extension for manuscript systems | Future | Users may want screening inside submission portals or repository pages. | Mature OJS/plugin proof plus a stable web-extension security model. | Do not automate third-party sites without permission. |
| Word/LibreOffice add-ins | Promoted to tracks 67-68 | Authors may want in-document citation highlighting and writeback. | Stable low-noise writeback plan and document-range provenance. | Do not claim safe in-document editing before reversible change plans exist. |
| VS Code / IDE extension | Promoted to track 66 | Agent and writing workflows may benefit from inline reference diagnostics. | Stable CLI JSON diagnostics and MCP resources. | Do not reimplement core logic in the extension. |
| Full HTTP MCP service | Future | Hosted or remote MCP clients may need Streamable HTTP instead of local stdio. | Authentication, tenancy, rate-limit, and deployment threat model. | Do not claim hosted MCP from local stdio support. |
| Organization policy packs | Future | Journals, universities, or funders may want custom rules. | Policy schema stability and signed/pinned policy-pack support. | Do not claim institution-specific compliance without approved policy packs. |
| Public benchmark corpus | Future | External comparability requires shared data, metrics, and governance. | Rights-cleared corpus, versioned baselines, and external evaluation protocol. | Do not claim SOTA or public benchmark leadership before this exists. |
| ML-assisted matching | Future | ML could improve noisy citation matching and extraction ranking. | Deterministic baseline, explainability, privacy, and fallback rules. | Do not replace deterministic/auditable checks with opaque scoring. |
| OCR/layout engine integration | Future | Scanned PDFs and complex documents need robust layout-aware extraction. | Fixture corpus and licensing review for OCR/layout engines. | Do not claim OCR-grade extraction from text-layer parsing. |
| Multilingual citation workflows | Future | Non-English manuscripts and references may need locale-specific parsing. | Multilingual fixtures and locale-aware CSL/provider rules. | Do not claim multilingual support from English-only fixtures. |
| Legal-provider expansion | Future | Legal citation users may need broader jurisdiction/provider support. | Jurisdiction-specific fixtures and public-provider access. | Do not claim legal advice or filing compliance. |
| Research integrity signals | Future | Users may want expressions of concern, trials registration, funder, and ethics metadata. | Provider evidence sources and policy wording are stable. | Do not claim misconduct, fraud, or truth scoring. |
| Institutional repository deposit workflow | Future | Universities may want deposit-ready metadata and repository handoff. | Repository provider tracks prove deposit metadata and handoff contracts. | Do not claim repository submission automation without live proof. |
| LLM-agent review assistants | Future | Agents could help explain review queues and draft low-noise suggestions. | Prompt contracts, audit logs, and hallucination/failure tests. | Do not let agents silently change canonical CSL or external systems. |
| Enterprise audit exports | Future | Larger organizations may need SIEM/GRC-friendly evidence exports. | Stable evidence ledger, audit schema, and security review. | Do not claim enterprise compliance before controls are audited. |

## Automatic Track Progression Contract

Every remaining track must include these phases:

1. `discover`: subagents inspect owned paths and public docs for drift.
2. `spec-lock`: update spec/plan/test matrix before implementation.
3. `implement`: apply the smallest reviewable slice.
4. `self-check`: run targeted local checks and update evidence notes.
5. `conductor-review`: run `$conductor-review` against the track.
6. `autofix`: apply review fixes unless they would broaden scope or require
   credentials/destructive action.
7. `progress`: move to the next independent slice only after review findings
   are closed or recorded as deferred.

Parallelization is allowed only when owned paths do not overlap. Subagents must
return exact files inspected and must not mark a requirement complete without
named evidence.
