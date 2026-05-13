---
title: Citation manager integrations
description: Citation manager sync and profile support.
---

Citation manager integrations focus on previewable, auditable sync paths.

- Keep profile mappings explicit.
- Require preview before apply for write paths.
- Treat `suppressed` and `review_required` suggestions as review controls, not
  automatic writes.
- Surface `suppressed_count`, `review_required_count`, suggestion class, and
  explanation text in preview/apply JSON.
