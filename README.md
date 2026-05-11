# Sourceright

[![Release](https://github.com/edithatogo/sourceright/actions/workflows/release.yml/badge.svg)](https://github.com/edithatogo/sourceright/actions/workflows/release.yml)
[![Publish crate](https://github.com/edithatogo/sourceright/actions/workflows/publish-crate.yml/badge.svg)](https://github.com/edithatogo/sourceright/actions/workflows/publish-crate.yml)
[![Coverage](https://github.com/edithatogo/sourceright/actions/workflows/coverage.yml/badge.svg)](https://github.com/edithatogo/sourceright/actions/workflows/coverage.yml)

Sourceright is Rust-first reference verification infrastructure for academic writing, agent workflows, and future legal citation work.

The first product goal is to take references from documents or text, produce canonical CSL JSON, standardise and clean the records, verify and enrich them through citation APIs, route uncertain records to manual agent review, then export clean reference files for tools such as EndNote, Zotero, and BibLaTeX.

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

See [conductor/tracks.md](conductor/tracks.md) for the implementation track map.

## Current Status

The Rust core now includes canonical CSL handling, verification sidecars, intake segmentation, provider evidence normalization, cleaning, conflict resolution, citation reconciliation, manual review queues, reporting, exports, journal screening contracts, legal citation records, and claim/source provenance graphs. Imported reference workflow material lives under `legacy/humanizer-next/` as provenance and regression material until audited, ported, or retired.

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

`bench` runs the checked-in fixture suite without live providers. `citation-sync`
defaults to preview mode and requires explicit `--apply` before writing audit
logs or remote fixture snapshots.

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
- Track 34 covers coverage measurement and reporting until the 90 percent floor is reproducible.
- Track 35 covers the final public docs cutover and launch.
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
cargo llvm-cov --locked --all-targets --summary-only --fail-under-lines 90
cargo mutants --workspace
pwsh -NoProfile -ExecutionPolicy Bypass -File scripts/verify.ps1 -CoverageMinimum 90
```

Coverage stays gated above 90 percent in CI and in the checked-in pre-commit
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
