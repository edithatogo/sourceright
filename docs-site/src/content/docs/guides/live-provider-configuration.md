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

Runtime controls are explicit and conservative:

| Setting | Environment variable | Default |
| --- | --- | --- |
| Request timeout | `SOURCERIGHT_PROVIDER_TIMEOUT_SECS` | `20` |
| Minimum interval | `SOURCERIGHT_PROVIDER_MIN_INTERVAL_MS` | `1000` |
| Retry ceiling | `SOURCERIGHT_PROVIDER_MAX_RETRIES` | `2` |
| Cache directory | `SOURCERIGHT_PROVIDER_CACHE_DIR` | unset |

Live smoke reports include a `runtime_controls` object with `timeout_secs`,
`min_interval_ms`, `max_retries`, and `cache_enabled`. Keep that block with
provider outcomes when recording external proof.

Cache hits remain provider evidence with provenance. They must not silently
update canonical CSL or hide rate-limit, outage, malformed-response, or
conflict diagnostics.
