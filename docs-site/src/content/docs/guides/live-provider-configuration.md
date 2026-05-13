---
title: Live Provider Configuration
description: Opt-in live provider settings and safety boundaries.
---

Live provider access is opt-in. Keep the default path fixture-backed unless a
session explicitly needs real provider responses.

Set both global flags before live smoke checks:

```text
SOURCERIGHT_LIVE_PROVIDERS=1
SOURCERIGHT_LIVE_PROVIDER_SMOKE=1
```

Capture live responses as sidecar evidence. Do not silently overwrite canonical
CSL.
