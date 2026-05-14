# Branch Inventory

Inventory refreshed on 2026-05-14 from read-only local git and GitHub CLI
queries. No branch was merged, deleted, reset, checked out, or force-updated.

## Commands Run

| Command | Purpose | Result |
| --- | --- | --- |
| `git status --short --branch` | Confirm current branch and dirty worktree before inventory. | On `main` tracking `origin/main`; worktree already contained unrelated modified and untracked files. |
| `git branch -a --no-merged main` | List local and remote branch tips that are not ancestors of `main`. | Seven stale local branches and seven matching remote branches remain non-ancestor tips. |
| `git branch -vv -a` | Capture branch heads, upstreams, and last commits. | Each stale local branch tracks the matching `origin/*` branch at the same commit. |
| `git log --oneline main..<branch>` | Identify commits reachable from stale branch tips but not from `main`. | One branch-tip commit per stale branch. |
| `git cherry -v main <branch>` | Test patch equivalence against `main`. | Each stale branch commit is reported with `-`, meaning the patch already exists on `main`. |
| `git diff --stat main...<branch>` | Record changed surfaces represented by each stale branch. | Diffstats map cleanly to current completed tracks and merged PR titles. |
| `git log --oneline --decorate --grep "#6\\|#7\\|#8\\|#9\\|#10\\|#11\\|#12" main` | Verify mainline PR merge commits for branch heads. | PRs #6 through #12 are present on `main`. |
| `git log --oneline --decorate --all --simplify-by-decoration --max-count=80` | Confirm decorated branch and tag topology. | `main` is at `47d7f38`; stale branches remain at pre-release feature tips. |
| `git stash list` | Check whether cleanup-worthy stash entries currently exist. | No stash entries were present. |
| `gh pr list --state all --limit 50 --json number,title,headRefName,baseRefName,state,mergedAt,mergeCommit,url` | Verify GitHub PR state for matching branch names. | Matching PRs #6-#12 are all `MERGED`. |

## Summary

| # | Branch | Type | Last commit | PR evidence | Current classification |
| --- | --- | --- | --- | --- | --- |
| 1 | `main` | default local + remote | `47d7f38` | Active default branch | Source of truth |
| 2 | `add-journal-workflow-integrations-track` | local + remote | `fa25a51` | PR #11 merged as `b556d50` | Superseded by merged mainline work |
| 3 | `continue-in-progress-tracks` | local + remote | `5425ed4` | PR #7 merged as `92c67c6` | Superseded by merged mainline work |
| 4 | `finish-active-reference-surfaces` | local + remote | `ce87a6a` | PR #9 merged as `2396fe6` | Superseded by merged mainline work |
| 5 | `implement-active-tracks-next` | local + remote | `678c8a1` | PR #8 merged as `0cdffd0` | Superseded by merged mainline work |
| 6 | `implement-intake-providers-cleaning-exports` | local + remote | `abeae3a` | PR #10 merged as `bd2c5f7` | Superseded by merged mainline work |
| 7 | `require-explicit-export-output` | local + remote | `a08ca93` | PR #12 merged as `dacc329` | Superseded by merged mainline work |
| 8 | `switch-to-renovate` | local + remote | `6316691` | PR #6 merged as `821f84e` | Superseded by merged mainline work |

The stale feature branches are not ancestors of `main`, so they still appear in
`git branch -a --no-merged main`. They are nevertheless patch-equivalent to
mainline commits: `git cherry -v main <branch>` reports `-` for each branch-tip
commit. This is consistent with PR merge history where the branch tip itself
does not remain an ancestor of `main`.

## Branch Details

### `main`

- **Last commit**: `47d7f38 Configure Copilot security automation`
- **Remote**: `origin/main`, `origin/HEAD`
- **Current role**: Source of truth for active work.
- **Validation note**: The worktree is dirty from unrelated edits, so this track
  did not run destructive cleanup or integration operations.

### `add-journal-workflow-integrations-track`

- **Last branch commit**: `fa25a51 Add journal workflow integrations track`
- **Remote**: `origin/add-journal-workflow-integrations-track`
- **PR evidence**: PR #11, `Add journal workflow integrations track`, merged
  into `main` at `b556d50` on 2026-05-09.
- **Patch evidence**: `git cherry -v main add-journal-workflow-integrations-track`
  reports `- fa25a51...`, indicating equivalent patch content is already on
  `main`.
- **Intent mapped to current tracks**: Track 16, Journal workflow integrations.
- **Decision**: Superseded branch; no merge candidate.

### `continue-in-progress-tracks`

- **Last branch commit**: `5425ed4 Advance in-progress Conductor tracks`
- **Remote**: `origin/continue-in-progress-tracks`
- **PR evidence**: PR #7, `Advance in-progress Conductor tracks`, merged into
  `main` at `92c67c6` on 2026-05-09.
- **Patch evidence**: `git cherry -v main continue-in-progress-tracks` reports
  `- 5425ed4...`.
- **Intent mapped to current tracks**: Tracks 04, 05, 12, and 15.
- **Decision**: Superseded branch; no merge candidate.

### `finish-active-reference-surfaces`

- **Last branch commit**: `ce87a6a Finish active reference surfaces`
- **Remote**: `origin/finish-active-reference-surfaces`
- **PR evidence**: PR #9, `Finish active reference surfaces`, merged into
  `main` at `2396fe6` on 2026-05-09.
- **Patch evidence**: `git cherry -v main finish-active-reference-surfaces`
  reports `- ce87a6a...`.
- **Intent mapped to current tracks**: Tracks 04, 05, 12, and 15.
- **Decision**: Superseded branch; no merge candidate.

### `implement-active-tracks-next`

- **Last branch commit**: `678c8a1 Implement next active Conductor track slices`
- **Remote**: `origin/implement-active-tracks-next`
- **PR evidence**: PR #8, `Implement next active Conductor track slices`, merged
  into `main` at `0cdffd0` on 2026-05-09.
- **Patch evidence**: `git cherry -v main implement-active-tracks-next` reports
  `- 678c8a1...`.
- **Intent mapped to current tracks**: Tracks 04, 05, 12, and 15.
- **Decision**: Superseded branch; no merge candidate.

### `implement-intake-providers-cleaning-exports`

- **Last branch commit**: `abeae3a Implement intake providers cleaning and exports`
- **Remote**: `origin/implement-intake-providers-cleaning-exports`
- **PR evidence**: PR #10, `Implement intake, providers, cleaning, and exports`,
  merged into `main` at `bd2c5f7` on 2026-05-09.
- **Patch evidence**:
  `git cherry -v main implement-intake-providers-cleaning-exports` reports
  `- abeae3a...`.
- **Intent mapped to current tracks**: Tracks 03, 06, 07, and 11.
- **Decision**: Superseded branch; no merge candidate.

### `require-explicit-export-output`

- **Last branch commit**: `a08ca93 Require explicit export output selection`
- **Remote**: `origin/require-explicit-export-output`
- **PR evidence**: PR #12, `Require explicit export output selection`, merged
  into `main` at `dacc329` on 2026-05-10.
- **Patch evidence**: `git cherry -v main require-explicit-export-output`
  reports `- a08ca93...`.
- **Intent mapped to current tracks**: Track 11, Export suite.
- **Decision**: Superseded branch; no merge candidate.

### `switch-to-renovate`

- **Last branch commit**: `6316691 Switch dependency updates to Renovate`
- **Remote**: `origin/switch-to-renovate`
- **PR evidence**: PR #6, `Switch dependency updates to Renovate`, merged into
  `main` at `821f84e` on 2026-05-09.
- **Patch evidence**: `git cherry -v main switch-to-renovate` reports
  `- 6316691...`.
- **Intent mapped to current tracks**: Tracks 26, 41, and 42.
- **Decision**: Superseded branch; no merge candidate.

## Stash State

`git stash list` returned no entries during this inventory. Earlier references
to stash cleanup are stale for the current working copy. No stash operation is
recommended unless a later inventory shows actual stash entries.

## Findings

- There are no current merge candidates.
- There is no partial salvage candidate in the inventoried stale branch set.
- The stale branches can be considered closure candidates because their patch
  content is already represented on `main` through merged PRs #6-#12.
- The local and remote branch refs should remain untouched until the user gives
  explicit approval for branch cleanup.

## Deferred Operations Requiring Explicit Approval

The following operations are intentionally not performed by this track:

| Operation | Required explicit approval | Minimum preflight before execution |
| --- | --- | --- |
| Delete local stale branches | User names the local branches and approves `git branch -d`. | Re-run `git branch -a --no-merged main`, `git cherry -v main <branch>`, and `git status --short --branch`. |
| Delete remote stale branches | User names the remote branches and approves `git push origin --delete <branch>`. | Re-run `gh pr list --state all` for matching heads and verify each PR remains merged. |
| Merge any branch | User names the branch, target, and permits a temporary integration branch. | Create a temporary integration branch, merge without force, run targeted checks, and record results before any final merge request. |
| Force-update or reset branch refs | Separate explicit approval naming the branch ref and exact operation. | Document why non-force alternatives are insufficient; do not proceed from this track by default. |
| Stash cleanup | User approves `git stash drop` or `git stash clear` after a fresh `git stash list` shows entries. | Capture stash list and confirm no pending work depends on those entries. |
