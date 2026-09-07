# StandardFlow citation-evidence integration

Sourceright remains the authority for canonical CSL, provider-backed verification evidence, citation reconciliation and reference-integrity workflows. StandardFlow owns standards identity, applicability, evidence expectations and standards-derived assessment state.

The optional exchange emits a versioned `citation_verification` evidence envelope. It is local, read-only, network-off, telemetry-off and write-off by default. StandardFlow may associate the evidence with an applicable requirement; it may not silently rewrite CSL, resolve provider conflicts, declare a reference true, or approve publication.

The consumer is exact-pinned to proposed `edithatogo/standards_check` revision `820898e7ae21784145f98e04d8fc482367a6f015` from pull request 59. Automatic pin updates are prohibited. Absence or incompatibility of StandardFlow leaves normal Sourceright behaviour unchanged.

Validate the source contract and fixture with:

```bash
python scripts/check_standardflow_integration.py
```

This validator proves local structure and boundary policy only. Compilation, live provider behaviour, downstream compatibility, external validation and public acceptance require separate receipts.
