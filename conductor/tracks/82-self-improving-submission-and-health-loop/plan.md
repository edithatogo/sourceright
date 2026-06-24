# Plan

1. [x] Create machine-readable submission requirements inventory.
2. [x] Add submission-readiness verifier script.
3. [x] Add CI workflow for submission readiness changes.
4. [x] Add policy tests that bind the inventory, workflow, and repo-health
   target to the submission contracts.
5. [x] Harden the Windows GNU verifier so native command failures stop the
   script.
6. [x] Add local submission packet drafts for MCP directories, citation
   managers, journal platforms, arXiv upstream repositories, AI clients, and
   VS Code/Open VSX.
7. [x] Add a machine-readable submission packet manifest and readiness checks
   for packet coverage, validation gates, blockers, and approval boundaries.
8. [x] Promote host-specific agents, skills, or workflows only after their
   package path has a stable requirements contract (documented in
   `agent-workflow.md`; no new agents promoted).
9. [x] Keep repo health at least 9.5 before any external submission claim changes
   (verified in `health-loop-2026-06-09.md` and inventory `repo_health_target`).
