# Live Provider Adapters Plan

1. Define live adapter policy.
   - Establish opt-in flags and credential environment variables.
   - Define default fixture behavior and skipped live smoke behavior.
   - Document rate-limit and privacy expectations.

2. Implement public live adapters.
   - Add Unpaywall evidence capture.
   - Add OpenCitations evidence capture.
   - Add arXiv and Europe PMC evidence capture.
   - Add repository record evidence capture where provider metadata is available.

3. Add licensed provider hooks.
   - Define bring-your-own-key provider configuration.
   - Ensure missing credentials skip live smoke tests cleanly.
   - Avoid embedding secrets or provider-specific entitlements.

4. Preserve sidecar and conflict boundaries.
   - Store matches, confidence, conflicts, and provenance in sidecar evidence.
   - Do not mutate canonical CSL from live responses.
   - Route conflicts into existing conflict resolution and manual review paths.

5. Add tests and docs.
   - Keep recorded/fixture tests as the default.
   - Add credential-gated live smoke tests.
   - Document provider configuration and evidence interpretation.
