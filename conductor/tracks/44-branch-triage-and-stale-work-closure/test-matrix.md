# Branch Triage And Stale-Work Closure Test Matrix

| Scenario | Acceptance |
| --- | --- |
| Branch inventory | Every `git branch -a --no-merged main` local and remote branch is listed with last commit, PR evidence, and mapped intent. |
| Superseded branch | Current mainline evidence, patch-equivalence evidence, and mapped Conductor track are named. |
| Merge candidate | No merge candidate is recommended unless patch equivalence fails; any future candidate requires a temporary integration branch and targeted checks before merge recommendation. |
| Human decision | Ambiguous or destructive branch operations list the exact unresolved question and explicit approval gate. |
| No destructive action | No branch deletion, reset, checkout, merge, stash cleanup, or force push occurs without explicit approval. |
| Documentation validation | Track 44 markdown uses consistent tables, current command evidence, and no stale stash claim. |
