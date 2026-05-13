# Limitations

This page is the blunt version of the product boundary. Sourceright is useful
today as a technical preview, but it is still constrained by fixture-backed
execution, explicit trust gates, and a narrow set of implemented commands.

## Current limits

- Live provider use is opt-in and should not be assumed in CI or normal local
  runs.
- Provider evidence must not silently overwrite canonical CSL.
- Scanned PDF OCR is not treated as a completed extraction path yet; binary
  sources should surface diagnostics rather than pretend extraction succeeded.
- Legal citations stay separate from academic CSL.
- Claim/source/provenance work does not assert claim truth.
- Benchmark results are deterministic and fixture-backed; they are not a live
  provider quality claim.
- Registry entries marked `planned`, `planned_adapter`, `planned_public_api`,
  or `planned_byo_key` are roadmap surfaces, not current live promises.

## What technical preview means here

- The contract exists.
- Fixtures exist for the documented paths.
- Validation and reporting are in place for the supported shapes.
- The system still expects a human to review edge cases, conflicts, and
  unresolved queue items.

## When to stop

Stop and inspect the source record when you see:

- a provider conflict;
- a low-confidence or unresolved review item;
- a schema mismatch;
- a scanned or otherwise unsupported extraction source;
- a report that asks for human review instead of a clean pass.

## Do not infer

Do not infer production readiness, full extraction coverage, or complete live
provider availability from the current docs, manifests, or benchmark fixtures.
