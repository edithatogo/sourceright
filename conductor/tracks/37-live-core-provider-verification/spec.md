# Live Core Provider Verification Spec

## Goal

Add opt-in live verification for the core academic providers that examiner-grade
reference audits need: Crossref, DataCite, OpenAlex, PubMed/NCBI, and DOI
resolution.

## Scope

- Query live provider APIs only when explicitly enabled.
- Cache provider responses with retrieval timestamps and enough request metadata
  to explain each result.
- Normalize provider evidence into sidecar candidates without mutating canonical
  CSL.
- Handle rate limits, missing identifiers, provider outages, malformed
  responses, and conflicting candidates deterministically.
- Keep fixture-backed tests as the default CI path.

## Outputs

- Provider client contracts for Crossref, DataCite, OpenAlex, PubMed/NCBI, and
  DOI resolution.
- Recorded fixtures for common match, no-match, conflict, and provider-error
  cases.
- Sidecar evidence entries with provider identity, retrieval time, confidence,
  and raw-enough payloads for audit.
- Documentation for live-provider configuration and trust boundaries.

## Boundaries

Live evidence must never silently overwrite canonical CSL. All canonical changes
must flow through conflict resolution, review decisions, or explicit writeback
plans.
