# Mature Zotero Plugin Plan

1. Decide API mode: local Zotero API, Better BibTeX handoff, `.xpi`, or staged combination.
2. Implement fixture-backed preview/apply/audit contract.
3. Add disposable-library smoke guarded by explicit environment variables.
4. Add packaging if `.xpi` is selected.
5. Add public distribution notes and compatibility matrix.
6. Run targeted tests and `$conductor-review`.
7. Apply local fixes automatically; require explicit approval for live library writes or public submission.
