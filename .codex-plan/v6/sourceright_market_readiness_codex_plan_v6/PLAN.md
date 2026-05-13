# Sourceright market-readiness plan v6

## Product readiness conclusion

Sourceright is ready to be positioned as:

> an open technical preview / pilot candidate for auditable reference-verification and citation-integrity workflows.

It should not yet be positioned as:

> a production-ready institutional product or externally benchmarked SOTA system.

## Why

Current public repo state indicates Sourceright already has meaningful product substance:
- Rust CLI/package at v0.1.20;
- canonical CSL handling;
- verification sidecars;
- cleaning and conflict handling;
- citation reconciliation;
- manual review queues;
- reports and exports;
- journal screening;
- legal citation records;
- provenance graphs;
- schemas;
- plugin registry/manifests;
- demos;
- benchmark scaffold.

The remaining work is mostly market proof, polish, and clarity:
- make the benchmark runner status consistent and documented;
- add benchmark result pages;
- add worked examples and case studies;
- improve Rust API documentation;
- link/polish demos;
- clarify plugin/provider implementation status;
- harden read-only MCP examples;
- publish a launch narrative that avoids overclaiming.

## Priority sequence

### Slice 0 — Audit and baseline
No source changes except `.codex-plan/` and `AGENTS.md`.

Goals:
- confirm repo state;
- run quality gates;
- identify existing failures;
- confirm benchmark current status.

### Slice 1 — Benchmark maturity
Turn the existing scaffold into a credible internal regression benchmark.

Goals:
- resolve `runner_status: scaffold_only` inconsistency;
- document whether `sourceright bench` executes all fixtures or only selected fixtures;
- add `benchmark-results.md` or equivalent;
- add acceptance thresholds for deterministic fixtures;
- separate “internal regression benchmark” from “external comparative benchmark.”

### Slice 2 — Documentation depth
Add practical guides and worked examples.

Goals:
- quickstart;
- author preflight workflow;
- editorial triage workflow;
- university repository workflow;
- legal citation mode workflow;
- artifact/schema guide;
- live provider configuration guide;
- benchmark interpretation guide.

### Slice 3 — Rust API docs
Improve docs.rs credibility.

Goals:
- add crate-level documentation;
- add docs to public structs/enums/functions;
- add examples for key workflows;
- ensure docs build cleanly.

### Slice 4 — Demos and landing page
Make the existing demos useful to visitors.

Goals:
- link GitHub Pages and Streamlit demo from README/docs;
- add screenshots or report-card examples;
- add “no live provider calls” disclaimers;
- add demo data refresh command;
- create a three-minute demo script.

### Slice 5 — Plugin/provider status clarity
Make the registry honest and market-safe.

Goals:
- add status matrix;
- distinguish core normalizers, fixture-backed, planned public API, planned BYO-key, planned adapter, and not implemented;
- document licensing/caching caveats for paid providers.

### Slice 6 — Case studies
Add fake-but-realistic case-study fixtures.

Goals:
- biomedical manuscript with retraction/version warning;
- preprint with version-of-record issue;
- legal filing with ambiguous or pinpoint citation issue;
- each with input, CSL, sidecar, report, review queue, and export manifest.

### Slice 7 — MCP read-only hardening
Add examples and safety docs.

Goals:
- exact read-only tool/resource list;
- sample config for Codex/Claude/Cursor where applicable;
- transcript examples;
- threat model;
- explicit “no write tools yet” language.

### Slice 8 — Launch package
Prepare technical-preview launch materials.

Goals:
- README positioning;
- docs homepage;
- release notes;
- FAQ;
- limitations page;
- pilot invitation language;
- issue templates for pilots.

## Non-goals for this cycle

- Full runtime plugin loading.
- Workspace split.
- Live-network benchmark requirements.
- ML/NLP extraction dependencies.
- Paid provider integrations beyond BYO-key manifests and documentation.
- Legal compliance claims.
- SOTA performance claims.
