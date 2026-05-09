# Standardisation Cleaning Plan

1. Define normalization rules and diagnostics. Implemented through `CleaningReport` and transformation records.
2. Implement identifier cleaning. Implemented for DOI, id, title, and page-range normalization.
3. Implement name/date/container normalization. Date-risk detection is implemented; richer name/container parsing remains pending.
4. Implement duplicate detection. Implemented through duplicate grouping by DOI or type/title fallback.
5. Record material transformations for review. Implemented with review flags and sidecar queue status for risky changes and duplicates.
