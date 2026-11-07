# Scoped Evidence Consumers Plan

Status: **planned**, 8 November 2026. All implementation work below is pending.
Existing Track 14 completion records continue to describe its delivered linkage
baseline only. Follow the [workflow](../../workflow.md),
[requirements](../../requirements.md) and [test matrix](test-matrix.md).

## Discover and lock

- [ ] Map the consumer's exact view/claim/source contracts to existing sidecars
  and provenance spans; name the owned runtime paths and additive schema version.
  Use rcagent CAW08 as the consumer acceptance owner. Confirm whether optional
  support observations are needed before adding any provider-facing surface.
- [ ] Lock synthetic positive and negative fixtures, byte/character offset
  conventions, cache isolation and failure semantics with the consumer. Preserve
  Unicode and normalization mappings; absence of text or capability is explicit.

## Implement and check

- [ ] Add the smallest local identity/span/binding adapter, reusing existing
  hashes, serializers and validation mechanisms. Prove prior unscoped behaviour
  and canonical CSL are unchanged before deleting any duplicated consumer helper.
- [ ] If justified, add transport for typed, version-bound support observations.
  rcagent CAW09 owns semantic support, CAW10 temporal entitlement, and CAW21
  combined quality/coverage evaluation. Contradictory-evidence retrieval remains
  a consumer verification requirement; it is not the CAW10 deliverable.
  A `not_assessed` response remains useful.
- [ ] Run the matrix with network disabled and synthetic canaries. Verify there
  is no network fallback, exception/log leak, out-of-view cache reuse, silent
  observation promotion or post-render stale acceptance. Record actual commands,
  failures and exact input/output revisions.

## Review, migration and progression

- [ ] Review the bounded diff against this spec and the global no-truth/canonical
  data boundaries; apply in-scope review fixes and preserve unresolved findings.
- [ ] Obtain producer/consumer parity receipts at exact revisions, then decide
  whether the optional adapter can be selected. Keep the existing consumer local
  checks as the fallback; rollback disables this adapter without erasing evidence
  or changing canonical records. Re-run affected tests when the view, source,
  schema, runtime or consumer revision changes.

The first coding-agent slice is a small contract/fixture adapter; estimate its
effort after mapping existing types rather than making it a prerequisite for the
HDC study. No model training, research scoring, dependency installation, external
message, dataset release or deployment is part of this planning increment.
