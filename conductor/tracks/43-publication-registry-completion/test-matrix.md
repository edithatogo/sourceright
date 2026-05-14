# Publication Registry Completion Test Matrix

| Scenario | Acceptance |
| --- | --- |
| Accepted registry | Public listing URL, version, date checked, and install metadata are recorded. |
| Prepared registry | Metadata exists but docs explicitly say not accepted yet. |
| Deferred registry | Blocking requirement and revisit trigger are documented. |
| GHCR evidence | Direct package evidence is captured when permissions allow; otherwise MCP Registry indirection is labelled. |
| Package-manager decision | Homebrew/Scoop/Chocolatey/winget/npm/PyPI each has a yes/no/defer rationale. |
| Review loop | `$conductor-review` runs and findings are fixed or deferred. |
