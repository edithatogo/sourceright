# Mature Zotero Plugin Spec

## Goal

Deliver mature Zotero integration that can be shared publicly, not just a file
export claim.

## Completion Contract

- Dry-run preview compares Zotero records with Sourceright CSL/sidecar state.
- Apply requires explicit user action and writes an audit log.
- A disposable-library smoke gate is ignored by default, skips cleanly without
  credentials, and only performs live Zotero discovery/planning when explicitly
  enabled with disposable-library environment variables.
- If a Zotero `.xpi` is built, it has packaging, install notes, versioning,
  permissions, and public distribution notes.
- Distribution notes cover Zotero Forums/plugin listing state without claiming
  official acceptance before evidence exists.

## Parallelization

- Subagent A: Zotero data/API contract.
- Subagent B: preview/apply/audit engine.
- Subagent C: `.xpi` packaging and install docs.
- Subagent D: tests and disposable-library smoke.
