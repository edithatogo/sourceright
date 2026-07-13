# Native Reference and Citation Model Plan

1. **[x] Discover.** Inspect the live neutral boundary and historical model
   design without importing the old scaffold.
2. **[x] Lock documents.** Define labels, spans, confidence states, and the
   no-CSL/no-truth claim boundary.
3. **[x] Implement.** Add the deterministic reference/callout baseline,
   versioned manifest, fixtures, and policy tests.
4. **[x] Validate.** Run formatting, fixture, serialization, policy, and
   workflow checks; attempt Rust checks and record toolchain limits.
5. **[x] Review locally.** Check source grounding, ambiguity, canonical CSL
   isolation, and malformed-input behavior.
6. **[x] Apply fixes.** Resolve local findings before closeout.
7. **[x] Closeout.** Record learned-runtime and independent-cohort deferrals;
   leave archive cleanup to review.

## Review fixes

- [x] Add a bounded input-size guard before segmentation.
- [x] Rename extraction disposition from `accepted` to `extracted` to avoid
  implying bibliographic verification.

## Completion signal

Deterministic reference segmentation, conservative fields, numeric callout
linking, source spans, confidence/review status, and a machine-readable model
manifest run on checked-in fixtures without network or CSL mutation.

## Validation note

`cargo test` was attempted with the locked dependency graph but remains blocked
by the workstation linker: host build scripts resolve Git's Unix `link.exe`
instead of a usable MSVC linker. Formatting, diff, fixture, and workflow checks
are the available local evidence.
