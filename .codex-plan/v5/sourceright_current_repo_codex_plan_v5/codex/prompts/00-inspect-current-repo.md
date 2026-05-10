The Sourceright repo has changed substantially since an earlier overlay plan. Treat the current repository as the source of truth.

There may be an older overlay ZIP or unpacked overlay nearby. Do not bulk-apply it. Use it only as design reference if needed.

Task: inspect only. Do not modify files.

Please inspect:
- Cargo.toml
- README.md
- conductor/tracks.md
- docs/
- src/
- .github/

Then produce a slice-by-slice plan that answers:
1. What is already implemented?
2. What is implemented but under-documented?
3. What is documented but not implemented?
4. What gaps remain in these areas:
   - JSON/YAML schemas
   - plugin manifest and registry
   - benchmark harness
   - Streamlit/GitHub Pages demos
   - recency/versioning checks
   - style/policy checks
   - citation-manager profiles/adapters
   - real read-only MCP server
   - provider expansion with fixture-backed tests
5. Which files should be touched first?
6. Which changes should be deferred?

Important constraints:
- Do not split the Rust crate yet.
- Do not overwrite existing modules.
- Do not add live network tests.
- Keep provider evidence in the sidecar.
- Keep legal citations separate from CSL.
- Do not add claim-truth scoring.
