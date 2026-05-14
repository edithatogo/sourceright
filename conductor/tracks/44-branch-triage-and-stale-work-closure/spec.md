# Branch Triage And Stale-Work Closure Spec

## Goal

Prevent old unmerged branches from hiding planned work, stale implementation
ideas, or conflicts with the current source-of-truth architecture.

## Scope

- Inventory local and remote branches not merged into `main`.
- Compare each branch to current `main`, Conductor tracks, requirements, and
  design contract.
- Classify each branch as merge candidate, superseded, partial salvage,
  archive/delete candidate, or needs human decision.
- Do not merge or delete without explicit evidence and normal review.

## Parallelization

Each branch can be assigned to a separate explorer subagent. Workers may only
edit files for a branch if the lead assigns a non-overlapping salvage slice.
