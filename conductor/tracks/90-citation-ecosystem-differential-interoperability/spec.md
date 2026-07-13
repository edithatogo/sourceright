# Specification

## Objective

Measure and improve Sourceright's bibliographic conversion and citation-manager
interoperability using mature external implementations as differential oracles,
without replacing the Rust core or treating third-party output as canonical
truth.

## Requirements

1. Maintain a curated, license-attributed fixture corpus covering BibTeX,
   BibLaTeX, RIS, CSL JSON, embedded DOCX/ODT citation metadata, and rendered
   bibliographies.
2. Compare normalized semantic records produced by Sourceright and selected
   external tools. Byte-for-byte formatting differences are not failures.
3. Classify each difference as equivalent, intentional policy, information
   loss, unsupported construct, parser discrepancy, or review required.
4. Exercise BibLaTeX-CSL-BibLaTeX and RIS-CSL-RIS round trips, protected casing,
   macros, names, dates, inheritance, multilingual fields, unknown fields, and
   malformed/adversarial input.
5. Produce deterministic JSON reports suitable for CI artifacts and agents.
6. Keep external execution optional and pinned. Core Rust tests must remain
   network-free and reproducible from checked-in fixtures.
7. Preserve original input and diagnostics. External output must never silently
   overwrite `references.csl.json` or verification state.

## Source roles

- Citation.js: broad conversion oracle and possible thin JavaScript adapter.
- biblatex-csl-converter: loss-aware mapping and embedded DOCX/ODT citation
  reference.
- retorquere/bibtex-parser: BibTeX parser oracle and adversarial fixture source.
- JabRef/JabKit: external import/export and round-trip endpoint.
- citeproc-js: CSL rendering and conformance oracle; CSL-M lessons do not merge
  legal records into academic CSL.
- Fidus Writer: workflow and document-integration reference only.
- Astrocite: archived AST-parser design reference only.

## Security and licensing

- Record source URL, upstream revision, license, provenance, and permitted reuse
  for every imported fixture.
- Do not copy code or fixture suites with unclear or incompatible licensing.
- Run external parsers in constrained CI jobs with untrusted-input limits,
  pinned lockfiles, no secrets, and no write access beyond temporary output.
- AGPL Fidus Writer remains a design reference unless separately reviewed.
- LGPL biblatex-csl-converter is isolated as an optional tool boundary unless a
  legal review approves another integration shape.
- Verify citeproc-js licensing before copying any fixture or source material.

## Acceptance criteria

- A machine-readable fixture manifest and schema are policy-tested.
- Differential reports are deterministic and preserve raw input provenance.
- Known lossy behavior in Sourceright's current exporters is explicitly
  measured and documented.
- At least one fixture-backed lane exists for each selected active oracle.
- JabRef/JabKit and rendering checks can be run as optional integration lanes.
- CI distinguishes mandatory Rust gates from optional external conformance.
- Documentation states that disagreement with an oracle is evidence for review,
  not proof that either implementation is correct.

## Out of scope

- Replacing Sourceright's Rust core with JavaScript or Java.
- Making CSL rendering a prerequisite for reference verification.
- Copying upstream implementation code into Rust.
- Treating external parser output as provider evidence or canonical truth.
- Folding CSL-M legal data into academic `references.csl.json`.
