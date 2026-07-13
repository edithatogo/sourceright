# Native Scholarly NER and Entity Linking Plan

1. **[x] Discover.** Inspect the live `EntityRecognizer` boundary and the
   historical GROBID-NER decision without importing the old scaffold.
2. **[x] Lock documents.** Define vocabulary, mapping relations, link evidence,
   provenance, and domain claim boundaries.
3. **[x] Implement.** Add the deterministic lexicon baseline, audit/ledger,
   fixture, and policy tests.
4. **[x] Validate.** Run formatting, fixture, serialization, mapping, and
   workflow checks; attempt Rust tests and record toolchain limits.
5. **[x] Review locally.** Check span preservation, mapping lossiness, link
   separation, and citation independence.
6. **[x] Apply fixes.** Resolve local findings before closeout.
7. **[x] Closeout.** Record learned model, domain-pack, and bridge deferrals;
   leave archive cleanup to review.

## Review fixes

- [x] Reject inputs above 25 MiB before matching.
- [x] Reject empty lexicon patterns to prevent zero-width match expansion.

## Completion signal

An optional source-grounded entity schema and deterministic fixture baseline are
available with mapping/link provenance and a GROBID-NER audit; reference
extraction remains independent when NER is not selected.

## Validation note

`cargo test` was attempted with the locked dependency graph but remains blocked
by the workstation linker: host build scripts resolve Git's Unix `link.exe`
instead of a usable MSVC linker. Formatting, diff, fixture, JSON, and workflow
checks are the available local evidence.
