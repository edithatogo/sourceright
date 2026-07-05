# Track 84 - Test Matrix

| Scenario | Acceptance | Evidence | Default-CI or opt-in-live |
|---|---|---|---|
| Adapter skeleton is defined | The Janeway adapter boundary is documented and separated from core screening logic | `spec.md` and `plan.md` describe the package shape and command boundary | default-CI |
| Package metadata is explicit | The track defines the metadata and install packaging expectations needed for Janeway | `spec.md` includes the package contract and claim boundary | default-CI |
| Preview-first boundary is preserved | The adapter contract keeps screening logic in the Rust core and does not imply write-capable host behavior | Review of `spec.md` and `plan.md` | default-CI |
| Fixture-backed checks are defined | The test matrix names deterministic checks that do not need a live Janeway system | `test-matrix.md` contains fixture-backed package contract scenarios | default-CI |
