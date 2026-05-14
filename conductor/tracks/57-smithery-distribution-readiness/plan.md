# Smithery Distribution Readiness Plan

1. Verify current Smithery publishing requirements.
2. Choose Streamable HTTP or MCPB/local packaging as the first supported path.
3. Add required metadata/configuration files.
4. Add local validation and install smoke.
5. Update publishing and release-status docs.
6. Run `$conductor-review`.
7. Apply local fixes automatically; defer publication until public listing evidence exists.

## Progress Notes

- 2026-05-14: Current Smithery docs verified. Smithery supports URL publishing
  for Streamable HTTP MCP servers and `.mcpb` bundle publishing for local stdio
  servers. Sourceright chose the MCPB/local path because the implemented MCP
  runtime is stdio.
- 2026-05-14: Added `smithery/mcpb/manifest.template.json`,
  `scripts/build-smithery-mcpb.ps1`, and `tests/smithery_distribution_policy.rs`.
  The track is complete as a prepared MCPB distribution path, not as an
  accepted Smithery listing.
