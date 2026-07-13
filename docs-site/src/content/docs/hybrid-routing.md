---
title: Hybrid routing
description: Explicit backend selection, fallback, resource, and observability policies.
---

Sourceright exposes explicit manual and `auto` backend routing contracts. Each
decision records policy, ordered attempts, versions, configuration fingerprints,
reasons, accepted backend or abstention, and a deterministic cache key.

Manual mode never silently falls back. Auto mode compares only matching task
and calibration identifiers, rejects oversized input before backend selection,
and keeps experimental backends disabled by default. Diagnostic helpers redact
common secret fields before logs are emitted.

This is fixture-backed routing policy evidence. It does not claim production
throughput, model quality, universal optimality, live rollback, or hidden cloud
fallback behavior.
