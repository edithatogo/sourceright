---
title: Policy engine
description: Policy evaluation for reference, recency, and review behavior.
---

The policy engine turns repository policy into checks and reports.

- Keep policy evaluation deterministic.
- Make the policy outputs visible in reports and review queues.
- URL/archive findings are derived from CSL and sidecar evidence without live
  network calls in the default path.
- URL/archive issue codes distinguish invalid, redirected, archived, missing
  archive, DOI landing-page, broken, offline, and unchecked evidence states.
