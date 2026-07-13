# Track 108 Specification

Reporting must remain useful when a workspace contains canonical CSL but no verification sidecar. It must report the CSL records with a non-risk informational boundary diagnostic rather than failing on a missing derived artifact. If a sidecar exists but cannot be parsed, the error must identify the path and give a repair action; provider evidence must never be silently discarded or merged into canonical CSL.

GitHub issues: #18 and #19.
