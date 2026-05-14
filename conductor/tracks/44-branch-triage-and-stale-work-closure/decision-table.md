# Branch Decision Table

Updated on 2026-05-14. This table is documentation only; no branch operation was
performed.

| Branch | Intent mapped to current tracks | Evidence from inventory | Decision | Approval gate |
| --- | --- | --- | --- | --- |
| `add-journal-workflow-integrations-track` | Track 16 | PR #11 merged at `b556d50`; `git cherry -v main` reports `- fa25a51...`. | Superseded; no merge candidate. | Delete only after explicit approval for local and/or remote branch deletion. |
| `continue-in-progress-tracks` | Tracks 04, 05, 12, 15 | PR #7 merged at `92c67c6`; `git cherry -v main` reports `- 5425ed4...`. | Superseded; no merge candidate. | Delete only after explicit approval for local and/or remote branch deletion. |
| `finish-active-reference-surfaces` | Tracks 04, 05, 12, 15 | PR #9 merged at `2396fe6`; `git cherry -v main` reports `- ce87a6a...`. | Superseded; no merge candidate. | Delete only after explicit approval for local and/or remote branch deletion. |
| `implement-active-tracks-next` | Tracks 04, 05, 12, 15 | PR #8 merged at `0cdffd0`; `git cherry -v main` reports `- 678c8a1...`. | Superseded; no merge candidate. | Delete only after explicit approval for local and/or remote branch deletion. |
| `implement-intake-providers-cleaning-exports` | Tracks 03, 06, 07, 11 | PR #10 merged at `bd2c5f7`; `git cherry -v main` reports `- abeae3a...`. | Superseded; no merge candidate. | Delete only after explicit approval for local and/or remote branch deletion. |
| `require-explicit-export-output` | Track 11 | PR #12 merged at `dacc329`; `git cherry -v main` reports `- a08ca93...`. | Superseded; no merge candidate. | Delete only after explicit approval for local and/or remote branch deletion. |
| `switch-to-renovate` | Tracks 26, 41, 42 | PR #6 merged at `821f84e`; `git cherry -v main` reports `- 6316691...`. | Superseded; no merge candidate. | Delete only after explicit approval for local and/or remote branch deletion. |

## Explicit Approval Gates

- **Local branch deletion**: requires the user to approve `git branch -d` and
  name the branches to delete.
- **Remote branch deletion**: requires the user to approve
  `git push origin --delete <branch>` and name the remote branches to delete.
- **Merging**: requires the user to name the source branch and target branch,
  approve use of a temporary integration branch, and review targeted checks
  before any final merge.
- **Force operations**: require a separate explicit approval naming the exact ref
  and operation. No force operation is recommended by this inventory.
- **Stash cleanup**: requires a fresh non-empty `git stash list` plus explicit
  approval for `git stash drop` or `git stash clear`. Current inventory shows no
  stash entries.
