# Track 96 Test Matrix

| Scenario | Acceptance | Evidence | Default-CI or opt-in-live |
| --- | --- | --- | --- |
| Manual selection | Requested unavailable backend fails explicitly without fallback. | Policy test. | Default-CI |
| Auto trace | Ordered attempts, reasons, accepted backend, and policy are serialized. | Golden fixture. | Default-CI |
| Confidence comparability | Different task/calibration IDs cannot be numerically compared. | Policy test. | Default-CI |
| Resource budget | Oversized input abstains before backend selection. | Policy test. | Default-CI |
| Cache replay | Same inputs produce same key; mode/backend/model/config/options changes alter it. | Hash test. | Default-CI |
| Redaction | Secrets, empty secret values, and sensitive key values are safely handled. | Redaction test. | Default-CI |
| Field merge | No merge is performed by the route policy. | Schema boundary. | Default-CI |
| Load/deadline/rollback | Live fault and deployment evidence precede production claims. | Deferred operational drills. | Opt-in |
