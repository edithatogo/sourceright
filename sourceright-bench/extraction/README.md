# Scholarly extraction benchmark

This is a deterministic technical-preview harness for stage-wise extraction
evidence. The checked-in fixture is self-authored and hash-verified. It is not
a claim about general PDF, GROBID, CiteWeft, or model accuracy.

The manifest keeps source, license, access, split, cohort, and immutable input
hash metadata together. Gold and prediction snapshots expose reference
segmentation, field extraction, callout linking, backend provenance, and
coordinate availability. They also record execution status, latency, and
optional peak memory; absent resource measurements remain absent. Unavailable
coordinates remain unavailable rather than being scored as zero.

Run it with:

```text
cargo run --bin sourceright -- extraction-bench --json
```

Restricted or independently held-out corpora must remain outside this
repository. Add only a license-audited manifest and retrieval instructions for
those opt-in suites.
