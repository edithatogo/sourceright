# Track 73: MCP Directory Submission Hardening

## Goal

Take MCP directory surfaces from prepared/accepted mix to mature, refreshable
submission evidence for Official MCP Registry, Smithery, and Glama.

## User Outcome

The MCP release path has versioned registry evidence, repeatable package
validation, and clear issue/listing records for every MCP directory.

## Scope

- Official MCP Registry release refresh contract.
- Smithery `.mcpb` bundle build, install smoke, and listing submission plan.
- Glama metadata verification and listing/API evidence plan.
- Registry proof records with URL, version, date, artifact id, and install
  metadata.

## Out Of Scope

- Implementing a new HTTP MCP transport.
- Submitting to Smithery or Glama without approval.

## Data Contracts

`server.json`, `glama.json`, Smithery MCPB manifest, release artifacts, and
registry evidence rows must remain version-aligned.

## Claim Boundary

Official MCP Registry may remain accepted only for verified versions. Smithery
and Glama stay prepared until external accepted listing evidence is recorded.

## Evidence Level Target

Hardened local package, then submitted, then publicly accepted per directory.

## Parallelization Plan

Official MCP refresh, Smithery bundle smoke, and Glama metadata verification can
run in parallel after shared release version data is frozen.

## Maturity, Stability, And Testing

Maturity requires default schema checks, local bundle smoke, stdio transcript
smoke, release-surface refresh, and opt-in live directory verification before
submission.
