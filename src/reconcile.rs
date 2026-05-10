use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::csl::{CslDocument, CslItem, normalize_identifier, normalize_title};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CitationReconciliationReport {
    pub occurrences: Vec<CitationOccurrence>,
    pub matches: Vec<CitationMatch>,
    pub issues: Vec<CitationReconciliationIssue>,
}

impl CitationReconciliationReport {
    pub fn to_markdown(&self) -> String {
        let mut markdown = String::from("# Sourceright Citation Reconciliation Report\n\n");
        markdown.push_str(&format!(
            "- Citation occurrences: {}\n- Matched citations: {}\n- Issues: {}\n\n",
            self.occurrences.len(),
            self.matches.len(),
            self.issues.len()
        ));

        if self.issues.is_empty() {
            markdown.push_str("No in-text citation reconciliation issues detected.\n");
        } else {
            for issue in &self.issues {
                markdown.push_str(&format!(
                    "- `{}` `{}`: {}\n",
                    issue.issue_type.as_str(),
                    issue.reference_id.as_deref().unwrap_or("document"),
                    issue.message
                ));
            }
        }

        markdown
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CitationOccurrence {
    pub text: String,
    pub style: CitationStyle,
    pub key: String,
    pub span: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CitationStyle {
    AuthorDate,
    Numeric,
    FootnoteLike,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CitationMatch {
    pub occurrence_text: String,
    pub reference_id: String,
    pub confidence: CitationMatchConfidence,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CitationMatchConfidence {
    Exact,
    Probable,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CitationReconciliationIssue {
    pub issue_type: CitationReconciliationIssueType,
    pub reference_id: Option<String>,
    pub citation_text: Option<String>,
    pub message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CitationReconciliationIssueType {
    MissingReference,
    UncitedReference,
    DuplicateCitation,
    NumericOrder,
    AmbiguousMatch,
}

impl CitationReconciliationIssueType {
    fn as_str(self) -> &'static str {
        match self {
            Self::MissingReference => "missing_reference",
            Self::UncitedReference => "uncited_reference",
            Self::DuplicateCitation => "duplicate_citation",
            Self::NumericOrder => "numeric_order",
            Self::AmbiguousMatch => "ambiguous_match",
        }
    }
}

pub fn reconcile_citations(text: &str, references: &CslDocument) -> CitationReconciliationReport {
    let occurrences = extract_citation_occurrences(text);
    let reference_index = ReferenceIndex::new(references);
    let mut matches = Vec::new();
    let mut issues = Vec::new();
    let mut cited_ids = BTreeSet::new();
    let mut citation_counts = BTreeMap::<String, usize>::new();
    let mut last_numeric = 0;

    for occurrence in &occurrences {
        if occurrence.style == CitationStyle::Numeric {
            if let Ok(number) = occurrence.key.parse::<usize>() {
                if number < last_numeric {
                    issues.push(CitationReconciliationIssue {
                        issue_type: CitationReconciliationIssueType::NumericOrder,
                        reference_id: None,
                        citation_text: Some(occurrence.text.clone()),
                        message: "Numeric citation appears after a higher numbered citation."
                            .to_string(),
                    });
                }
                last_numeric = last_numeric.max(number);
            }
        }

        match reference_index.match_occurrence(occurrence) {
            MatchResult::Matched(reference_id, confidence) => {
                cited_ids.insert(reference_id.clone());
                *citation_counts.entry(reference_id.clone()).or_default() += 1;
                matches.push(CitationMatch {
                    occurrence_text: occurrence.text.clone(),
                    reference_id,
                    confidence,
                });
            }
            MatchResult::Ambiguous(ids) => issues.push(CitationReconciliationIssue {
                issue_type: CitationReconciliationIssueType::AmbiguousMatch,
                reference_id: None,
                citation_text: Some(occurrence.text.clone()),
                message: format!(
                    "Citation could match multiple references: {}",
                    ids.join(", ")
                ),
            }),
            MatchResult::Missing => issues.push(CitationReconciliationIssue {
                issue_type: CitationReconciliationIssueType::MissingReference,
                reference_id: None,
                citation_text: Some(occurrence.text.clone()),
                message: "Citation does not match a reference-list entry.".to_string(),
            }),
        }
    }

    for (reference_id, count) in citation_counts {
        if count > 1 {
            issues.push(CitationReconciliationIssue {
                issue_type: CitationReconciliationIssueType::DuplicateCitation,
                reference_id: Some(reference_id),
                citation_text: None,
                message: "Reference is cited more than once; check whether repeated citation is intended."
                    .to_string(),
            });
        }
    }

    for item in &references.items {
        if !cited_ids.contains(&item.id) {
            issues.push(CitationReconciliationIssue {
                issue_type: CitationReconciliationIssueType::UncitedReference,
                reference_id: Some(item.id.clone()),
                citation_text: None,
                message: "Reference-list entry has no detected in-text citation.".to_string(),
            });
        }
    }

    CitationReconciliationReport {
        occurrences,
        matches,
        issues,
    }
}

pub fn extract_citation_occurrences(text: &str) -> Vec<CitationOccurrence> {
    let mut occurrences = Vec::new();
    for (line_index, line) in text.lines().enumerate() {
        extract_parenthetical(line, line_index + 1, &mut occurrences);
        extract_numeric(line, line_index + 1, &mut occurrences);
    }
    occurrences
}

fn extract_parenthetical(
    line: &str,
    line_number: usize,
    occurrences: &mut Vec<CitationOccurrence>,
) {
    let mut cursor = 0;
    while let Some(start) = line[cursor..].find('(') {
        let absolute_start = cursor + start;
        let Some(end) = line[absolute_start..].find(')') else {
            break;
        };
        let absolute_end = absolute_start + end + 1;
        let candidate = &line[absolute_start..absolute_end];
        if candidate.contains(',') && candidate.chars().any(|ch| ch.is_ascii_digit()) {
            occurrences.push(CitationOccurrence {
                text: candidate.to_string(),
                style: CitationStyle::AuthorDate,
                key: author_key(candidate),
                span: format!("line:{line_number}:{}-{absolute_end}", absolute_start + 1),
            });
        }
        cursor = absolute_end;
    }
}

fn extract_numeric(line: &str, line_number: usize, occurrences: &mut Vec<CitationOccurrence>) {
    let mut cursor = 0;
    while let Some(start) = line[cursor..].find('[') {
        let absolute_start = cursor + start;
        let Some(end) = line[absolute_start..].find(']') else {
            break;
        };
        let absolute_end = absolute_start + end + 1;
        let candidate = &line[absolute_start..absolute_end];
        let key = candidate.trim_matches(['[', ']']).trim();
        if key.chars().all(|ch| ch.is_ascii_digit()) && !key.is_empty() {
            occurrences.push(CitationOccurrence {
                text: candidate.to_string(),
                style: CitationStyle::Numeric,
                key: key.to_string(),
                span: format!("line:{line_number}:{}-{absolute_end}", absolute_start + 1),
            });
        }
        cursor = absolute_end;
    }
}

fn author_key(candidate: &str) -> String {
    candidate
        .trim_matches(['(', ')'])
        .split(',')
        .next()
        .unwrap_or_default()
        .split_whitespace()
        .next()
        .unwrap_or_default()
        .to_ascii_lowercase()
}

enum MatchResult {
    Matched(String, CitationMatchConfidence),
    Ambiguous(Vec<String>),
    Missing,
}

struct ReferenceIndex<'a> {
    references: &'a CslDocument,
    author_keys: BTreeMap<String, Vec<String>>,
}

impl<'a> ReferenceIndex<'a> {
    fn new(references: &'a CslDocument) -> Self {
        let mut author_keys = BTreeMap::<String, Vec<String>>::new();
        for item in &references.items {
            for key in reference_author_keys(item) {
                author_keys.entry(key).or_default().push(item.id.clone());
            }
        }
        Self {
            references,
            author_keys,
        }
    }

    fn match_occurrence(&self, occurrence: &CitationOccurrence) -> MatchResult {
        match occurrence.style {
            CitationStyle::Numeric | CitationStyle::FootnoteLike => occurrence
                .key
                .parse::<usize>()
                .ok()
                .and_then(|number| number.checked_sub(1))
                .and_then(|index| self.references.items.get(index))
                .map(|item| MatchResult::Matched(item.id.clone(), CitationMatchConfidence::Exact))
                .unwrap_or(MatchResult::Missing),
            CitationStyle::AuthorDate => {
                let ids = self
                    .author_keys
                    .get(&occurrence.key)
                    .cloned()
                    .unwrap_or_default();
                match ids.as_slice() {
                    [id] => MatchResult::Matched(id.clone(), CitationMatchConfidence::Probable),
                    [] => MatchResult::Missing,
                    _ => MatchResult::Ambiguous(ids),
                }
            }
        }
    }
}

fn reference_author_keys(item: &CslItem) -> Vec<String> {
    let mut keys = Vec::new();
    if let Some(authors) = item.extra.get("author").and_then(|value| value.as_array()) {
        for author in authors {
            if let Some(family) = author.get("family").and_then(|value| value.as_str()) {
                keys.push(family.to_ascii_lowercase());
            }
        }
    }

    if keys.is_empty() {
        let id = normalize_identifier(&item.id).to_ascii_lowercase();
        if let Some(first) = id.split(['-', ' ', '_']).find(|part| !part.is_empty()) {
            keys.push(first.to_string());
        }
    }

    if keys.is_empty()
        && let Some(title) = item.title.as_deref()
    {
        keys.push(
            normalize_title(title)
                .to_ascii_lowercase()
                .split_whitespace()
                .next()
                .unwrap_or_default()
                .to_string(),
        );
    }

    keys
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use serde_json::json;

    use super::*;
    use crate::csl::CslItem;

    #[test]
    fn author_date_citations_match_reference_author_keys() {
        let references = CslDocument {
            items: vec![CslItem {
                id: "smith-2024".to_string(),
                item_type: "article-journal".to_string(),
                title: Some("Trial".to_string()),
                doi: None,
                extra: BTreeMap::from([("author".to_string(), json!([{"family": "Smith"}]))]),
            }],
        };

        let report = reconcile_citations("Evidence supports this (Smith, 2024).", &references);

        assert_eq!(report.occurrences.len(), 1);
        assert_eq!(report.matches[0].reference_id, "smith-2024");
        assert!(report.issues.is_empty());
    }

    #[test]
    fn numeric_citations_report_missing_uncited_duplicate_and_order_issues() {
        let references = CslDocument {
            items: vec![
                CslItem {
                    id: "first".to_string(),
                    item_type: "article-journal".to_string(),
                    title: Some("First".to_string()),
                    doi: None,
                    extra: BTreeMap::new(),
                },
                CslItem {
                    id: "second".to_string(),
                    item_type: "article-journal".to_string(),
                    title: Some("Second".to_string()),
                    doi: None,
                    extra: BTreeMap::new(),
                },
            ],
        };

        let report = reconcile_citations(
            "Prior work [2] disagrees with [1] and [3]. [1]",
            &references,
        );
        let issue_types = report
            .issues
            .iter()
            .map(|issue| issue.issue_type)
            .collect::<Vec<_>>();

        assert!(issue_types.contains(&CitationReconciliationIssueType::NumericOrder));
        assert!(issue_types.contains(&CitationReconciliationIssueType::MissingReference));
        assert!(issue_types.contains(&CitationReconciliationIssueType::DuplicateCitation));
        assert!(!issue_types.contains(&CitationReconciliationIssueType::UncitedReference));
    }

    #[test]
    fn ambiguous_author_matches_are_reported_for_manual_review() {
        let references = CslDocument {
            items: vec![
                CslItem {
                    id: "smith-a".to_string(),
                    item_type: "article-journal".to_string(),
                    title: Some("A".to_string()),
                    doi: None,
                    extra: BTreeMap::from([("author".to_string(), json!([{"family": "Smith"}]))]),
                },
                CslItem {
                    id: "smith-b".to_string(),
                    item_type: "article-journal".to_string(),
                    title: Some("B".to_string()),
                    doi: None,
                    extra: BTreeMap::from([("author".to_string(), json!([{"family": "Smith"}]))]),
                },
            ],
        };

        let report = reconcile_citations("(Smith, 2024)", &references);

        assert_eq!(
            report.issues[0].issue_type,
            CitationReconciliationIssueType::AmbiguousMatch
        );
        assert!(report.to_markdown().contains("ambiguous_match"));
    }
}
