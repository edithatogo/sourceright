---
title: Native entity model
description: Optional source-grounded entity recognition and linking boundaries.
---

Sourceright exposes an optional entity schema independent from reference
extraction and canonical CSL. Mentions preserve source spans, original labels,
mapped classes, mapping relations, confidence, and separate link candidates.

The checked-in baseline is a deterministic lexicon recognizer for a
self-authored general-scholarly fixture. Link candidates are evidence only;
default execution does not query registries. Biomedical and legal packs,
learned NER models, calibration, and GROBID-NER compatibility bridges require
separate domain and licensing evidence.
