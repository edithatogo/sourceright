# Track 72 - Test Matrix

| Area | Check | Expected |
| --- | --- | --- |
| contract doc | shared submission contract exists | `conductor/submission-contracts.md` names all target surfaces |
| docs mirrors | public mirrors stay aligned | `docs/src/submission-contracts.md` and docs-site mirror match |
| inventory | machine-readable requirements inventory exists | `conductor/submission-requirements.json` parses and lists all surfaces |
| packet manifest | surface packets are listed | `conductor/submission-packets/manifest.json` covers every family |
| readiness workflow | self-improving controls are wired | workflow and verifier script references are present |
| approval boundary | external submission stays approval-gated | explicit human approval remains required |
