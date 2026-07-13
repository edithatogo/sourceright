# Research Assessment — 2026-07-11

| Project | Observed state | Sourceright role | Adoption decision |
| --- | --- | --- | --- |
| Citation.js | Active JavaScript project; MIT; modular BibTeX, RIS, DOI, BibJSON, Wikidata and CSL support | Broad conversion oracle and optional adapter | Evaluate through a pinned runner; do not move core logic to JavaScript. |
| biblatex-csl-converter | Active TypeScript project; LGPL-3.0; bidirectional CSL/BibLaTeX plus document citation extraction | Highest-value mapping/loss reference | Isolate as optional tooling and fixtures pending license review. |
| retorquere/bibtex-parser | Active MIT parser used by Better BibTeX | Parser oracle and adversarial behavior reference | Use for differential cases and diagnostics, not canonical truth. |
| JabRef/JabKit | Active MIT Java citation manager and CLI | Real import/export interoperability endpoint | Add optional disposable round-trip smoke. |
| citeproc-js | Active mature CSL/CSL-M renderer with extensive integration tests; GitHub SPDX unresolved | Rendering/conformance oracle | Verify license before fixture reuse; keep legal model separate. |
| Fidus Writer | Active AGPL-3.0 collaborative academic editor | Document workflow and integration reference | Learn from architecture; no dependency or copied code. |
| Astrocite | Archived MIT TypeScript AST-to-CSL project | Parser architecture reference | Record lessons only; do not adopt. |

The highest-value gap is Sourceright's currently narrow, hand-written
BibLaTeX export mapping. Differential tests should quantify loss before any
implementation expansion. Benchmark claims remain separate from product claims.
