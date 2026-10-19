---
name: sourceright
description: Read-only scholarly citation reconciliation and provenance review using the SourceRight CLI, with explicit parsing and scientific verification limits.
---

# Scholarly citation review

Use this skill for read-only reconciliation of manuscript citations with a supplied CSL bibliography. Read README.md and inspect the actual CLI help before choosing other capabilities. Record the source commit, local patch version, executable path/hash and input hashes separately; a binary on PATH is not proof it matches this checkout.

Run `sourceright citations MANUSCRIPT WORKSPACE_ROOT` with the workspace bibliography (inspect CLI help for actual supported options). DOCX intake reads main document paragraphs including table paragraphs, normalizes directly formatted numeric superscripts, and joins adjacent superscript runs. Bracketed comma groups and hyphen/en-dash/em-dash inclusive ranges are expanded. Each expanded marker retains the original group and UTF-8 byte span in extracted text, not Word page coordinates. Positive integers only; groups are limited to 4096 bytes and 1000 expanded entries. Malformed numeric groups are reported for manual review, not partly accepted. First-use ordering ignores legitimate repeated citations. Repeated citations are not duplicate bibliography entries.

This is heuristic intake, not a complete Word layout, field or style resolver. Style-inherited superscripts, equation exponents, custom fields, notes, and tracked revisions require explicit manual adjudication. Use a verified clean derivative and preserve extraction evidence. Do not mistake numeric exponents for citations. Do not suppress warnings silently; document limitations and independently compare extracted markers against the source document.

Keep canonical `references.csl.json`, provider verification evidence and derived review queues separate. Bibliographic identity matches do not verify metadata, source existence, claim support or scientific correctness. Literal bibliography indices can support a declared syntax-only check but must not be promoted into verified canonical references. Claim review requires primary-source passages and records of access limitations.

Do not write back to manuscripts, overwrite canonical CSL, invoke provider mutation or transmit unpublished content without task authorization. Hand findings to the repository's contract-controlled writer and independent verifier. Readability and prose scores do not establish authorship, truth or submission readiness.
