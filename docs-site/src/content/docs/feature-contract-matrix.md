---
title: Feature contract matrix
description: Canonical Sourceright requirements, MoSCoW priority, and repo contract.
---

This matrix is the public requirements document and repo contract for
Sourceright. The Conductor-owned planning authority is
`conductor/requirements.md`; this page mirrors the same contract for users and
contributors. It defines what the technical preview must preserve, what is
market-readiness hardening, and what is intentionally excluded for now.

## Contract rules

- `references.csl.json` is the canonical academic-reference source of truth.
- `references.verification.json` records evidence, provenance, confidence,
  conflicts, and review state.
- `review-queue.jsonl` is derived operational work.
- Provider evidence must not silently overwrite canonical CSL.
- Legal citations stay separate from academic CSL.
- Claim/source/provenance features do not assert claim truth.
- MCP write tools must stay dry-run first and require explicit apply.
- Default tests must be deterministic; live services are opt-in external tests.

## MoSCoW matrix

| Feature | MoSCoW | Current status | Repo contract | Evidence and tests |
| --- | --- | --- | --- | --- |
| Canonical CSL model | Must | Implemented | Academic references are stored as clean CSL JSON keyed by stable `id`. | CSL docs, schema and validation tests |
| Verification sidecar | Must | Implemented | Provider evidence and review state stay outside canonical CSL. | Sidecar docs and schema tests |
| Review queue | Must | Implemented | Review work is reproducible from CSL plus sidecar state. | Review queue fixtures and report tests |
| CSL validation | Must | Implemented | CLI and tests emit deterministic diagnostics. | `validate-csl --json`, schema inventory tests |
| Provider evidence normalization | Must | Implemented | Provider matches are recorded with provenance and confidence. | Provider fixtures and report tests |
| In-text citation reconciliation | Must | Implemented | Missing, uncited, duplicate, and ambiguous citations are surfaced without claiming source truth. | Reconciliation tests |
| Export suite | Must | Implemented | XML, ENW, RIS, BibLaTeX, and YAML exports remain deterministic. | Export tests and docs |
| CLI contract | Must | Implemented | Public commands keep JSON output stable where advertised. | CLI end-to-end tests |
| MCP read contract | Must | Implemented | Tools, resources, and prompts are discoverable and local-stdio oriented. | MCP distribution checks |
| Plugin registry | Must | Implemented | Runtime discovery validates manifests before exposing capabilities. | Plugin registry tests |
| Fixture benchmark | Must | Implemented | Benchmark claims are fixture-backed regression signals, not public SOTA claims. | `sourceright bench --json` |
| Publication metadata | Must | Implemented | Release, crate, OCI MCP, MCP Registry, and Glama metadata stay version-aligned. | Release and MCP distribution tests |
| Security governance | Must | Implemented | CI uses least privilege, dependency review, CodeQL, cargo audit, Scorecard, pinned actions, and quiet dependency automation. | Security workflow and Renovate config |
| OJS-compatible screening | Should | Contracted | OJS is the first journal target, implemented through platform-neutral screening outputs. | Journal integration docs and fixtures |
| Zotero-first citation sync | Should | Contracted | Sync defaults to preview, records audit logs, and only writes on explicit apply. | Citation-sync schema tests |
| Live core providers | Should | In progress | Crossref, DOI, DataCite, OpenAlex, PubMed, and ORCID remain opt-in and sidecar-only. | Live-provider configuration docs |
| DOCX/PDF extraction hardening | Should | In progress | Extraction preserves provenance spans and reports OCR limitations honestly. | Hardening fixtures |
| Citation disambiguation | Should | In progress | Institutional authors, same-author citations, and style variants route ambiguity to review. | Disambiguation fixtures |
| URL/archive integrity | Should | In progress | URL, DOI landing-page, redirect, and archive evidence is recorded as evidence, not truth. | URL/archive fixtures |
| Low-noise writeback | Should | In progress | Suggestions are thresholded, explained, dry-run first, and auditable. | Writeback tests |
| Legal citation mode | Could | Implemented as separate model | Legal citation records never force legal citations into academic CSL. | Legal roadmap and model tests |
| Claim/source provenance | Could | Implemented as bounded graph | Claims can be linked to sources, but Sourceright does not score claim truth. | Provenance tests |
| Additional journal platforms | Could | Planned | Adapters should call the Rust core or CLI/MCP, not reimplement verification logic. | Future platform fixtures |
| More citation managers | Could | Planned | EndNote and other managers should follow preview/apply/audit semantics. | Sync contract docs |
| HTTP MCP hosting | Could | Deferred | Local stdio remains the current server contract until a separate transport track exists. | MCP docs |
| AI client MCP packaging | Should | Contracted | Claude, Codex, GitHub Copilot, and generic MCP clients must be documented as client configuration or deferred package tracks, not implied plugins. | Host packaging docs and MCP transcript smoke |
| VS Code extension | Could | Local package smoke-tested | The VSIX scaffold reuses CLI/MCP diagnostics and does not reimplement citation logic. Current `.vscode` files remain development tooling only. | Host packaging docs and Track 77 VSIX smoke |
| Microsoft Word add-in | Could | Contracted | A future Office Add-in must prove range provenance and reversible dry-run editing before any in-document write claim. | Host packaging docs |
| LibreOffice extension | Could | Contracted | A future `.oxt`/UNO package must prove Writer range mapping and reversible dry-run editing before extension claims. | Host packaging docs |
| Marketplace submission evidence | Should | Fixture-backed refresh cadence | Host marketplaces and directories require accepted/prepared/deferred evidence before release wording can claim installability, and release-surface wording changes run the offline refresh verifier. | Release-status, host packaging docs, and `scripts/verify-release-surface-refresh.ps1` |
| Automatic final verification | Won't for now | Excluded | The product must not claim examiner-grade final verification until tracks 36-40 prove it. | Product guidelines |
| AI authorship detection | Won't for now | Excluded | Citation errors are not treated as proof of AI authorship. | Report wording tests |
| Silent provider overwrite | Won't for now | Excluded | Provider data never mutates canonical CSL without review or explicit apply. | Sidecar boundary tests |
| Default live external CI | Won't for now | Excluded | OJS, Zotero, registry, and provider live tests require opt-in credentials and sample data. | External-test plan |

## External test contract

External tests should be added as opt-in smoke suites:

- OJS fixture and live-test-instance smoke for submission screening.
- CLI installed-binary smoke for `init`, `validate-csl`, `report`, `export`,
  `bench`, `citation-sync`, and `mcp status`.
- MCP stdio transcript smoke for tool/resource/prompt discovery and read-only
  resources.
- Zotero or Better BibTeX preview/apply smoke using a disposable library.
- Registry smoke for crates.io, docs.rs, GHCR, official MCP Registry, Glama,
  and any future package-manager channels.
- Host-package smoke for future Claude/Codex client guides, VS Code VSIX,
  Office Add-in sideload, and LibreOffice `.oxt` packages when those tracks
  move beyond contracted status.
