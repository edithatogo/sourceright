# URL Archive Integrity Spec

## Goal

Add URL, DOI landing-page, redirect, and archive evidence so reference audits can
flag broken or unstable online sources without noisy canonical overwrites.

## Scope

- Check URL reachability, HTTP status, redirects, and content-type hints.
- Check DOI URL resolution separately from bibliographic DOI normalization.
- Capture archive evidence from archive.org or Memento-compatible services when
  available.
- Suggest archive URLs or review actions without silently changing CSL.
- Classify broken, redirected, archived, paywalled, timeout, and unsupported
  cases for reporting.

## Outputs

- Sidecar evidence model extensions or provider candidates for URL/archive
  checks.
- Fixture-backed tests for reachable, broken, redirected, archived, and timeout
  cases.
- Policy/reporting issues for URL integrity and archive coverage.
- Documentation for archive-check configuration and expected noise limits.

## Boundaries

Network checks must be opt-in or fixture-backed by default. Suggested archive
or URL changes belong in review/writeback plans, not direct canonical CSL
mutation.
