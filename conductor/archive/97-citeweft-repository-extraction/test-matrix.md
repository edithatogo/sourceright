# Track 97 Test Matrix

| Scenario | Acceptance | Evidence | Default-CI or opt-in-live |
| --- | --- | --- | --- |
| Core inventory | Every standalone-neutral path exists, is classified, and does not overlap adapters. | Manifest/verifier. | Default-CI |
| Forbidden imports | Neutral modules do not import CSL, sidecar, workspace, intake, or provider modules. | PowerShell verifier. | Default-CI |
| Adapter boundary | Sourceright adapters are listed separately and are not claimed standalone. | Manifest policy test. | Default-CI |
| Schema/license gates | IR/model schema versions, license policy, and open external gates are recorded. | JSON policy test. | Default-CI |
| History rehearsal | Representative history survives a disposable split before any remote write. | Rehearsal record. | Opt-in |
| Independent CI/package | Fresh standalone clone builds, tests, docs, security, and package dry runs. | External candidate artifact. | Opt-in |
| Downstream compatibility | Sourceright consumes immutable CiteWeft candidate and passes full suite. | Downstream CI. | Opt-in |
| Publication/rollback | Live release, provenance, checksums, issue migration, and rollback are verified. | External release packet. | Opt-in |
