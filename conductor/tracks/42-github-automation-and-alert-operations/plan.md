# GitHub Automation And Alert Operations Plan

1. Discover current GitHub-side capabilities.
   - Check Copilot coding-agent availability.
   - Check installed apps and Marketplace integrations.
   - Record unavailable API scopes rather than guessing.
2. Audit alert state.
   - Query Dependabot alerts when token scopes allow it.
   - Query code-scanning alerts when token scopes allow it.
   - Compare GitHub state to local `cargo audit`, `npm audit`, CodeQL, and
     Scorecard evidence.
3. Review automation policy.
   - Confirm Renovate grouping, scheduling, automerge constraints, and labels.
   - Confirm Dependabot does not create duplicate noisy update PRs.
   - Confirm Copilot issue template and setup workflow are still valid.
4. Implement local fixes only.
   - Update workflows, Renovate config, Copilot instructions, or docs when the
     repo-local contract is wrong.
5. Validate.
   - Run affected workflow syntax checks where practical.
   - Run docs parity and security automation policy tests when changed.
   - Push and verify `CI`, `Security`, `Pages`, and `Copilot Setup Steps`.
6. Run `$conductor-review`.
   - Apply local review fixes automatically.
   - Defer account-side GitHub settings with exact UI/API evidence required.
