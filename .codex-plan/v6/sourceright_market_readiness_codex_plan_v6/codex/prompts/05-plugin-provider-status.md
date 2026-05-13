# 05 — Plugin and provider status matrix

Make plugin/provider status clear and honest.

Tasks:
1. Inspect `plugins/registry.toml`, `plugins/manifests/`, provider modules, and live provider modules.
2. Add or update a status matrix:
   - implemented/core;
   - fixture-backed;
   - optional live smoke;
   - planned public API;
   - planned BYO-key;
   - planned adapter.
3. Document license/caching constraints for Dimensions, Scopus, Web of Science, and similar providers.
4. Ensure market-facing docs do not imply planned integrations are production-ready.
5. Run tests.

Do not implement new live providers in this slice.
