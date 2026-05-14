# Publication Registry Completion Plan

1. Inventory current registry evidence from repo docs and live public listings.
2. For each registry surface, classify status:
   - accepted
   - submitted
   - prepared
   - blocked
   - deferred
   - not applicable
3. Close easy gaps.
   - Add missing metadata, URLs, labels, install commands, or docs.
   - Do not publish to a new registry without a maintainable validation path.
4. Add validation scripts or smoke checks where useful.
5. Update release-status docs and Conductor evidence.
6. Run `$conductor-review`.
7. Apply local fixes, then progress to any next registry only when the previous
   one has evidence or a recorded deferral.
