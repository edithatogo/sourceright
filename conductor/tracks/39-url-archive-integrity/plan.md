# URL Archive Integrity Plan

1. Define URL/archive evidence fields and reporting classifications.
2. Add fixture-backed URL, DOI redirect, and archive response examples.
3. Implement opt-in URL reachability and redirect checking.
4. Add archive.org or Memento evidence capture where configured.
5. Surface broken-link and archive-coverage issues in policy/reporting outputs.
6. Route proposed URL/archive updates through review or writeback plans.

## Completion Signal

Reference reports can distinguish missing, broken, redirected, archived, and
unchecked URLs with low-noise sidecar evidence and no silent CSL mutation.

## Progress Notes

- 2026-05-12: First network-free integrity slice landed. Policy evaluation now
  validates canonical CSL URLs and classifies provider-backed redirect/archive
  evidence from the sidecar without mutating canonical data.
- 2026-05-12: Second network-free integrity slice landed. URL policy now
  distinguishes DOI landing-page evidence, missing archive evidence for
  non-DOI landing pages, and DOI URLs mistakenly supplied as archive snapshots.
- 2026-05-12: Third network-free integrity slice landed. Sidecar provider URL
  status evidence now classifies broken, offline, and unchecked URL states as
  deterministic policy issues without live network calls.
- 2026-05-13: Consolidation pass updated policy docs to list the URL/archive
  issue classes and reinforce that default checks do not fetch URLs.
