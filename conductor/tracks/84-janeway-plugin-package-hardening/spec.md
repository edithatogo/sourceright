# Track 84 - Janeway Plugin Package Hardening

## Goal

Turn the Janeway reconnaissance contract into a packageable adapter skeleton with explicit boundaries, test hooks, and a clear path to CLI-backed screening.

## User outcome

Maintainers can point to a packageable Janeway integration contract, a stable boundary for command invocation, and fixture-backed checks that prove the adapter shape before any live install smoke.

## Scope

- Define the Janeway adapter package shape and metadata requirements.
- Specify how Sourceright should be invoked from Janeway without moving screening logic into the host platform.
- Define permissions, report handoff, and preview-first behavior.
- Add fixture-backed checks for the package contract and adapter boundary.

## Out of scope

- Live Janeway deployment or acceptance claims.
- Upstream Janeway PRs.
- Full plugin implementation if the Janeway extension model requires a sidecar first.
- Automatic install or registration outside an explicit test harness.

## Data contracts

- Janeway adapter package metadata.
- A Janeway install archive or skeleton package.
- CLI invocation contract for `sourceright journal-screen`.
- Fixture-backed report handoff evidence.

## Claim boundary

> "Packageable" is not "deployed", and "preview-first" is not "production-ready".

The track may claim that a Janeway adapter skeleton is defined and testable, but it must not claim installation or live use until the smoke track passes.

## Evidence level target

**fixture-backed** - the package shape, adapter boundary, and tests should be deterministically checkable without any live Janeway system.

## Parallelization plan

- Package metadata and the adapter boundary can be designed in parallel.
- The fixture-backed test matrix depends on the package shape being stable.
- Docs can reference the package boundary once the contract is locked.
