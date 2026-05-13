# Citation Matching Disambiguation Spec

## Goal

Reduce false positives and false negatives in citation reconciliation for
examiner-grade audits, especially around group authors and ambiguous author-year
citations.

## Scope

- Support institutional and group authors without treating every token as a
  personal surname.
- Handle same-author same-year suffixes such as 2024a and 2024b.
- Improve `et al.`, ampersand, initials, particles, organization names, and
  title-derived fallback matching.
- Preserve ambiguity as review issues when automated matching is not safe.
- Cover author-date, numeric/Vancouver, and mixed citation styles with fixtures.

## Outputs

- Improved citation matching heuristics and diagnostics.
- Ambiguity classifications that are useful to human reviewers.
- Regression fixtures for institutional authors, same-author years, and
  style-mixed manuscripts.
- Documentation describing how low-confidence matches are handled.

## Boundaries

The matcher should prefer low-noise review issues over aggressive automatic
matches. It must not assert that a citation supports a claim; it only reconciles
citation mentions against reference-list records.
