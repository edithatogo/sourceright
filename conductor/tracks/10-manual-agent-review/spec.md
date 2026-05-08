# Manual Agent Review Spec

## Goal

Create a review workflow for records that automated verification cannot resolve safely.

## Scope

- Queue generation.
- Agent/subagent partitioning.
- Original extracted text, provider candidates, diffs, confidence, and decision recording.
- Human-readable and machine-readable review outputs.

## Outputs

- `review-queue.jsonl`.
- Review decision records merged back into `references.verification.json`.
