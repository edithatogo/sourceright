# Track 89 - Implementation Plan

## Phase 1: Contract design [ ]

- [ ] Define the root research package schema.
- [ ] Define node types for manuscripts, references, data, code, protocols, ethics, checklists, reviews, contributions, and outputs.
- [ ] Define graph relationships and lifecycle statuses.

## Phase 2: Export compatibility [ ]

- [ ] Map manuscript views to JATS XML and Markdown or MyST.
- [ ] Map references to CSL JSON, BibLaTeX, RIS, and ENW.
- [ ] Map data and workflows to RO-Crate-compatible metadata.
- [ ] Map deposits to Crossref, DataCite, repository, and preprint metadata.

## Phase 3: Governance [ ]

- [ ] Attach access controls, embargoes, and consent metadata to graph nodes.
- [ ] Preserve provenance and version state for every node.
- [ ] Define public API views that do not leak confidential data.

## Phase 4: Fixture and validation [ ]

- [ ] Add a synthetic Transforming Health package fixture.
- [ ] Validate package export round trips where feasible.
- [ ] Confirm docs do not claim live platform support.
