# Live Provider Adapters Spec

## Goal

Implement opt-in live provider adapters for Unpaywall, OpenCitations, arXiv, Europe PMC, repository records, and licensed bring-your-own-key providers.

## Scope

- Add live adapter contracts for supported providers.
- Keep fixture-backed provider tests as the default validation path.
- Gate live smoke tests behind credentials and explicit opt-in flags.
- Write provider results only as sidecar evidence.
- Preserve conflict handling and review-state semantics.

## Outputs

- Live provider adapter interfaces and implementations.
- Provider-specific sidecar evidence records.
- Credential-gated live smoke tests.
- Fixture-backed deterministic tests.
- Provider documentation and configuration examples.

## Boundaries

Live provider evidence must never silently overwrite canonical CSL. Canonical changes require explicit conflict resolution or review workflows.

Licensed providers must require user-supplied keys and must not be exercised in default CI.
