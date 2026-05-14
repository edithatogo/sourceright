# GitHub Automation And Alert Operations Test Matrix

| Scenario | Acceptance |
| --- | --- |
| Copilot setup | `.github/copilot-instructions.md`, Copilot issue template, and `copilot-setup-steps.yml` are present and the setup workflow passes. |
| Alert inventory | Dependabot and code-scanning alert checks are recorded with either current alert counts or explicit token/API limitation. |
| Installed apps | GitHub Apps/Marketplace inventory is recorded from API or settings screenshot/notes. |
| Renovate quietness | Routine updates remain grouped and scheduled; majors remain manual. |
| Notification posture | No scheduled issue spam or broad bot-assignment workflow is added. |
| Review loop | `$conductor-review` findings are fixed or deferred before the track advances. |
