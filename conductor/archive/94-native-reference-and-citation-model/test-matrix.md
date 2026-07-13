# Track 94 Test Matrix

| Scenario | Acceptance | Evidence | Default-CI or opt-in-live |
| --- | --- | --- | --- |
| Section detection | References heading and source offset are retained. | Fixture test. | Default-CI |
| Entry segmentation | Numbered entries become separate raw-text candidates. | Fixture test. | Default-CI |
| Field grounding | Extracted fields have source spans inside the raw entry. | Policy test. | Default-CI |
| DOI evidence | DOI is conservative and never fetched or verified. | Fixture test. | Default-CI |
| Callout linking | Numeric callouts link only to known reference IDs; unknowns abstain. | Fixture test. | Default-CI |
| Confidence routing | Weak parses are marked `review`; `extracted` is not verification. | Unit test. | Default-CI |
| Resource limit | Inputs above the deterministic baseline limit are rejected before segmentation. | Unit test. | Default-CI |
| Model manifest | Labels, schema, backend, license, and runtime are mandatory. | JSON policy test. | Default-CI |
| CSL isolation | Output serializes without canonical CSL or sidecar mutation. | Serialization test. | Default-CI |
| Learned runtime | Portable model parity, calibration, and resource evidence precede any model artifact. | Deferred plan/issue. | Opt-in |
