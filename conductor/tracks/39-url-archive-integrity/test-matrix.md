# URL Archive Integrity Test Matrix

| Scenario | Expected coverage |
| --- | --- |
| Reachable URL | Records status and retrieval evidence. |
| Redirecting URL | Reports final URL without overwriting CSL. |
| Broken URL | Emits review issue with status/error evidence. |
| DOI landing page | Separates DOI normalization from URL resolution. |
| Archived URL | Records archive candidate evidence. |
| Timeout/offline | Emits deterministic unchecked/failed diagnostic. |
