# Branch Triage And Stale-Work Closure Plan

1. Run branch inventory.
   - `git branch -a --no-merged main`
   - `gh pr list --state all` for matching branches when available.
2. Assign branches to subagents.
   - Inspect branch diff stats.
   - Map branch intent to current tracks and requirements.
   - Identify conflicts, obsolete work, or salvageable commits.
3. Create a branch decision table.
4. For merge candidates, run targeted checks on a temporary integration branch.
5. For superseded branches, record the track or commit that superseded them.
6. Run `$conductor-review` on the triage evidence.
7. Apply local documentation fixes automatically; require explicit user approval
   for merges, branch deletion, or force operations.
