# Low-Noise Writeback Suggestions Spec

## Goal

Produce examiner-usable writeback suggestions that explain exactly what should
change, why, and what evidence supports it, while keeping apply actions explicit
and auditable.

## Scope

- Generate dry-run CSL patch plans for missing DOI, corrected metadata, URL, and
  archive suggestions.
- Rank suggestions by confidence, evidence type, and expected reviewer effort.
- Suppress noisy or weak suggestions unless they add review value.
- Integrate with citation-manager sync previews and MCP dry-run write tools.
- Require explicit apply semantics and audit logs for any write action.

## Outputs

- Writeback suggestion schema and fixtures.
- CLI/MCP dry-run outputs that separate safe suggestions, review-required
  suggestions, and rejected noisy suggestions.
- Audit logs for applied changes.
- Documentation for thresholds, review semantics, and no-silent-overwrite rules.

## Boundaries

This track must not turn provider evidence into automatic canonical changes.
Apply mode remains explicit, reviewable, and auditable.
