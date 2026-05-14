# Contract Evidence And Overclaim Gates Plan

1. Build an evidence ledger from requirements, tracks, plugin registry, release
   status, and external proof suites.
2. Add policy tests for forbidden claims and unsupported status promotion.
3. Add docs guidance for allowed wording at each evidence level.
4. Update track templates or existing tracks to require review/evidence before
   completion.
5. Run local docs and policy tests.
6. Run `$conductor-review`.
7. Apply review fixes automatically and block status promotion until evidence
   exists or deferral is recorded.

## Completion Note

Track 47 implemented on 2025-07-16. All owned paths updated:

- **conductor/requirements.md**: Already contained MoSCoW, evidence levels, and
  overclaim guards for "Automatic final verification" and "AI detector" rows.
  Verified no changes needed.
- **conductor/design.md**: Already contained 7 Mermaid diagrams including
  Anti-Overclaim Gate. Verified no changes needed.
- **docs/src/limitations.md**: Added explicit "Forbidden claims" table covering
  all seven forbidden claim types with guard language.
- **docs/src/release-notes.md**: Already contained overclaim-aware wording.
  Verified no changes needed.
- **docs-site mirrors**: Created limitations.md and release-notes.md mirrors.
- **tests/requirements_contract_policy.rs**: Added 7 new tests: forbidden-claim
  scanning, evidence-ledger track 47 existence, limitations docs checks,
  docs-site parity for limitations and release notes, conductor design diagram
  count, and conductor requirements track 47 guards.
- **evidence-ledger.json**: Added fixture-backed entry for track 47.
- **metadata.json**: Status changed from "planned" to "completed".
- **review.md**: Created with completion review summary.
