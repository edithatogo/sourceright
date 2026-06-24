# Agent, Skill, and Workflow Rules

This repository should become self-improving through small, testable workflows,
not through undocumented agent sprawl.

## Add A Workflow When

- a repeated verification step is already documented;
- the step can fail deterministically;
- the workflow has a small path trigger;
- the workflow does not require secrets unless manually dispatched.

## Add A Skill Or Agent When

- the host package path is stable;
- the expected input/output contract is written in a Conductor track;
- there is a local smoke test or fixture;
- the skill/agent can improve a specific submission lane without mutating
  unrelated files.

## Do Not Add A Skill Or Agent When

- the host requirements are still unknown;
- the work is only a one-off external submission;
- the package path may not exist;
- it would encourage overclaiming local config as public acceptance.

## Current Decision

Tracks 73-81 produced hardened local packages and readiness-reviewed upstream
drafts. Track 82 wires the inventory, verifier, CI workflow, and policy tests
into a self-improving loop. **No new host-specific agent or skill is promoted
yet** — add one only when a specific surface needs automation beyond the
existing scripts and a stable host package contract is recorded in the inventory.

## Blockers

None.
