# Microsoft Word Add-In Packaging Spec

## Goal

Define the Microsoft Word integration contract for citation audit and
enrichment inside documents.

## Contract

This track is complete only when a Word add-in package or explicit deferral
exists with:

- Office Add-in manifest, taskpane/runtime, sideload instructions, and AppSource
  readiness notes if public distribution is pursued;
- document-range provenance that maps Sourceright diagnostics back to Word
  ranges without losing CSL/sidecar boundaries;
- dry-run-first editing, reversible change plans, and audit logs for any
  in-document mutation; and
- fixture documents, privacy notes, and install smoke coverage.

## Claim Boundary

DOCX extraction support is not Word add-in support. No in-document editing claim
is allowed until reversible write plans and user-visible apply semantics exist.
