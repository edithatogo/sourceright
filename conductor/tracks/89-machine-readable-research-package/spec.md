# Track 89 - Machine-Readable Research Package and Knowledge Graph

## Goal

Define the canonical package and graph model that lets a manuscript become a machine-readable research project with human-facing views.

## User outcome

Authors can submit one structured package that contains article text, references, data, code, protocols, ethics, reporting checklists, reviews, contribution credit, and dissemination outputs. Editors, agents, readers, indexers, funders, and repositories can query the same package at different levels of detail.

## Scope

- Define `sourceright.research_package.v1` as the root object.
- Support JSON, YAML, JATS XML, CSL JSON, BibLaTeX, RIS, ENW, RO-Crate-compatible metadata, and repository deposit metadata.
- Represent research questions, claims, evidence, methods, protocols, datasets, code, contributors, reviews, decisions, corrections, updates, and external dissemination as graph nodes.
- Model change over time as versioned state transitions.
- Provide API and export boundaries so independent indexers can query package metadata.
- Keep data access, ethics, consent, privacy, and embargo constraints attached to graph nodes.

## Out of scope

- Replacing all journal-specific submission formats in the first implementation.
- Publishing sensitive data or confidential reviews by default.
- Claiming graph truth without provenance and human review status.

## Contract boundary

Structured packages make submission auditable and reusable. They do not make claims correct. Every node must preserve provenance, status, confidence, and review state.

## Completion criteria

- Research package contract is documented.
- Knowledge graph relationship vocabulary is documented.
- Export targets are listed.
- Privacy and governance boundaries are explicit.
- Downstream tracks can depend on package and graph objects.
