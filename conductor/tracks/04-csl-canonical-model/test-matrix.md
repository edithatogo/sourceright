# CSL Canonical Model Test Matrix

| Scenario | Expected result |
| --- | --- |
| Valid article record | Serializes to stable CSL JSON. |
| Missing required fields | Emits deterministic validation diagnostics. |
| Style-specific input | Normalizes into style-neutral CSL fields. |
| Verification metadata | Is rejected or moved to the sidecar boundary. |
