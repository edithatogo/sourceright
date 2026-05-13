---
title: Limitations
description: Current implementation limits and hardening tracks for Sourceright.
---

Sourceright is currently suited to structured reference triage, deterministic
fixture-backed validation, and review queueing. It is not yet a final
examiner-grade verifier.

Current gaps are concentrated in the practical workflow:

- Real DOCX and PDF extraction still needs the dedicated adapters.
- OCR-backed scanned PDFs are still represented as explicit diagnostics.
- Live Crossref, DataCite, OpenAlex, and PubMed evidence remains opt-in.
- Provider evidence must not silently overwrite canonical CSL.
- Legal citations remain separate from academic CSL.
- Claim/source/provenance work links claims and sources without asserting claim
  truth.
- Smarter institutional-author and same-author citation handling still needs
  hardening.
- URL and archive integrity checks are still being expanded.
- Low-noise writeback suggestions are still being refined.
- Benchmark results are deterministic and fixture-backed; they are not a live
  provider quality claim.
- Plugin/provider statuses marked as planned or technical preview are roadmap
  surfaces, not current live promises.

The source workflow calls out Tracks 36-40 as the next hardening slice. For the
current operator surface, start with [Workflow](workflow) and review the
[benchmark guide](benchmarks) for the fixture-backed technical-preview gate.
