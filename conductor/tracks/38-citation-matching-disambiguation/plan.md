# Citation Matching Disambiguation Plan

1. Audit current reconciliation failures against institutional-author and
   same-author examples.
2. Add fixture cases for group authors, same-author same-year suffixes, `et al.`
   variants, title fallbacks, and Vancouver references.
3. Implement normalized author-key generation for people and organizations.
4. Add ambiguity scoring and review diagnostics for unsafe matches.
5. Update reports and docs to explain the difference between matched,
   ambiguous, missing, uncited, and out-of-order citations.

## Completion Signal

Citation reconciliation handles institutional authors and same-author citations
with stable low-noise diagnostics across checked-in fixtures.

## Progress Notes

- 2026-05-12: First disambiguation slice landed. Author-date matching now
  prefers full institutional author phrases before person-surname fallbacks, and
  same-author same-year suffixes such as `2024a` and `2024b` resolve
  deterministically by reference order.
- 2026-05-12: Second disambiguation slice landed. Multi-author citations and
  `et al.` variants now try full sequence and `first et al` keys before falling
  back to first-author matching.
- 2026-05-12: Third diagnostic slice landed. Reconciliation now emits low-noise
  diagnostics for mixed author-date/numeric style manuscripts and for
  title-derived fallback matches, while author-backed matches take precedence.
- 2026-05-13: Consolidation pass updated reporting/workflow docs to describe
  mixed-style and title-fallback diagnostics as manual-review signals.
- 2026-05-14: Completion review. Verified all 6 test-matrix scenarios have
  corresponding unit tests in `src/reconcile.rs` covering institutional authors,
  same-author same-year suffixes, et al. variants, Vancouver numeric citations,
  mixed-style manuscripts, title-fallback matches, and author-precedence behavior.
  Track status updated to completed.
