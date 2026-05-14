# Expanded Normaliser Provider Catalogue Plan

1. Build candidate provider inventory by domain.
2. Classify access: public API, licensed/BYO-key, search-only, repository,
   prohibited/deferred.
3. Add plugin manifests or explicit deferrals for accepted candidates.
4. Add fixtures for high-priority public providers.
5. Update provider docs and requirements matrix.
6. Run registry validation and `$conductor-review`.
7. Apply local fixes automatically.

---

## Completion Note (Track 62 Complete)

All provider catalogue work is documented in `docs/src/providers.md` as the
**Expanded Provider Catalogue** table. The table covers 25 provider candidates
across scholarly, economics, grey literature/repository, search, and biomedical
domains. Each row documents access model, current status, owning track, evidence
level, fixture coverage, default-CI behaviour, and an explicit overclaim guard
string.

Key decisions documented:
- **Economics providers** (RePEc, SSRN, NBER, EconLit, IDEAS): All deferred
  due to lack of unified REST API, licensing barriers, or bulk-only data formats.
  Rationale recorded in `economics-decision-log`.
- **Google Scholar**: Permanently prohibited per ADR 0005. Documented as
  assessment-only with no manifest, no fixtures, no CI path.
- **Grey literature/repositories**: Zenodo, OSF, Figshare, Dataverse are
  planned under the `provider.repository-records` manifest. Institutional
  repositories deferred due to platform diversity.
- **Clinical trial registries**: Deferred to a future health-sciences track.

Plugin manifests were **not** modified (they belong to tracks 48, 49, 50).
No plugin manifests were added or changed. All assessment-only/deferred
providers were logged in docs only.
