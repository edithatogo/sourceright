# Track 59 — Other Citation Manager Integrations: Review

## Current State

### Documentation Review

**Spec** requirements:
- EndNote support starts as ENW/RIS handoff and reference-checking proof
- Other managers get either an adapter plan, export/import proof, or explicit deferral
- No manager is called supported without fixture-backed import/export or live smoke evidence

**Test Matrix**:
| Scenario | Acceptance |
|----------|-----------|
| EndNote handoff | ENW/RIS output is generated, reparsed, and documented |
| Manager decision | Each target manager has support/defer rationale |
| Import/export proof | Supported managers have fixture-backed proof |
| Claim boundary | Docs do not imply live sync where only file handoff exists |

### Codebase Evidence (Updated)

| Component | Zotero | EndNote | Mendeley | Paperpile | JabRef | RefWorks | Citavi | Papers/RC |
|-----------|--------|---------|----------|-----------|--------|----------|--------|-----------|
| Plugin manifest | ✅ | ✅ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ |
| Registry entry | ✅ | ✅ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ |
| Profile YAML | ✅ | ✅ | ✅ | ❌ | ✅ | ❌ | ❌ | ✅ |
| Rust engine | ✅ | ✅ ENW/RIS | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ |
| Unit tests | ✅ | ✅ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ |
| Integration tests | ✅ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ |
| Fixtures | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ |

### Key Findings

1. **Only Zotero and EndNote have plugin manifests and registry entries.** The other 6 managers (Mendeley, Paperpile, JabRef, RefWorks, Citavi, Papers/ReadCube) have no manifests, no engine code, and no tests.

2. **Profile YAMLs exist for 5 of 8 managers** under `examples/citation-manager-profiles/`:
   - Zotero (`api_and_file`, `dry_run` default)
   - EndNote (`file` family, `enw/ris/xml`)
   - Mendeley (`api_and_file`, needs OAuth 2.0)
   - JabRef (`file` family, `biblatex/ris`)
   - Papers/ReadCube (`file` family, `ris/biblatex`)
   - Plus systematic review tools: RevMan, Rayyan, Covidence (all `file` family, `ris` only)
   - Missing profiles: Paperpile, RefWorks, Citavi

3. **ENW/RIS export is mature.** `export.rs` has fully functional `export_ris()` and `export_enw()` generators with correct tag mappings and type conversions. 3 unit tests verify output structure.

4. **No fixture-backed tests exist** for any citation manager. This is the critical gap preventing any manager from being called "supported."

5. **Decision table created** at `decision-table.md` with explicit support/defer decisions for all 8 target managers + 3 systematic review tools.

### Gap Analysis

| Gap | Impact |
|-----|--------|
| No ENW/RIS reparse tests | Cannot prove round-trip integrity for EndNote handoff |
| No Mendeley adapter code | Mendeley profile exists but no engine — requires OAuth 2.0 token management |
| No Paperpile/RefWorks/Citavi profiles | These managers lack even profile YAML definitions |
| No fixture directories | Zero fixture-backed proof for any manager |
| Missing plugin manifests (6 managers) | Only Zotero and EndNote have manifests |

## Recommendations

1. **Add EndNote ENW/RIS fixture** with export and reparse test to close the highest-priority gap.

2. **Create `provider-fixtures/endnote/`** with:
   - `handoff/sample.enw` — expected ENW output for a known CSL document
   - `handoff/sample.ris` — expected RIS output for a known CSL document
   - `handoff/reparse-expected.json` — expected CSL after reparse

3. **Maintain deferral decisions** for all non-Zotero/non-EndNote managers. Document in the decision table that:
   - Mendeley is the next candidate for adapter work (public API, existing profile, but OAuth 2.0 complexity)
   - JabRef is covered by existing BibLaTeX export
   - Paperpile, RefWorks, Citavi, Papers/ReadCube are file-handoff only
   - RevMan, Rayyan, Covidence are RIS-handoff only with no adapter work planned

4. **Create plugin manifests** for Mendeley and JabRef if implementation starts. For now, keep deferred managers manifest-free.

5. **Update docs** to explicitly call out which managers have implementation vs. which are deferred.

## Status

- **Previous status**: planned
- **Current status**: planned (decision table created ✅, EndNote ENW/RIS export exists ✅, all other managers deferred — fixtures needed before advancing)
