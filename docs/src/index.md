# Sourceright

Sourceright is reference verification infrastructure for academic references first, with future tracks for legal citations and claim/source/provenance work.

The project is designed around a clean separation between canonical reference data and verification metadata:

- `references.csl.json` is the canonical academic-reference file.
- `references.verification.json` records providers, confidence, provenance, conflicts, and review state.
- `review-queue.jsonl` carries unresolved records into human or agent review.

The production implementation is Rust-first, exposed through a CLI and MCP server mode, with runtime plugin discovery over the repository manifest set.
