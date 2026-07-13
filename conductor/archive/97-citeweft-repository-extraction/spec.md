# CiteWeft Repository Extraction Spec

## Goal

Make the neutral CiteWeft boundary independently extractable and auditable
without moving Sourceright CSL, providers, sidecars, CLI, MCP, review, or
exports without an approved repository/release migration.

## User outcome

Maintainers have a machine-readable split inventory, an independence verifier,
compatibility boundaries, and a reversible rehearsal plan before creating or
publishing a separate repository.

## Scope

- Inventory standalone-neutral modules versus Sourceright adapters.
- Verify forbidden imports and path ownership in the neutral set.
- Record schema, license, documentation, security, benchmark, package, and
  downstream compatibility gates.
- Preserve a no-live-publication/no-history-destructive-action boundary.

## Out of scope

- Creating a remote repository, pushing history, publishing crates, or changing
  SourceRight dependencies without explicit destination/approval.
- Rewriting neutral APIs or moving canonical CSL/provider/sidecar logic.
- Claiming standalone green CI, registry release, or downstream migration from
  a local inventory alone.

## Data contracts

The extraction manifest distinguishes `standalone_core` from
`sourceright_adapters`, names forbidden imports, records schema versions and
license policy, and gives explicit gates for history, CI/security, packaging,
release, downstream compatibility, rollback, and issue migration.

## Claim boundary

This track’s local slice proves an independence audit and reversible extraction
plan. It does not claim a separate repository or released package.

## Evidence level target

Machine-checked local independence inventory with a documented external gate
for the future history-preserving split and release sequence.

## Parallelization plan

- Lane A: neutral module inventory and import audit.
- Lane B: history/repository rehearsal after destination approval.
- Lane C: independent CI/security/package/release scaffolding.
- Lane D: downstream dependency migration only after an immutable candidate.
