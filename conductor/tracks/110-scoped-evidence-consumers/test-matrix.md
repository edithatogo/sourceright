# Scoped Evidence Consumers Test Matrix

All scenarios are future implementation acceptance checks. Their presence is not
execution evidence. Fixtures must be synthetic and stay independent of real case
payloads or sealed study labels.

| Scenario | Acceptance | Evidence | Default-CI or opt-in-live |
| --- | --- | --- | --- |
| Valid citation, unsupported claim | Linkage may pass; semantic support remains `not_assessed` or the separately attributed observation. | Explicit result fields; no inferred support from a citation link. | Default-CI, pending |
| Valid source, prohibited span | Reject the span even if another span in that source is allowed; do not expand the view. | Boundary fixture and local-access audit. | Default-CI, pending |
| Source/view/policy revision changed | Reject or invalidate stale observations and cache entries. | Paired revisions and cache-isolation regression. | Default-CI, pending |
| Unicode or normalized text offsets | Resolve exact original bytes under the declared convention, or return unavailable/mismatch. | Round-trip source-span fixtures and offset map. | Default-CI, pending |
| Missing local source or unsupported capability | Record `not_run`/unavailable; neither success nor an empty support finding. | Failure fixture and absence of remote fallback. | Default-CI, pending |
| Imported semantic observation | Preserve issuer, revisions, claim/view digests, verdict and execution state; reject cross-view or malformed results. | Consumer parity fixture; still no truth claim. | Default-CI, pending |
| Unknown temporal entitlement | Carry the consumer's uncertainty without inventing a cutoff or recipient awareness. | Opaque context round trip and no inferred chronology. | Default-CI, pending |
| Local-only job with hostile source instructions | No network call, implicit provider lookup or data-bearing error/telemetry output. | Denied-egress harness and synthetic canary checks. | Default-CI, pending |
| Final report changes a claim | Recheck the changed claim; old verification cannot be carried forward silently. | Pre/post-render digest and invalidation fixture. | Default-CI, pending |
| Sidecar/report generation | Preserve conflicting observations; canonical CSL and original sources stay unchanged. | Before/after hashes and existing regression tests. | Default-CI, pending |

## Planning inspection

The planning baseline is `3a603ec8344e9865c68998d48eca573499d3fb5d`.
Inspection used root instructions, Conductor index/workflow/registry, Track 14
and `src/provenance.rs`. Track 110 was unused at that revision. No active open PR
covered this consumer scope; the StandardFlow and registry PRs remain separate.
This inspection supplies planning context, not execution of the scenarios above.
