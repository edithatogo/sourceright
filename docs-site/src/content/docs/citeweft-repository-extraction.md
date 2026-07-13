---
title: CiteWeft repository extraction
description: The governed boundary between Sourceright and the external CiteWeft crate.
---

Sourceright consumes the published CiteWeft candidate at a pinned Git revision.
The candidate contains the neutral extraction core; Sourceright retains the
adapter, routing, policy, and verification boundaries.

The migration is validated by local Rust gates, the packaged consumer smoke,
the candidate checksum, and independent CiteWeft CI. Stable-release promotion
and rollback remain explicit Conductor gates.
