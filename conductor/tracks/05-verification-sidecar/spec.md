# Verification Sidecar Spec

## Goal

Store verification provenance without polluting canonical CSL JSON.

## Scope

- Provider candidates, confidence, conflicts, source provenance, review status, and decisions.
- Stable JSON suitable for CLI, MCP, and agent review.

## Outputs

- `references.verification.json`.
- `review-queue.jsonl` for records needing manual review.
