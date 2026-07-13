---
title: Native reference model
description: Deterministic, source-grounded reference and citation extraction.
---

The Rust core includes a deterministic, source-grounded baseline for finding a
References/Bibliography section, segmenting numbered entries, extracting
conservative author/title/year/DOI evidence, and linking numeric callouts.

Every candidate and callout retains a byte span. Unknown callouts and weak
parses are routed to `review`; `extracted` is not a verification claim. No provider is queried and no canonical CSL is
constructed or mutated. The checked-in model manifest records the labels,
runtime, configuration, license, and absence of a learned artifact.

This is fixture-backed technical-preview behavior, not universal citation-style
support, bibliographic truth verification, or a learned model claim.
