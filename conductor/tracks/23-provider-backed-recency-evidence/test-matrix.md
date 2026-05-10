# Provider-Backed Recency Evidence Test Matrix

| Scenario | Expected result |
| --- | --- |
| Retraction evidence | Report surfaces provider-backed retraction signal with source and severity. |
| Correction evidence | Report surfaces correction or erratum signal without treating it as automatic invalidation. |
| Expression of concern | Report surfaces integrity concern and recommends review. |
| Preprint evidence | Report identifies preprint status without treating it as an error by default. |
| Superseded guideline | Report flags newer or superseding guideline evidence when provider data supports it. |
| Publication age | Report surfaces age policy signals according to configured thresholds. |
| Sidecar storage | Evidence is stored in verification sidecar, not canonical CSL. |
| Journal report | Editor-facing and author-facing outputs include conservative recency/integrity language. |
| No claim truth assertion | Reports do not assert that cited claims are true or false. |
| No AI authorship assertion | Reports do not assert that issues prove AI authorship. |
