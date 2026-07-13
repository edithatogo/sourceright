# Rollback and issue migration

## Current candidate

- CiteWeft release: `v0.1.0-candidate.1`
- CiteWeft revision: `8c8932976250f9ca91c2bbda28ed68eeb191fa42`
- Sourceright migration commit: `ad1d675`
- Tracking issue: [CiteWeft #3](https://github.com/edithatogo/citeweft/issues/3)

## Reversible rollback

If candidate compatibility or security evidence regresses:

1. Revert the Sourceright dependency/wiring commit `ad1d675` in a reviewed
   pull request.
2. Restore the prior Sourceright neutral modules from commit `17f090e` only if
   the rollback requires local implementation availability.
3. Run the full locked Rust gates, `cargo-audit`, `cargo-deny`, and the
   CiteWeft extraction policy checks.
4. Record the failed candidate, replacement revision, checksums, and affected
   downstream adapters in the tracking issue.

No release deletion or history rewrite is required for rollback.

## Issue migration

- [x] Link the independent release/compatibility work to CiteWeft issue #3.
- [x] Record candidate checksum and packaged consumer smoke.
- [ ] Close or supersede Sourceright Track 97 only after rollback evidence and
  stable-release approval.
