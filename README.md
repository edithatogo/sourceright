# Sourceright

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
- Future legal citation verification with a separate legal citation model and public legal citation APIs.
- Later claim/source/provenance workflows once reference verification is reliable.

See [conductor/tracks.md](conductor/tracks.md) for the implementation track map.

## Current Status

The Rust core now includes canonical CSL handling, verification sidecars, intake segmentation, provider evidence normalization, cleaning, conflict resolution, citation reconciliation, manual review queues, reporting, exports, and journal screening contracts. Imported reference workflow material lives under `legacy/humanizer-next/` as provenance and regression material until audited, ported, or retired.

## Planned Distribution

- `sourceright` CLI binary.
- `sourceright mcp` server mode.
- GitHub Releases with platform binaries, checksums, and provenance artifacts.
- crates.io package once the public API is stable.
- Thin adapter packages only where native tool ecosystems require them.

## Development

```powershell
cargo check
cargo test
cargo fmt --check
cargo clippy --all-targets -- -D warnings
```

## License

Licensed under either of:

- Apache License, Version 2.0
- MIT license

at your option.
