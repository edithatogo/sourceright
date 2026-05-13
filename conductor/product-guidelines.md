# Sourceright Product Guidelines

## Product Positioning

- Describe Sourceright as reference triage and verification infrastructure, not
  as a fully automated final verifier until examiner-grade tracks are complete.
- Keep current trust boundaries explicit: deterministic checks, provider
  evidence, review queues, and export generation are usable now; robust
  DOCX/PDF extraction, live core-provider verification, citation
  disambiguation, URL/archive checking, and writeback suggestions remain
  hardening work.
- Do not imply that citation errors prove AI generation or that a source proves
  the truth of a claim.

## Data Boundaries

- `references.csl.json` is canonical clean academic bibliographic data.
- `references.verification.json` stores extraction provenance, provider
  candidates, confidence, conflicts, review state, and evidence.
- `review-queue.jsonl` is derived operational work and must be reproducible from
  canonical references plus sidecar state.
- Provider data must never silently overwrite canonical CSL.
- Legal citations remain separate from academic CSL.
- Claim/source/provenance work may link claims, citations, and sources, but must
  not assert claim truth.

## Implementation Rules

- Prefer additive, track-scoped changes that preserve existing Rust module
  boundaries.
- Keep provider integrations behind deterministic contracts with fixture-backed
  tests by default.
- Treat live network calls, citation-manager writes, and MCP write tools as
  explicit opt-in operations with audit trails.
- Preserve original extracted text and source spans whenever extraction results
  enter review or verification surfaces.
- Route uncertain, conflicting, or low-confidence changes through review queues
  or dry-run writeback plans.

## Review And Release Rules

- Substantial work should have a Conductor track with `spec.md`, `plan.md`,
  `test-matrix.md`, and `metadata.json`.
- Track status must not be promoted beyond the evidence in tests, docs, and
  implementation.
- Public docs, README wording, and Conductor status must use the same capability
  boundary language.
- Public demos must remain sample-data-only unless a future track explicitly
  adds live verification behavior, credentials, and audit semantics.
- Public examples must stay parseable, fixture-backed, and conservative: case
  studies are synthetic, citation-manager profiles default to dry-run, and MCP
  write examples must show explicit apply boundaries.
- Before closing implementation work, run the applicable Rust checks and any new
  validation scripts.
