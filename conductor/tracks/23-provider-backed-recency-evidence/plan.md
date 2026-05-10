# Provider-Backed Recency Evidence Plan

1. Define evidence categories.
   - Model retraction, correction, expression-of-concern, preprint, superseded-guideline, and publication-age evidence.
   - Include provider source, retrieval time, confidence, and affected reference id.

2. Wire provider evidence.
   - Consume live or fixture-backed provider adapter outputs.
   - Store evidence in `references.verification.json`.
   - Preserve conflicts and review state.

3. Add policy classification.
   - Map evidence categories to report severity and review recommendations.
   - Keep policy configurable where journal or workflow context differs.
   - Avoid claim-truth and AI-authorship assertions.

4. Extend reports.
   - Add reference report sections for recency and integrity evidence.
   - Add journal screening fields for editor-facing and author-facing outputs.
   - Keep author-facing language actionable and conservative.

5. Add tests and documentation.
   - Use fixture-backed provider evidence by default.
   - Cover report rendering and policy severity.
   - Document limitations and evidence interpretation.
