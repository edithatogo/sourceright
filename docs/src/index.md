# Sourceright

Sourceright is reference verification infrastructure for academic references first, with future tracks for legal citations and claim/source/provenance work.

The project is designed around a clean separation between canonical reference data and verification metadata:

- `references.csl.json` is the canonical academic-reference file.
- `references.verification.json` records providers, confidence, provenance, conflicts, and review state.
- `review-queue.jsonl` carries unresolved records into human or agent review.

The technical-preview implementation is Rust-first, exposed through a CLI and MCP server mode, with manifest-backed plugin discovery over the repository manifest set.

The canonical requirements and repo contract live in [Feature Contract Matrix](feature-contract-matrix.md). The architecture and Mermaid diagrams live in [Design](design.md). GitHub security automation and Copilot cloud-agent setup are documented in [Security Automation](security-automation.md).
