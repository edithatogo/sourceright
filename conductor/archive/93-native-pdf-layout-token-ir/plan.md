# Native PDF Layout and Token IR Plan

1. **[x] Discover.** Inspect the live CiteWeft boundary and historical candidate
   design without importing the old scaffold.
2. **[x] Lock documents.** Define the stable IR, coordinate semantics,
   diagnostics, limits, and claim boundary.
3. **[x] Implement contract slice.** Add the neutral layout types, bounded
   fixture adapter, deterministic reading order, and diagnostics.
4. **[x] Validate.** Run formatting, policy, fixture, serialization, and
   existing workflow checks; attempt Rust checks and record toolchain limits.
5. **[x] Review locally.** Check for CSL coupling, unsafe resource behavior,
   coordinate loss, and overclaim wording.
6. **[x] Apply fixes.** Resolve local findings before closeout.
7. **[x] Closeout.** Record the backend-selection deferral explicitly and mark
   the contract slice complete; leave archive cleanup to a separate review.

## Review fixes

- [x] Reject PDF-looking bytes in the text fixture adapter.
- [x] Include page identity in token source IDs.
- [x] Add serialization round-trip and overlapping-layout ambiguity tests.

## Completion signal

The repository has a stable, bounded, deterministic layout IR with fixture
evidence and explicit diagnostics. Native PDF backend selection remains gated
by independent benchmark/license/security evidence and is not claimed here.

## Validation note

`cargo test` was attempted with the locked dependency graph but remains blocked
by the workstation linker: host build scripts resolve Git's Unix `link.exe`
instead of a usable MSVC linker. Formatting, diff, fixture, and workflow checks
are the available local evidence.
