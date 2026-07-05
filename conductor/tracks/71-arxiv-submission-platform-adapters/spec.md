# Track 71: arXiv submission platform adapters

Define the thin adapter contract shared by arXiv submission surfaces.

## Scope

- Keep platform-specific logic behind a small adapter boundary.
- Separate current-platform and legacy-platform concerns.
- Preserve submission requirements and approval boundaries from track 72.
- Keep the contract fixture-backed until a later live-proof lane exists.

## Acceptance

- The adapter lane is documented as additive and dependency-gated.
- The shared contract can support both current and legacy arXiv paths.
- Downstream tracks can depend on a stable short alias.