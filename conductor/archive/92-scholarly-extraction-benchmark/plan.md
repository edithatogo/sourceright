# Scholarly Extraction Benchmark Plan

1. **[x] Discover.** Inspect the live benchmark harness, Track 91 GROBID
   fixture boundary, and the historical design packet.
2. **[x] Lock documents.** Define the manifest, stage metrics, hash/split policy,
   missing-data semantics, and claim boundary.
3. **[x] Implement.** Add the additive Rust benchmark module, CLI command, and
   self-authored fixture snapshots without touching canonical CSL behavior.
4. **[x] Validate.** Run unit, CLI, hash, deterministic-output, formatting,
   linting, locked-build, and existing benchmark checks.
5. **[x] Review locally.** Reinspect the diff for scope, security, licensing,
   failure accounting, and overclaim risks.
6. **[x] Apply fixes.** Record and fix local review findings before closeout.
7. **[x] Closeout.** Update the issue ledger and registry status; archive the
   completed track after review as explicitly requested.

## Completion signal

The checked-in suite runs offline and deterministically, verifies fixture
content hashes, reports stage-wise scores and unavailable coordinates, and
records the remaining independent-corpus limitation explicitly.
