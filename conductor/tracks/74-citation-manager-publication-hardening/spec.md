# Track 74: Citation-Manager Publication Hardening

## Goal

Move Zotero and EndNote from prepared/contracted adapter surfaces toward mature
publication submissions with explicit package, install, smoke, and listing
evidence.

## User Outcome

Users can tell whether Zotero is a CLI/Web API adapter, `.xpi`, or both; and
whether EndNote is an ENW/RIS handoff or a real plugin. Public claims match that
evidence exactly.

## Scope

- Zotero package decision, disposable-library smoke, preview/apply/audit proof,
  and public distribution path.
- EndNote ENW/RIS handoff hardening, reparse checks, and explicit no-plugin
  boundary unless a real plugin package is created.
- Submission templates for release notes, forum/listing posts, and accepted
  evidence updates.

## Out Of Scope

- Silent writes to user citation libraries.
- Claiming Zotero or EndNote marketplace acceptance without accepted evidence.

## Data Contracts

Zotero writes require preview/apply/audit semantics. EndNote remains file-based
unless a separate plugin contract is created.

## Claim Boundary

Zotero and EndNote claims must distinguish adapter, file export, `.xpi`, and
plugin listing states.

## Evidence Level Target

Hardened local package and submitted listing where an appropriate public
distribution path exists.

## Parallelization Plan

Zotero and EndNote lanes can run in parallel after shared citation-manager
export/import fixtures are frozen.

## Maturity, Stability, And Testing

Maturity requires fixture reparse checks, disposable-library smoke where
credentials exist, install docs, rollback instructions, and policy tests that
prevent plugin overclaims.
