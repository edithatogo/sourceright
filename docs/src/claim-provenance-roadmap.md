# Claim And Provenance Layer

Claim/source/provenance work builds on reliable reference verification.

The current baseline includes:

- Claim extraction.
- Claim-to-citation linkage.
- Evidence graphs.
- Document-level provenance summaries.

Use `sourceright provenance <document-text.txt>` to build a compact JSON graph of claim nodes, detected citation source nodes, and claim/source links. The report identifies unsupported claims and quoted claims without detected citation support.

This layer does not assert whether a claim is true. Quote verification, paraphrase verification, source-quality scoring, and semantic evidence grading remain future provider-backed work.
