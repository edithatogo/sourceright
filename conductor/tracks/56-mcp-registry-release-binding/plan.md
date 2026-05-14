# MCP Registry Release Binding Plan

1. Validate `server.json` schema, package name, image reference, and version.
2. Validate release workflow artifacts and OCI labels before registry submission.
3. Validate official MCP Registry listing after publication.
4. Keep Glama metadata aligned but separate from accepted MCP Registry evidence.
5. Update release-status docs and run `$conductor-review`.
6. Apply local fixes automatically; defer external publication if listing evidence is unavailable.
