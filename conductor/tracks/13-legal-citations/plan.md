# Legal Citations Plan

1. Define legal citation model separate from CSL where needed. Implemented through `LegalCitationRecord`.
2. Evaluate public APIs such as CourtListener and Caselaw Access Project. Implemented as provider candidate model slots, with AU public-provider hints for neutral citations.
3. Design provider confidence and conflict rules for jurisdiction-specific citations. Implemented as jurisdiction/provider issue diagnostics and confidence-bearing candidates.
4. Design exports or reports that legal users can actually consume. Implemented as JSON and Markdown legal citation reports.
5. Integrate with manual agent review after academic queue patterns are stable. Implemented as legal citation issue records suitable for review routing.
