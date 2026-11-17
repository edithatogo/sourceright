# Scoped Evidence Consumers Spec

## Goal

Allow a consumer to bind a provenance result to its exact permitted evidence
view while distinguishing source identity, citation linkage, exact-span fidelity
and optional semantic-support observations. This is a planned extension, not a
runtime API or an implemented semantic verifier.

## User outcome

An investigator can see what was actually checked, against which source bytes
and view, and which questions remain unassessed. The first consumer is the
[rcagent cognitive-autopsy programme](https://github.com/edithatogo/rcagent/pull/117),
whose CAW08–10 and CAW21 own exact source verification, local support assessment,
temporal entitlement and combined evaluation. Contradictory-evidence retrieval
remains a separate consumer verification requirement. Its initial studies can
continue with existing local checks while this optional integration is pending.

## Scope

- Reuse the [Track 14](../14-claim-source-provenance/spec.md) graph and existing
  verification sidecar; keep canonical CSL and legal citations unchanged.
- Define a versioned, neutral binding from a consumer's trusted local policy to
  the allowed source revisions and spans. Carry an opaque view ID, content
  digest, consumer context/recipient-scope reference and the immutable policy
  revision. Temporal cutoffs are consumer-supplied context; Sourceright does not
  infer who knew what or assign clinical meanings to clocks.
- Check source identity, revision/hash and exact span within that binding. A
  valid citation to an out-of-view span must not pass this scoped check.
- Carry optional consumer/provider support observations without converting them
  into Sourceright findings of truth. Results retain the assessor, method/model
  and configuration revisions, source/claim/view digests and execution state.
- Recheck the binding after report transformations. A changed claim or evidence
  view invalidates the earlier result instead of silently reusing it.

## Out of scope

Clinical labels, decision quality, causal attribution, patient preferences,
eligibility policy and release decisions remain with the consumer. This track
does not build a general LLM/NLI subsystem, fetch case data from remote services,
alter the provider-runtime ownership boundary, or claim that multiple models
provide independent truth. StandardFlow PR #105 remains a separate integration.

## Data contracts

These are design requirements; implementation must choose additive versioned
schema fields and prove compatibility before exposing an API.

| Record | Required distinctions |
| --- | --- |
| Consumer evidence binding | Exact source revisions/hashes, allowed spans and coordinate convention; immutable view/policy digests; opaque context and recipient-scope references; any declared cutoff and uncertainty supplied by the consumer. Model output cannot expand the allowlist. |
| Deterministic result | Identity, citation linkage and span fidelity are separate checks with `passed`, `failed` or `not_run` and explicit reason/evidence. Missing local bytes do not pass. |
| Optional support observation | `supported`, `contradicted`, `insufficient_evidence` or `not_assessed`, separate from execution states such as completed, unavailable or error. Record issuer, method/configuration, exact claim digest, permitted view and evidence spans. These are scoped observations, not canonical truth. |
| Derived report | Preserve unresolved/conflicting observations, reference the same binding and invalidate stale results after edits; never write support verdicts into canonical CSL. |

For a local-only consumer operation, no case-derived query, identifier, span,
exception payload or result may fall back to a remote provider. Keep existing
explicit public-bibliography operations separate. Cache keys and derived reports
must include the binding identity so a broader view cannot contaminate a narrower
one. Generic code validates identity and declared membership; the consumer owns
whether the selected evidence is substantively appropriate.

## Claim boundary

The current [provenance implementation](../../../src/provenance.rs) establishes
linkage diagnostics, not semantic support. A supported observation is bounded by
its assessor and evidence; a faithfully represented account can remain disputed.
The absence of a citation, absence of evidence and contradiction are different
states. Hash/span validation does not establish clinical or factual truth.

## Evidence level target

First target: deterministic, synthetic-fixture consumer parity at pinned producer
and consumer revisions. Require failure-path and network-boundary evidence before
claiming a scoped local adapter. Keep benchmark quality, model qualification,
downstream compatibility and operational authorisation separate. No source/model
data are admitted or published by this planning change.

## Parallelization plan

Sourceright owns neutral provenance/binding and observation transport. rcagent
owns local semantic checking, temporal interpretation, clinical materiality and
patient-centred evaluation. Searchright owns shared public provider/query/receipt
primitives. Reuse maintained hashing, serialization and schema tooling after
existing contract tests pass; do not copy consumer inference code into this crate.
Confirm owned runtime paths before implementation and review shared schemas
serially. See the [plan](plan.md) and [test matrix](test-matrix.md).
