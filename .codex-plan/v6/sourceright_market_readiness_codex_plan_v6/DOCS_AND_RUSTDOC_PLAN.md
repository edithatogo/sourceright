# Documentation and Rust API docs plan

## Public docs improvements

Add or expand:

```text
docs/src/quickstart.md
docs/src/worked-example-author-preflight.md
docs/src/worked-example-editorial-triage.md
docs/src/worked-example-university-repository.md
docs/src/worked-example-legal-mode.md
docs/src/schema-contracts.md
docs/src/artifacts.md
docs/src/live-provider-configuration.md
docs/src/benchmark-results.md
docs/src/plugin-provider-status.md
docs/src/limitations.md
docs/src/faq.md
```

Each worked example should include:
- sample input;
- command sequence;
- expected report snippet;
- how to interpret warnings;
- what the tool does not conclude.

## Rust API docs

Prioritize:
- crate-level `//!` docs;
- public model docs;
- public workflow function docs;
- CLI-facing function docs if exposed;
- examples that compile or at least explain artifact flow.

Suggested public items to document:
- CSL document types;
- verification sidecar types;
- report types;
- provider candidate/evidence types;
- review queue types;
- policy types;
- benchmark runner;
- plugin registry/discovery;
- citation sync preview/apply;
- MCP status/resource types;
- legal citation types;
- provenance graph types.

## Minimum acceptable target

For a preview launch:
- crate-level docs exist;
- core exported types have doc comments;
- docs.rs no longer looks empty;
- README links to docs site and examples.
