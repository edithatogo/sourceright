# External Proof Suites Spec

## Goal

Show Sourceright works in realistic integration contexts without making live
services mandatory in default CI.

## Required Proof Families

- Installed CLI smoke: installed binary can run `init`, `validate-csl`,
  `report`, `export`, `bench`, `citation-sync`, and `mcp status`.
- MCP stdio transcript smoke: real client-style transcript proves tool,
  resource, and prompt discovery.
- OJS proof: fixture-backed adapter contract and optional disposable OJS/test
  instance smoke.
- Citation-manager proof: Zotero-first preview/apply/audit fixture and optional
  disposable-library smoke; EndNote export handoff proof.
- Live provider proof: opt-in provider smoke with cache/rate-limit controls.
- Registry proof: public listing/install checks for accepted registries.

## Out Of Scope

- Default CI calls to live services.
- Tests that require private credentials without skip/diagnostic behavior.
- Claiming platform support beyond the proof family that passed.

## Parallelization

Each proof family can be implemented by a separate worker with disjoint test and
docs files. Shared schema/contracts must be edited serially by the lead.
