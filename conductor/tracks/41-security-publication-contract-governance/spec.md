# Security, Publication, and Contract Governance Spec

## Goal

Close the current dependency and supply-chain alerts, keep dependency automation
quiet, and make the product contract explicit enough to drive future market
readiness work.

## Scope

- Resolve open dependency alerts without changing product behavior.
- Keep Renovate as the low-noise dependency update lane.
- Pin GitHub Actions by commit SHA where practical.
- Inventory unmerged branches and publication surfaces.
- Add a canonical feature matrix that doubles as the requirements document and
  repo contract.
- Add a design document with diagrams for core data, CLI, MCP, plugin,
  journal, citation-manager, and release flows.

## Boundaries

- Do not add noisy Dependabot version-update PR routing unless Renovate is no
  longer enough.
- Do not claim final verifier, legal filing, or AI detector status.
- Do not auto-merge major dependency updates.
- Do not add live external platform tests to default CI without credentials,
  sample-data boundaries, and skip-by-default behavior.

## Contracts

- The feature matrix is the canonical public requirements surface.
- The design document explains the architecture behind that contract.
- Publication claims must distinguish accepted listings from prepared metadata.
- Dependency automation must prefer grouped, scheduled, safe updates.
