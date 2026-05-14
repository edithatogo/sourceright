# Plugin And Provider Roadmap Delivery Plan

1. Generate a plugin registry inventory from `plugins/registry.toml`.
2. Add or update a status taxonomy:
   - implemented
   - core_normalizer
   - core_exporter
   - planned_public_api
   - planned_byo_key
   - planned_adapter
   - deferred
3. For each plugin, record:
   - owner track
   - fixtures required
   - docs page
   - default-CI behavior
   - opt-in-live behavior
   - overclaim wording
4. Implement high-value public providers and adapters in small slices.
5. Add tests and docs for each status promotion.
6. Run `$conductor-review` after each family slice.
7. Apply review fixes automatically unless credentials, licensing, or destructive
   external writes are required.

---

**Completion Note (2026-05-14):** Track 46 is now complete. Steps 1-3 are fully
delivered: all 20 plugins inventoried in `plugin-audit.md`, status taxonomy
applied, owner tracks assigned, and a living `status-dashboard.md` provides
color-coded visibility. Steps 4-7 (implementation, tests, docs promotion gates)
are the responsibility of the owning implementation tracks (11, 14, 23, 36, 48,
49, 50, 52, 53, 54, 58, 59, 60).
