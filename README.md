# Sourceright

[![Release](https://github.com/edithatogo/sourceright/actions/workflows/release.yml/badge.svg)](https://github.com/edithatogo/sourceright/actions/workflows/release.yml)
[![Publish crate](https://github.com/edithatogo/sourceright/actions/workflows/publish-crate.yml/badge.svg)](https://github.com/edithatogo/sourceright/actions/workflows/publish-crate.yml)
[![Coverage](https://github.com/edithatogo/sourceright/actions/workflows/coverage.yml/badge.svg)](https://github.com/edithatogo/sourceright/actions/workflows/coverage.yml)

Sourceright is Rust-first reference triage and verification infrastructure for academic writing, agent workflows, and future legal citation work. It is currently a technical preview suitable for developer evaluation and pilot conversations.

## Who This Is For

- Technical editors and research-integrity teams that need auditable reference cleanup.
- Library, repository, and publisher teams that want deterministic citation verification workflows.
- Developers and agent workflows that need CSL-centered bibliographic pipelines with sidecar evidence.
- Legal-tech researchers evaluating a separate legal citation path without conflating it with academic CSL.

## What It Does / Does Not Do

Sourceright takes references from documents or text, produces canonical CSL JSON, standardises and cleans the records, collects provider evidence, routes uncertain records to manual review, and exports clean reference files for tools such as EndNote, Zotero, and BibLaTeX.

It does not claim to be a production-ready institutional product, a court filing compliance system, or a substitute for examiner-grade final verification. Robust DOCX/PDF extraction, live core-provider checks, better citation disambiguation, URL/archive integrity, and low-noise writeback suggestions remain hardening tracks.

## Workflow

```text
document/text
  -> extracted references and in-text citations
  -> references.csl.json
  -> standardisation, cleaning, verification, enrichment
  -> references.verification.json and review-queue.jsonl
  -> conflict resolution, citation reconciliation, and reference integrity reports
  -> XML, ENW, RIS, BibLaTeX, and YAML exports
```

## Roadmap

- Academic reference extraction and CSL JSON canonicalisation.
- Provider-backed verification through Crossref first, then DOI resolution, DataCite, OpenAlex, PubMed/NCBI, and ORCID where useful.
- In-text citation reconciliation against reference-list entries.
- Manual review queues designed for human and agent/subagent verification.
- Platform-neutral journal screening outputs for editorial workflows, with OJS as the first public integration target.
- Reference reports for citation integrity auditing and AI-related citation-error signals.
- CLI and MCP server interfaces over the same Rust core.
- Legal citation analysis with a separate legal citation model and public-provider slots.
- Claim/source/provenance graphs built over detected citation support without asserting claim truth.
- Examiner-grade audit hardening for real DOCX/PDF extraction, live provider evidence, institutional-author matching, URL/archive checks, and explicit writeback plans.

See [conductor/tracks.md](conductor/tracks.md) for the implementation track map.
The canonical requirements and repo contract are in
[docs/src/feature-contract-matrix.md](docs/src/feature-contract-matrix.md), with
the design diagrams in [docs/src/design.md](docs/src/design.md).

## Current Status

The Rust core now includes canonical CSL handling, verification sidecars, intake segmentation, provider evidence normalization, cleaning, conflict resolution, citation reconciliation, manual review queues, reporting, exports, journal screening contracts, legal citation records, and claim/source provenance graphs. It should be treated as a structured triage and audit workflow and technical preview until tracks 36-40 close the examiner-grade verification gaps. Imported reference workflow material lives under `legacy/humanizer-next/` as provenance and regression material until audited, ported, or retired.

## CLI

```text
sourceright init [document-or-directory]
sourceright validate-csl [--json] <references.csl.json>
sourceright report [--json|--mcp-resource] [.sourceright-directory]
sourceright export --preview --all [.sourceright-directory]
sourceright bench [--json]
sourceright citation-sync [--preview|--apply] [.sourceright-directory]
sourceright mcp
```

`bench` runs the checked-in fixture suite without live providers. The benchmark
surface is a technical preview for deterministic regression and stress checks.
`citation-sync` defaults to preview mode and requires explicit `--apply` before
writing audit logs or remote fixture snapshots.

## Planned Distribution

- `sourceright` CLI binary.
- `sourceright mcp` server mode.
- GitHub Releases with platform binaries, checksums, and provenance artifacts.
- crates.io package after release dry runs pass.
- OCI MCP image metadata for the official MCP Registry via `server.json` +
  `Dockerfile` labels.
- Smithery readiness (Streamable HTTP first; MCPB/local path until then).
- Glama ownership metadata via `glama.json`.
- `release-status.md` artifacts from the release and tag-triggered publish workflows.
- `coverage-status.md` artifacts from the scheduled coverage workflow.
- Thin adapter packages only where native tool ecosystems require them.
- Track 30 owns the Starlight/Astro docs-site migration and Pages deployment.
- Track 31 is reserved for coverage, mutation, property, load, edge, integration, and end-to-end test hardening.
- Track 32 is reserved for publishing governance and provenance automation.
- Track 33 covers live publication to crates.io, GitHub Releases, and registries.
- Track 34 covers coverage measurement and reporting until the 85 percent floor is reproducible.
- Track 35 covers the final public docs cutover and launch.
- Tracks 36-40 cover examiner-grade audit hardening: document extraction, live core providers, citation disambiguation, URL/archive integrity, and low-noise writeback suggestions.
- See `docs/src/release-runbook.md`, `docs/src/coverage-reporting.md`, and `docs/src/docs-cutover.md` for the operational sequence behind those tracks.
- `v*.*.*` tags now auto-start the crate publish workflow, and the MCP registry workflow follows the release workflow completion.

### Distribution metadata files

- `server.json` (`io.github.edithatogo/sourceright`)
- `Dockerfile` labels:
  - `io.modelcontextprotocol.server.name`
  - `org.opencontainers.image.source`
- `glama.json`

## Development

```powershell
cargo check
cargo test
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo run --bin sourceright -- bench
cargo package --locked
cargo publish --dry-run --locked
cargo deny check advisories bans sources
cargo tree -d --locked
typos --config typos.toml
cargo llvm-cov --locked --all-targets --summary-only --fail-under-lines 85
cargo mutants --workspace
pwsh -NoProfile -ExecutionPolicy Bypass -File scripts/verify.ps1 -CoverageMinimum 85
```

Coverage stays gated above 85 percent in CI and in the checked-in pre-commit
hook.

Release and publish workflows each emit a `release-status.md` artifact so the
latest tag has a named checklist surface.
Coverage runs emit a `coverage-status.md` artifact so the latest numeric report
has a named checklist surface.

## License

Licensed under either of:

- Apache License, Version 2.0
- MIT license

at your option.
