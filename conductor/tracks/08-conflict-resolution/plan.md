# Conflict Resolution Plan

1. Define confidence inputs and thresholds. Implemented with `ConflictResolutionPolicy`.
2. Implement deterministic merge decisions for safe cases. Implemented for high-confidence provider values filling missing canonical fields.
3. Preserve unresolved conflicts in sidecar metadata. Implemented for provider/canonical disagreements.
4. Route ambiguous records to review queues. Implemented for plausible provider values below the auto-merge threshold and provider disagreements.
5. Add explainable CLI output for conflicts. Implemented through `sourceright conflicts`.
