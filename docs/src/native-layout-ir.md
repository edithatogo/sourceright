# Native layout IR

Sourceright exposes a backend-neutral page, block, token, box, provenance, and
diagnostic contract in the Rust core. Tokens retain page-relative geometry and
source IDs; reading order is deterministic and ambiguity is reported.

The current checked-in adapter is a self-authored text fixture adapter used to
validate the contract. It is not a PDF parser and does not claim OCR, native
PDF coverage, or universal layout accuracy. A PDF backend may be selected only
after Track 92 benchmark evidence and license, security, malformed-input,
resource-limit, and cross-platform checks.

Empty/no-text input emits `ocr_required` and produces no synthetic text. Input,
page, and token limits are enforced before unbounded processing.
