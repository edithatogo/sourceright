# Track 75: Journal-Platform Publication Hardening

## Goal

Move OJS/PKP and journal-platform integrations from fixture-backed local
skeletons toward mature packages and accepted submission evidence.

## User Outcome

Editors and maintainers can see the OJS package, compatibility matrix,
permissions, smoke proof, Gallery submission status, and how arXiv upstream work
is separated from OJS packaging.

## Scope

- OJS/PKP generic plugin package hardening and PKP Plugin Gallery readiness.
- Shared journal-screening contract stability.
- Cross-links to granular arXiv upstream tracks.

## Out Of Scope

- Claiming PKP Gallery acceptance before accepted listing evidence.
- Treating arXiv upstream modules as OJS packages.

## Data Contracts

Journal platforms emit `sourceright.journal_screening.v1` outputs and never
write canonical CSL or external platform state by default.

## Claim Boundary

OJS may be called package-ready only after package build and install smoke pass;
Gallery acceptance requires external listing evidence.

## Evidence Level Target

Hardened local package, submitted, then publicly accepted for PKP/OJS.

## Parallelization Plan

OJS package hardening can proceed in parallel with arXiv tracks because arXiv
owns separate manifests, fixtures, and upstream repo contracts.

## Maturity, Stability, And Testing

Maturity requires OJS version compatibility, settings and permissions docs,
package validation, fixture smoke, optional live test-instance smoke, and
maintainer-facing submission notes.
