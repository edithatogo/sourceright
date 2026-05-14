# LibreOffice Extension Packaging Spec

## Goal

Define the LibreOffice integration contract for local, auditable citation
diagnostics in Writer documents.

## Contract

This track is complete only when a LibreOffice extension package or explicit
deferral exists with:

- `.oxt` packaging metadata and LibreOffice extension-site readiness notes if
  public distribution is pursued;
- UNO/Python or command-adapter wiring that calls the Rust core rather than
  reimplementing verification;
- document-range provenance, dry-run write plans, and audit logging; and
- local install, smoke, and uninstall checks using fixture documents.

## Claim Boundary

ODT/DOCX file processing and LibreOffice extension support are separate claims.
No Writer add-on claim is allowed until an installable package or documented
deferral exists.
