# CLI Entrypoint Integration Plan

1. Audit the current CLI surface.
   - Confirm how `bench` and `citation-sync` are exposed today.
   - Identify any help text, usage text, or docs that still omit the new entrypoints.

2. Wire the commands into the main CLI surface.
   - Ensure the commands are discoverable from the primary `sourceright` entrypoint.
   - Keep argument parsing and help output consistent with existing command patterns.

3. Align documentation and examples.
   - Update CLI docs and command references.
   - Add concise examples for the new entrypoints where appropriate.

4. Run a consistency pass.
   - Check for stale command names, broken references, or duplicated guidance.
   - Confirm the new surfaces do not imply live network use by default.

5. Validate.
   - Run the standard Cargo gates.
   - Recheck the command/help/docs surfaces after the edits.
