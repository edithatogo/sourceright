# Feature Contract Matrix

This matrix is the canonical requirements document and repo contract for
Sourceright. It defines what the technical preview must preserve, what is
market-readiness hardening, and what is intentionally excluded for now.

## Contract Rules

- `references.csl.json` is the canonical academic-reference source of truth.
- `references.verification.json` records evidence, provenance, confidence,
  conflicts, and review state.
- `review-queue.jsonl` is derived operational work.
- Provider evidence must not silently overwrite canonical CSL.
- Legal citations stay separate from academic CSL.
- Claim/source/provenance features do not assert claim truth.
- MCP write tools must stay dry-run first and require explicit apply.
- Default tests must be deterministic; live services are opt-in external tests.

## MoSCoW Matrix

| Feature | MoSCoW | Current status | Repo contract | Evidence and tests |
| --- | --- | --- | --- | --- |
| Canonical CSL model | Must | Implemented | Academic references are stored as clean CSL JSON keyed by stable `id`. | `docs/src/csl-model.md`, schema and validation tests |
| Verification sidecar | Must | Implemented | Provider evidence and review state stay outside canonical CSL. | `docs/src/verification-sidecar.md`, sidecar schema tests |
| Review queue | Must | Implemented | Review work is reproducible from CSL plus sidecar state. | `review-queue.jsonl` fixtures and report tests |
| CSL validation | Must | Implemented | CLI and tests emit deterministic diagnostics. | `validate-csl --json`, schema inventory tests |
| Provider evidence normalization | Must | Implemented | Provider matches are recorded with provenance and confidence. | provider fixtures and report tests |
| In-text citation reconciliation | Must | Implemented | Missing, uncited, duplicate, and ambiguous citations are surfaced without claiming source truth. | reconciliation tests |
| Export suite | Must | Implemented | XML, ENW, RIS, BibLaTeX, and YAML exports remain deterministic. | export tests and docs |
| CLI contract | Must | Implemented | Public commands keep JSON output stable where advertised. | `tests/cli_end_to_end.rs` |
| MCP read contract | Must | Implemented | Tools, resources, and prompts are discoverable and local-stdio oriented. | MCP distribution checks |
| Plugin registry | Must | Implemented | Runtime discovery validates manifests before exposing capabilities. | `plugins/registry.toml`, plugin tests |
| Fixture benchmark | Must | Implemented | Benchmark claims are fixture-backed regression signals, not public SOTA claims. | `sourceright bench --json` |
| Publication metadata | Must | Implemented | Release, crate, OCI MCP, MCP Registry, and Glama metadata stay version-aligned. | release and MCP distribution tests |
| Security governance | Must | Implemented | CI uses least privilege, dependency review, CodeQL, cargo audit, Scorecard, pinned actions, and quiet dependency automation. | `.github/workflows/security.yml`, Renovate config |
| OJS-compatible screening | Should | Contracted | OJS is the first journal target, implemented through platform-neutral screening outputs. | journal integration docs and fixtures |
| Zotero-first citation sync | Should | Contracted | Sync defaults to preview, records audit logs, and only writes on explicit apply. | citation-sync schema tests |
| Live core providers | Should | In progress | Crossref, DOI, DataCite, OpenAlex, PubMed, and ORCID remain opt-in and sidecar-only. | live-provider configuration docs |
| DOCX/PDF extraction hardening | Should | In progress | Extraction preserves provenance spans and reports OCR limitations honestly. | hardening fixtures |
| Citation disambiguation | Should | In progress | Institutional authors, same-author citations, and style variants route ambiguity to review. | disambiguation fixtures |
| URL/archive integrity | Should | In progress | URL, DOI landing-page, redirect, and archive evidence is recorded as evidence, not truth. | URL/archive fixtures |
| Low-noise writeback | Should | In progress | Suggestions are thresholded, explained, dry-run first, and auditable. | writeback tests |
| Legal citation mode | Could | Implemented as separate model | Legal citation records never force legal citations into academic CSL. | legal roadmap and model tests |
| Claim/source provenance | Could | Implemented as bounded graph | Claims can be linked to sources, but Sourceright does not score claim truth. | provenance tests |
| Additional journal platforms | Could | Planned | Adapters should call the Rust core or CLI/MCP, not reimplement verification logic. | future platform fixtures |
| More citation managers | Could | Planned | EndNote and other managers should follow preview/apply/audit semantics. | sync contract docs |
| HTTP MCP hosting | Could | Deferred | Local stdio remains the current server contract until a separate transport track exists. | MCP docs |
| Automatic final verification | Won't for now | Excluded | The product must not claim examiner-grade final verification until tracks 36-40 prove it. | product guidelines |
| AI authorship detection | Won't for now | Excluded | Citation errors are not treated as proof of AI authorship. | report wording tests |
| Silent provider overwrite | Won't for now | Excluded | Provider data never mutates canonical CSL without review or explicit apply. | sidecar boundary tests |
| Default live external CI | Won't for now | Excluded | OJS, Zotero, registry, and provider live tests require opt-in credentials and sample data. | external-test plan |

## External Test Contract

External tests should be added as opt-in smoke suites:

- OJS fixture and live-test-instance smoke for submission screening.
- CLI installed-binary smoke for `init`, `validate-csl`, `report`, `export`,
  `bench`, `citation-sync`, and `mcp status`.
- MCP stdio transcript smoke for tool/resource/prompt discovery and read-only
  resources.
- Zotero or Better BibTeX preview/apply smoke using a disposable library.
- Registry smoke for crates.io, docs.rs, GHCR, official MCP Registry, Glama,
  and any future package-manager channels.
