# Implementation Plan

## Phase 1 — Fixture and provenance contract

- [x] Task: Define the interoperability fixture manifest and JSON Schema.
    - [x] Require source URL, pinned revision, license, provenance, input format, expected semantic assertions, and reuse status.
    - [x] Add original-input and expected-CSL pairs without importing incompatible upstream suites.
- [x] Task: Add policy tests for manifest completeness and license allowlisting.
- [ ] Task: Conductor - User Manual Verification 'Fixture and provenance contract' (Protocol in workflow.md).

## Phase 2 — Differential conversion harness

- [x] Task: Define a normalized semantic comparison model and difference taxonomy.
- [x] Task: Add a pinned Citation.js runner and a pinned Better BibTeX parser
  runner with lockfile, license, and protected-casing fixture smoke.
- [ ] Task: Add a pinned optional runner for biblatex-csl-converter.
    Remains isolated until LGPL packaging and legal review are complete.
- [x] Task: Emit deterministic JSON and human-readable difference reports.
- [x] Task: Add a secret-free, core-only comparison CLI lane with deterministic
  output and an optional Markdown artifact path. External ecosystem runners
  remain separate and optional.
- [ ] Task: Exercise BibLaTeX-CSL-BibLaTeX and RIS-CSL-RIS round trips.
- [ ] Task: Conductor - User Manual Verification 'Differential conversion harness' (Protocol in workflow.md).

## Phase 3 — BibTeX parser robustness

- [x] Task: Start adversarial fixtures for macros, protected casing, TeX
  markup, inheritance, malformed records, and resource limits.
    - [x] Add a self-authored macro/protected-casing/month/multiple-author
      fixture with expected CSL and manifest assertions.
- [ ] Task: Compare Sourceright behavior with retorquere/bibtex-parser without treating the oracle as truth.
- [ ] Task: Record information loss and parser discrepancies as explicit diagnostics.
- [ ] Task: Conductor - User Manual Verification 'BibTeX parser robustness' (Protocol in workflow.md).

## Phase 4 — Manager and rendering interoperability

- [ ] Task: Add optional JabRef/JabKit import-export smoke and round-trip fixtures.
- [ ] Task: Add citeproc-js rendering fixtures after license verification.
- [ ] Task: Document Fidus Writer document-workflow lessons and the explicit Astrocite no-adoption decision.
- [ ] Task: Conductor - User Manual Verification 'Manager and rendering interoperability' (Protocol in workflow.md).

## Phase 5 — CI, documentation, and review

- [x] Task: Add a secret-free, pinned, constrained external-conformance CI lane with failure artifacts.
    - [x] Run the basic, adversarial BibTeX, and RIS fixture matrix with
      lockfile-pinned Node runners and upload JSON/Markdown artifacts.
- [x] Task: Keep mandatory Rust checks independent from optional ecosystem tooling availability.
- [x] Task: Publish the support matrix, known losses, reproduction commands, and claim boundaries in `interop-runners/README.md`.
- [ ] Task: Run format, clippy, tests, locked check, harness validation, and Conductor review.
- [ ] Task: Conductor - User Manual Verification 'CI, documentation, and review' (Protocol in workflow.md).
