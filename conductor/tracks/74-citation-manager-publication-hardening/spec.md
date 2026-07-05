# Citation-Manager Publication Hardening Spec

## Goal

Harden Zotero and EndNote publication paths with package decisions, smoke proof,
import/export checks, and exact plugin-claim boundaries so that each citation-manager
integration has a verified distribution channel, passing import/export roundtrip,
and recorded listing or deferral evidence.

## User outcome

A reviewer or operator can confirm for each citation manager:
- **Zotero**: Package decision (`.xpi` or CLI/Web API binary) is documented and
  consistent with the distribution notes. Import/export roundtrip via RIS/ENW
  passes. Install smoke is recorded. Marketplace listing status is explicit
  (accepted, prepared, deferred, or not-applicable).
- **EndNote**: ENW handoff validation passes. RIS export/import roundtrip passes.
  Distribution notes separate shareable package from official acceptance.
- **RIS/ENW export suite**: All import/export roundtrip scenarios pass for Zotero
  and EndNote profiles, with recorded evidence per format.

## Scope

- **Zotero package hardening**: Confirm the packaging decision (CLI/Web API
  binary, not `.xpi`) is locked in `packaging-decision.md` or equivalent. Verify
  install smoke proof (`sourceright citation-sync --preview` with remote fixture)
  passes. Document distribution channel (GitHub Releases / crates.io) and
  marketplace listing status (not-applicable for `.xpi` Plugin Gallery).
- **Zotero `.xpi` decision revalidation**: Revisit the deferred `.xpi` decision
  from Track 58 to confirm it still holds. If demand or requirement has changed,
  produce a `.xpi` build script and install smoke; otherwise record a revalidation
  finding.
- **EndNote ENW handoff validation**: Verify that `sourceright export --format enw`
  produces valid EndNote-tagged records that roundtrip through a parser. Record
  structural validation evidence.
- **RIS import/export smoke**: Verify that `sourceright export --format ris` and
  a companion RIS import path (or at least a reparse validation) produce identical
  bibliographic records for representative fixtures. Record roundtrip evidence.
- **Import/export test automation**: Add or verify roundtrip tests for RIS, ENW,
  and BibLaTeX exports that parse the generated output and compare record count
  and key identifiers against the source CSL JSON.
- **Distribution notes for each citation manager**: For Zotero and EndNote,
  document the shareable package (binary, file export) separately from official
  marketplace acceptance. Update `docs/src/citation-manager-integration.md` and
  `docs-site/src/content/docs/guides/citation-manager.md` accordingly.
- **Marketplace listing probe**: For each citation manager, record the listing
  status (accepted listing URL, prepared-but-not-accepted, or
  not-applicable) in a format consistent with Track 69's evidence model.

## Out of scope

- Mendeley, Paperpile, JabRef, RefWorks, Citavi, and Rayyan live sync (owned by
  Track 59 or future tracks).
- OJS/PKP publication hardening (owned by Track 75).
- Revision of the core citation-sync engine (`src/citation_sync.rs`) — only
  packaging, distribution, and import/export validation is in scope.
- Zotero API adapter changes — only packaging decision and install smoke
  evidence is gathered, not API contract changes.
- New citation-manager adapters beyond Zotero and EndNote.

## Data contracts

| Contract | Source | Format |
|---|---|---|
| CSL canonical reference model | `references.csl.json` | CSL-JSON array |
| Export RIS format | `sourceright export --format ris` | RIS (`TY  -`/`ER  -` blocks) |
| Export ENW format | `sourceright export --format enw` | EndNote tagged records |
| Export BibLaTeX format | `sourceright export --format biblatex` | BibLaTeX `.bib` entries |
| Packaging decision | Track 58 `packaging-decision.md` | Markdown decision record |
| Marketplace evidence | Track 69 `marketplace-evidence.md` | JSON array of `{surface, url, status, version, date}` |
| Submission requirements | Track 72 submission-contracts | Per-host requirement tables |

## Claim boundary

**"Package-ready" not "marketplace-accepted".** A citation-manager integration
may be described as "package-ready" when:
- The packaging decision is documented and consistent with distribution notes.
- Import/export roundtrip passes for all supported formats (RIS, ENW, BibLaTeX).
- Install smoke proof is recorded (fixture-backed or local).
- Marketplace listing status is explicitly recorded as accepted, prepared,
  deferred, or not-applicable.

No integration may be described as "Zotero-accepted", "EndNote-accepted", or
"marketplace-accepted" without a public listing URL, version, date, and install
metadata in the marketplace evidence ledger.

## Evidence level target

`default-CI` for import/export roundtrip tests and structural validation.
`opt-in-live` for marketplace listing probes and distribution-channel
verification.

## Parallelization plan

- **Subagent A**: Zotero packaging decision audit and `.xpi` revalidation.
  Reviews `packaging-decision.md`, checks install smoke evidence, records
  revalidation finding.
- **Subagent B**: EndNote ENW handoff validation. Runs ENW export against
  representative fixtures, structural validation, roundtrip parser check.
- **Subagent C**: RIS import/export roundtrip. Runs RIS export against
  representative fixtures, structural validation, roundtrip parser check.
- **Subagent D**: Distribution notes and marketplace listing probes. Updates
  `docs/src/citation-manager-integration.md` and docs-site guide with per-manager
  package and listing status. Records listing probe results.

Subagents B and C (format roundtrips) can run in parallel after a shared
fixture selection. Subagent A must complete before Subagent D (distribution
notes depend on packaging decision). Subagent D runs last.
