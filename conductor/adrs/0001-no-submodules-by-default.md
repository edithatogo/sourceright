# ADR 0001: No Git Submodules By Default

## Status

Accepted.

## Decision

Sourceright will not use git submodules for immature plugins or provider
adapters by default.

## Rationale

The plugin API, evidence contracts, and host integrations are still moving.
Submodules would add version skew, checkout complexity, CI friction, and a
higher chance that public claims drift from the core repository.

## Consequences

Plugins stay in-repo until they have an independent release lifecycle, separate
maintainers, host-specific packaging requirements, or a stable compatibility
contract.
