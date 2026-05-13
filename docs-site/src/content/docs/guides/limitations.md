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
- Smarter institutional-author and same-author citation handling still needs
  hardening.
- URL and archive integrity checks are still being expanded.
- Low-noise writeback suggestions are still being refined.

The source workflow calls out Tracks 36-40 as the next hardening slice. For the
current operator surface, start with [Workflow](workflow) and review the
[benchmark guide](benchmarks) for the fixture-backed technical-preview gate.
