# Standardisation Cleaning Test Matrix

| Scenario | Expected result |
| --- | --- |
| Messy DOI | Normalized DOI is stored. |
| Author variants | Pending richer CSL name parser; current implementation preserves originals. |
| Duplicate records | Duplicate candidates are grouped. |
| Risky transformation | Record is queued for manual review. |
| Page ranges | En dash and em dash page ranges are normalized deterministically. |
