# Track 86 - Proprietary Journal Platform Matrix

## Goal

Map the integration surfaces, access model, and test environment expectations for proprietary journal platforms so Sourceright can be positioned as a bounded adapter set rather than an overclaimed universal integration.

## User outcome

Maintainers can see which vendor platforms are realistic for Sourceright, what evidence is needed for each, and which integrations should stay as contract-only until a test environment or API access exists.

## Scope

- Survey official developer portals and public integration pages for ScholarOne, Editorial Manager, eJournalPress, and Manuscript Manager.
- Classify each platform by API surface, webhook/batch support, plugin support, and realistic test access.
- Define a vendor-adapter matrix that separates read-only discovery from live integration claims.
- Capture the first-pass test environment requirements for each platform.

## Out of scope

- Live vendor integrations without documented access.
- Vendor certification claims.
- Subscription or procurement decisions.
- Any claim that a platform is supported unless evidence exists.

## Data contracts

- `conductor/tracks/86-proprietary-journal-platform-matrix/test-matrix.md` for the capability matrix.
- `references.verification.json` and track notes for evidence provenance if platform-specific citations are added later.
- Vendor documentation or public developer portals as the evidence sources.

## Claim boundary

> This track is a platform matrix, not a support announcement.

All entries must distinguish documented capability from inferred integration potential.

## Evidence level target

**contracted** - the track is done when the vendor matrix, access assumptions, and evidence boundaries are explicit and reviewable.

## Parallelization plan

- Platform discovery can run in parallel per vendor.
- The matrix and claim boundary should be locked before any implementation work.
- Test environment assumptions should be written as blockers when the vendor does not publish a usable sandbox.
