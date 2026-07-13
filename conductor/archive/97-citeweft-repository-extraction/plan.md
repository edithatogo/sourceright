# CiteWeft Repository Extraction Plan

1. **[x] Discover.** Inspect the current neutral modules, adapters, manifests,
   package boundaries, and historical split design.
2. **[x] Lock documents.** Define standalone core, downstream adapters,
   forbidden imports, gates, and rollback boundary.
3. **[x] Implement local slice.** Add the extraction manifest, verifier, policy
   test, docs, and rehearsal status.
4. **[x] Validate.** Run formatting, verifier, JSON, policy, and workflow checks;
   attempt Rust checks and record toolchain limits.
5. **[x] Review locally.** Check no destructive history operations, no external
   publication claims, and no neutral-to-CSL coupling.
6. **[x] Apply fixes.** Resolve local findings before closeout.
7. **[ ] External progression.** Create the approved repository, perform the
   history-preserving split, publish releases, and migrate downstream only after
   explicit destination and release approvals.

## Review fixes

- [x] Verify core/adapter inventories are disjoint.
- [x] Verify schema contracts and all external gates are present and open.
- [x] Expand forbidden-import checks across downstream Sourceright surfaces.

## Completion signal

The local extraction inventory and verifier pass. The external repository/release
gate remains open and is not represented as completed by this track.
