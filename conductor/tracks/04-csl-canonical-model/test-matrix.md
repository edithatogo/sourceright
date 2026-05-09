# CSL Canonical Model Test Matrix

| Scenario | Expected result |
| --- | --- |
| Valid article record | Serializes to stable CSL JSON. |
| Missing required fields | Emits deterministic validation diagnostics. |
| Non-canonical IDs | Emits deterministic diagnostics for whitespace normalization and duplicate normalized IDs. |
| Non-canonical types, titles, and DOIs | Emits deterministic diagnostics and exposes normalization helpers for provider matching. |
| Style-specific input | Normalizes into style-neutral CSL fields while preserving unknown CSL payload fields. |
| Verification metadata | Is rejected or moved to the sidecar boundary. |
| Canonical read/write formatting | Parses CSL JSON through the canonical entrypoint and writes deterministic, newline-terminated pretty JSON with stable key ordering. |
