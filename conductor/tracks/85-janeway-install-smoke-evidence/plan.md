# Track 85 - Implementation Plan

## Phase 1: Discover [ ]

- [ ] Identify the best disposable Janeway test target available to the repo.
- [ ] Record how the Janeway install and plugin activation steps should be executed.
- [ ] Decide what transcript and report evidence the smoke run must capture.

## Phase 2: Lock spec [ ]

- [ ] Freeze the smoke protocol and blocker documentation format.
- [ ] Confirm the claim boundary: install-smoked, not published.
- [ ] Lock the evidence artifacts that need to be committed after the smoke run.

## Phase 3: Implement [ ]

- [ ] Run the Janeway install and activation smoke when the environment is available.
- [ ] Capture the screening report output and transcript evidence.
- [ ] If the live path is blocked, write the precise blocker with the attempted command chain.

## Phase 4: Run checks [ ]

- [ ] Verify the transcript references the installed package and report result.
- [ ] Confirm the smoke evidence is reviewable and bounded.
- [ ] Check the surrounding docs for claim-boundary wording.

## Phase 5: conductor-review [ ]

- [ ] Run `$conductor-review` on the Janeway smoke evidence.
- [ ] Record any findings about missing evidence or overclaim risk.

## Phase 6: Apply fixes [ ]

- [ ] Repair transcript formatting, blocker wording, or report evidence if review finds issues.
- [ ] Re-run the smoke if the environment allows it.

## Phase 7: Progress [ ]

- [ ] Promote Janeway from smoke evidence into publication-hardening only after the smoke artifact is stable.
- [ ] Keep publication wording separate from evidence wording.
