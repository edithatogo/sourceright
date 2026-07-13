# Track 92 Test Matrix

| Scenario | Acceptance | Evidence | Default-CI or opt-in-live |
| --- | --- | --- | --- |
| Manifest validation | Missing hash/license/access/split/cohort or live network is rejected. | Rust unit tests and CLI run. | Default-CI |
| Input integrity | Fixture bytes match the manifest SHA-256. | `extraction-bench` report. | Default-CI |
| Metric goldens | Hand-computed precision/recall/F1 values are stable. | Unit test and JSON report. | Default-CI |
| Leakage detector | A content hash cannot cross split boundaries. | Manifest validation test. | Default-CI |
| Missing coordinates | Coordinates report unavailable, not a zero score. | Unit test and fixture report. | Default-CI |
| Stage attribution | Segmentation, field, and callout stages remain separate. | Report schema and fixture. | Default-CI |
| Backend provenance | Version, model, config fingerprint, and hardware are reported. | Fixture report. | Default-CI |
| Operations metadata | Status and latency are reported; peak memory may remain unavailable. | Fixture report schema. | Default-CI |
| Determinism | Repeated JSON runs are byte-stable. | PowerShell comparison. | Default-CI |
| Restricted corpus boundary | No restricted corpus is checked in; future opt-in data has retrieval/license evidence. | README and scope contract. | Opt-in |
