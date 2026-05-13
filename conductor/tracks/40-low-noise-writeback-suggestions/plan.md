# Low-Noise Writeback Suggestions Plan

1. Define the suggestion schema and confidence/noise thresholds.
2. Generate dry-run CSL patch plans from provider, conflict, URL, and archive
   evidence.
3. Add reviewer-facing explanations and evidence links for each suggestion.
4. Integrate suggestions with citation-manager preview and MCP dry-run write
   tools.
5. Add audit logging for explicit apply paths.
6. Add tests for suppressing weak or conflicting suggestions.

## Completion Signal

Sourceright can produce low-noise dry-run writeback plans that reviewers can
apply explicitly, with every suggested change traceable to sidecar evidence.

## Progress Notes

- 2026-05-12: First preview-noise slice landed. Citation-manager sync actions
  now expose suggestion kinds for `safe_update`, `no_op`, `low_confidence`, and
  `conflict` while preserving the existing explicit apply gate.
- 2026-05-12: Second preview-noise slice landed. Citation-manager sync preview
  actions now include reviewer-facing explanations for create, skip, update,
  and conflict branches without changing audit logging or apply semantics.
- 2026-05-12: Third preview-noise slice landed. Weak narrow-fit create
  suggestions can now be suppressed, narrow conflicts can be marked
  review-required, and apply/audit behavior records those outcomes explicitly.
- 2026-05-13: Consolidation pass updated citation-manager docs and schema notes
  for suppressed/review-required counters, suggestion classes, and explanations.
- 2026-05-13: Contract-hardening pass added citation-sync schema tests and a
  full schema inventory test so public schema files must remain valid JSON,
  packaged, and documented in both docs surfaces.
