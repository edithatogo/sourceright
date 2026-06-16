# Track 85 - Janeway Install and Smoke Evidence

## Goal

Prove a disposable-instance Janeway install and screening smoke, or record the blocker precisely, before any publication or support claim moves beyond the package contract.

## User outcome

Maintainers can point to an install transcript, an activation transcript, and a screening report transcript that show how Sourceright behaves inside Janeway when the environment is available.

## Scope

- Disposable-instance or local-container Janeway install smoke.
- Plugin activation or bridge registration evidence.
- Screening invocation and report retrieval evidence.
- Blocker documentation when the live path is unavailable.

## Out of scope

- Upstream acceptance or gallery-style publication claims.
- Generalized host support claims beyond the tested Janeway environment.
- Persistent environment mutation outside the smoke run.

## Data contracts

- Smoke transcript Markdown.
- Janeway install archive or package artifact.
- Janeway instance configuration or environment note.
- Report output from a Janeway-driven screening run.

## Claim boundary

> "Install-smoked" is not "Janeway-approved" and not "published".

The track may claim only that a Janeway disposable-instance smoke was recorded or that the live path is blocked with a precise reason.

## Evidence level target

**opt-in-live-proven** - the smoke requires an explicit live or disposable environment and does not run in default CI.

## Parallelization plan

- Install setup and report format decisions can be drafted in parallel.
- Smoke execution depends on the adapter package contract from Track 84.
- Blocker documentation can be written even when the live environment is unavailable.
