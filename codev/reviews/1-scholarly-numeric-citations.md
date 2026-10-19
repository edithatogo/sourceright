# Scholarly numeric citation diagnostics review

Implemented bounded comma/range groups, source byte spans, atomic malformed-group diagnostics, first-use numeric ordering, legitimate repeat handling and contiguous DOCX superscript run normalization. Added maintained read-only scholarly SKILL.md. Existing public duplicate-citation enum is retained for compatibility but repeated references no longer generate that finding.

Regression work: the initial library run with new tests produced four failures (group extraction, malformed diagnostic, first-use repeats, prior duplicate expectation). After fixes the library passed. Independent review subsequently identified nested/letter/semicolon/unclosed group holes; balanced scanning and regression cases repaired these. Ordinary prose brackets are covered as a negative control. DOCX regression constructs a real ZIP document containing split superscript runs and checks extracted numeric keys.

Validation: regression tests cover the final balanced scanner and DOCX split-run handling. The upstream integration uses the published CiteWeft Git revision; standalone checks and GitHub Actions results are recorded in the pull request.

Limits: spans are in extracted UTF-8 text, not Word coordinates. Direct formatting only; no complete style, field or tracked-revision resolution. Numeric exponents may resemble citations and require manual inspection. Source matching never establishes claim truth. No manuscript, canonical CSL or remote data was changed.
