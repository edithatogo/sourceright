# Plugin And Provider Roadmap Delivery Spec

## Goal

Ensure every plugin/provider listed in `plugins/registry.toml` has an owned
implementation path, fixture path, or explicit deferral so planned features do
not disappear from the roadmap or look completed prematurely.

## Plugin Families

- Core normalizers: Crossref and PubMed/repository records.
- Public API providers: Unpaywall, OpenCitations, arXiv, Europe PMC,
  repository records, CourtListener.
- Licensed BYO-key providers: Dimensions, Scopus, Web of Science.
- Adapters: Zotero, EndNote, OJS, DOCX/PDF extraction, GitHub Pages demo.
- Local logic: local bibliographic matcher, retraction/recency checks,
  claim-source relevance.
- Exporters: citation file exports.

## Contracts

- Planned plugins stay visibly planned until fixture-backed behavior exists.
- BYO-key providers must not run in default CI.
- Adapters must shell out to or call the Rust core rather than reimplementing
  verification logic.
- Every status change must update docs, tests, and Conductor evidence.

## Parallelization

Subagents can own provider families independently:

- Public API providers.
- Licensed providers.
- Citation-manager adapters.
- Journal/document/demo adapters.
- Legal/provenance/local matcher providers.
