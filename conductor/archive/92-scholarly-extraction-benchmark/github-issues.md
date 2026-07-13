# GitHub Issue Ledger

## Issue #37

Track 92: scholarly extraction benchmark. The implementation adds the offline
manifest, hash-verified self-authored fixture, stage-wise metrics, backend
provenance, unavailable-coordinate semantics, leakage validation, and the
`extraction-bench` CLI. Independent license-audited corpora, pinned live GROBID
container runs, resource metrics, and cohort thresholds remain explicitly
opt-in follow-on work.

Completion evidence: https://github.com/edithatogo/sourceright/issues/37#issuecomment-4946809007

Review fix: added explicit operation status, latency, and optional peak-memory
metadata to the snapshot/report contract. Resource values are not inferred when
unavailable.
