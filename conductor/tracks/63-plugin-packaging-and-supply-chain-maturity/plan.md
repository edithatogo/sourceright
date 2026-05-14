# Plugin Packaging And Supply-Chain Maturity Plan

1. Formalize the no-submodule default and split criteria.
2. Add plugin evidence ledger schema or docs.
3. Add status, compatibility, provenance, and deprecation requirements.
4. Add signing/pinning and SBOM expectations for installable plugins.
5. Update plugin authoring and registry docs.
6. Run schema/docs tests and `$conductor-review`.
7. Apply local fixes automatically.

## Progress

Track 63 has been implemented as follows:

- **docs/src/plugin-authoring.md** — fully updated with:
  - Packaging Policy section: no-submodule default and four explicit split
    criteria (independent release lifecycle, separate maintainers,
    host-specific packaging, stable compatibility contract).
  - Evidence-Ledger Requirements table mapping status levels to fixture,
    docs, install path, version/compatibility, signed/pinned artifact,
    and CI validation expectations.
  - Provenance Requirements covering signing, pinning, SBOM, and
    deprecation expectations for installable plugins.
  - Sandbox Policy documenting `[runtime]`, `[auth]`, and `[cache]`
    manifest fields and their rules.
  - Status Taxonomy Reference table listing all six status values.
  - Overclaim Guards section with rules for describing planned plugins.
  - The original Manifest Rules and Test Expectations sections are
    preserved and re-ordered appropriately.
- **docs/src/plugin-registry.md** — cross-references the new authoring
  guide in the Categories section.
- **conductor/tracks/63-plugin-packaging-and-supply-chain-maturity/**
  — metadata.json status changed to "completed", plan.md documents
  completion, review.md created.
- **conductor/evidence-ledger.json** — track 63 evidence level promoted
  from "contracted" to "fixture-backed" with updated allowed_claims
  and blockers reflecting policy documents in place.
