Proceed with Slice 1: external schemas and examples.

Add schema files that reflect the current Rust models and documented output contracts. Do not replace Rust code. Do not invent incompatible field names.

Target additive files, adjusting names if the repo already has a convention:

```text
schemas/sourceright.verification.schema.json
schemas/sourceright.reference-report.schema.json
schemas/sourceright.review-queue.schema.json
schemas/sourceright.journal-screening.schema.json
schemas/sourceright.legal-citation-report.schema.json
schemas/sourceright.provenance-report.schema.json
schemas/sourceright.export-manifest.schema.json
schemas/sourceright.policy.schema.json
docs/schema-contracts.md
examples/workspace/ if the repo does not already have a better sample workspace
```

Requirements:
- First inspect current Rust structs/constants and output examples.
- Match current `schema_version` strings where they already exist.
- Keep `references.csl.json` canonical and sidecar metadata separate.
- Add a lightweight validation script only if useful.
- Run cargo tests and any new validation script.
- Summarize all added files and uncertainty.
- Do not commit.
