# Manual Review

Manual review is a normal workflow stage, not an error path.

Records should enter the queue when provider data conflicts, confidence is low, extraction is uncertain, or transformations are risky. Review items should include:

- Original extracted text.
- Canonical candidate record.
- Provider candidates.
- Field-level diffs.
- Confidence and diagnostics.
- Accept, reject, merge, or unresolved decisions.

The queue is partitionable for subagents without losing provenance or duplicating active ownership. Use:

- `sourceright review queue [.sourceright-directory]`
- `sourceright review partitions [--size <n>] [.sourceright-directory]`
- `sourceright review import-decisions <decisions.json> [.sourceright-directory]`

Decision imports update `references.verification.json` and refresh `review-queue.jsonl`.
