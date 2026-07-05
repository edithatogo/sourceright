# Track 85 - Test Matrix

| Scenario | Acceptance | Evidence | Default-CI or opt-in-live |
|---|---|---|---|
| Disposable-instance install recorded | The transcript shows the Janeway install or registration step | `live-smoke-*.md` contains the install transcript | opt-in-live |
| Plugin or bridge activation recorded | The transcript shows that Sourceright was activated or registered in Janeway | `live-smoke-*.md` contains the activation step and result | opt-in-live |
| Screening report retrieved | The transcript captures a screening report emitted through the Janeway path | `live-smoke-*.md` contains the report output | opt-in-live |
| Blocker documented when live path is unavailable | The track records the exact blocker and attempted command chain instead of claiming success | `live-smoke-*.md` or an equivalent blocker note contains the failure reason | opt-in-live |
