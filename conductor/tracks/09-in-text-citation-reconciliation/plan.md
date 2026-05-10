# In-Text Citation Reconciliation Plan

1. Define citation occurrence model. Implemented through `CitationOccurrence`.
2. Implement style-tolerant citation detection for initial academic styles. Implemented for author-date and numeric citations.
3. Match occurrences to CSL reference IDs. Implemented with author-key and numeric-position matching.
4. Report missing, uncited, duplicate, and order issues. Implemented in `CitationReconciliationReport`.
5. Route ambiguous matches to manual review. Implemented as explicit ambiguous-match report issues for manual review routing.
