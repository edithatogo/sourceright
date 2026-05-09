# Verification Sidecar Plan

1. Define sidecar schema keyed to CSL reference IDs. Completed with `VerificationSidecar` keyed by CSL id.
2. Add provider candidate and confidence structures. Completed with bounded provider candidates and source-preserving payloads.
3. Add conflict and manual-review states. Completed with conflict invariant checks, review states, transitions, and retained decisions.
4. Add deterministic serialization. Completed with sorted reference keys, stable JSONL review queue derivation, and canonical sidecar formatting.
5. Add compatibility versioning. Completed with `sourceright.verification.v1` and schema-version validation diagnostics.
