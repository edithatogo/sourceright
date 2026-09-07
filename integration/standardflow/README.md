# StandardFlow citation-evidence integration

Sourceright remains the authority for canonical CSL, provider-backed verification evidence, citation reconciliation and reference-integrity workflows. StandardFlow owns standards identity, applicability, evidence expectations and standards-derived assessment state.

The optional exchange emits a versioned `citation_verification` evidence envelope. It is local, read-only, network-off, telemetry-off and write-off by default. StandardFlow may associate the evidence with an applicable requirement; it may not silently rewrite CSL, resolve provider conflicts, declare a reference true, or approve publication.

`producer_baseline_revision` identifies the existing Sourceright logic whose semantics are represented. `artifact_provenance` separately records where the contract or envelope can be retrieved. During review this is a deliberately mutable pull-request path; a post-merge verification receipt must replace it with the immutable artifact-bearing commit before any released interoperability claim.

The consumer is exact-pinned to verified `edithatogo/standards_check` revision `f248d7c4dd40dd34cb45b5b6150b4750a5b28864` from pull request 59. Automatic pin updates are prohibited. Absence or incompatibility of StandardFlow leaves normal Sourceright behaviour unchanged.

Validate the source contract and fixture with:

```bash
python scripts/check_standardflow_integration.py
```

This validator proves local structure and boundary policy only. Compilation, live provider behaviour, downstream compatibility, external validation and public acceptance require separate receipts.
