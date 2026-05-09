use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::csl::{CslDocument, CslItem, normalize_doi, normalize_identifier, normalize_title};
use crate::sidecar::{ReviewStatus, VerificationSidecar};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CleaningReport {
    pub document: CslDocument,
    pub sidecar: VerificationSidecar,
    pub transformations: Vec<CleaningTransformation>,
    pub duplicate_groups: Vec<DuplicateGroup>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CleaningTransformation {
    pub reference_id: String,
    pub field: String,
    pub before: String,
    pub after: String,
    pub requires_review: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DuplicateGroup {
    pub key: String,
    pub reference_ids: Vec<String>,
}

pub fn standardize_document(
    document: &CslDocument,
    sidecar: &VerificationSidecar,
) -> CleaningReport {
    let mut cleaned = document.clone();
    let mut sidecar = sidecar.clone();
    let mut transformations = Vec::new();

    for item in &mut cleaned.items {
        normalize_item(item, &mut sidecar, &mut transformations);
    }

    let duplicate_groups = duplicate_groups(&cleaned);
    for group in &duplicate_groups {
        for reference_id in &group.reference_ids {
            sidecar
                .references
                .entry(reference_id.clone())
                .or_default()
                .review_status = ReviewStatus::Queued;
        }
    }

    CleaningReport {
        document: cleaned,
        sidecar,
        transformations,
        duplicate_groups,
    }
}

fn normalize_item(
    item: &mut CslItem,
    sidecar: &mut VerificationSidecar,
    transformations: &mut Vec<CleaningTransformation>,
) {
    let reference_id = item.id.clone();
    replace_if_changed(
        transformations,
        &reference_id,
        "id",
        item.id.clone(),
        normalize_identifier(&item.id),
        false,
    )
    .inspect(|value| item.id = value.clone());

    let normalized_title = item.title.as_deref().map(normalize_title);
    if let (Some(before), Some(after)) = (item.title.clone(), normalized_title)
        && before != after
    {
        transformations.push(CleaningTransformation {
            reference_id: reference_id.clone(),
            field: "title".to_string(),
            before,
            after: after.clone(),
            requires_review: false,
        });
        item.title = Some(after);
    }

    if let Some(before) = item.doi.clone() {
        let after = normalize_doi(&before);
        if before != after {
            transformations.push(CleaningTransformation {
                reference_id: reference_id.clone(),
                field: "DOI".to_string(),
                before,
                after: after.clone(),
                requires_review: false,
            });
            item.doi = (!after.is_empty()).then_some(after);
        }
    }

    normalize_pages(item, &reference_id, transformations);
    normalize_issued_date(item, &reference_id, sidecar, transformations);
}

fn replace_if_changed(
    transformations: &mut Vec<CleaningTransformation>,
    reference_id: &str,
    field: &str,
    before: String,
    after: String,
    requires_review: bool,
) -> Option<String> {
    if before == after {
        return None;
    }

    transformations.push(CleaningTransformation {
        reference_id: reference_id.to_string(),
        field: field.to_string(),
        before,
        after: after.clone(),
        requires_review,
    });
    Some(after)
}

fn normalize_pages(
    item: &mut CslItem,
    reference_id: &str,
    transformations: &mut Vec<CleaningTransformation>,
) {
    let Some(Value::String(before)) = item.extra.get("page").cloned() else {
        return;
    };
    let after = before.replace(['–', '—'], "-");
    if before != after {
        transformations.push(CleaningTransformation {
            reference_id: reference_id.to_string(),
            field: "page".to_string(),
            before,
            after: after.clone(),
            requires_review: false,
        });
        item.extra.insert("page".to_string(), Value::String(after));
    }
}

fn normalize_issued_date(
    item: &CslItem,
    reference_id: &str,
    sidecar: &mut VerificationSidecar,
    transformations: &mut Vec<CleaningTransformation>,
) {
    let Some(Value::String(date)) = item.extra.get("issued") else {
        return;
    };
    let Some(year) = date
        .get(0..4)
        .filter(|year| year.chars().all(|ch| ch.is_ascii_digit()))
    else {
        return;
    };

    transformations.push(CleaningTransformation {
        reference_id: reference_id.to_string(),
        field: "issued".to_string(),
        before: date.clone(),
        after: format!("{{\"date-parts\":[[{year}]]}}"),
        requires_review: true,
    });
    sidecar
        .references
        .entry(reference_id.to_string())
        .or_default()
        .review_status = ReviewStatus::Queued;
}

fn duplicate_groups(document: &CslDocument) -> Vec<DuplicateGroup> {
    let mut by_key = BTreeMap::<String, Vec<String>>::new();
    for item in &document.items {
        let key = item
            .doi
            .as_deref()
            .map(normalize_doi)
            .filter(|doi| !doi.is_empty())
            .unwrap_or_else(|| {
                format!(
                    "{}:{}",
                    item.item_type,
                    item.title
                        .as_deref()
                        .map(normalize_title)
                        .unwrap_or_default()
                )
            });
        by_key.entry(key).or_default().push(item.id.clone());
    }

    by_key
        .into_iter()
        .filter_map(|(key, reference_ids)| {
            (reference_ids.len() > 1).then_some(DuplicateGroup { key, reference_ids })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn messy_doi_and_pages_are_normalized_and_recorded() {
        let document = CslDocument {
            items: vec![CslItem {
                id: "smith-2024".to_string(),
                item_type: "article-journal".to_string(),
                title: Some("  Trial   Paper ".to_string()),
                doi: Some("https://doi.org/10.1000/ABC".to_string()),
                extra: BTreeMap::from([("page".to_string(), Value::String("1–4".to_string()))]),
            }],
        };

        let report = standardize_document(&document, &VerificationSidecar::empty());
        let item = &report.document.items[0];

        assert_eq!(item.title.as_deref(), Some("Trial Paper"));
        assert_eq!(item.doi.as_deref(), Some("10.1000/abc"));
        assert_eq!(item.extra["page"], Value::String("1-4".to_string()));
        assert_eq!(report.transformations.len(), 3);
    }

    #[test]
    fn duplicate_records_are_grouped_and_queued_for_review() {
        let document = CslDocument {
            items: vec![
                CslItem {
                    id: "a".to_string(),
                    item_type: "article-journal".to_string(),
                    title: Some("Same".to_string()),
                    doi: Some("10.1000/example".to_string()),
                    extra: BTreeMap::new(),
                },
                CslItem {
                    id: "b".to_string(),
                    item_type: "article-journal".to_string(),
                    title: Some("Same".to_string()),
                    doi: Some("https://doi.org/10.1000/EXAMPLE".to_string()),
                    extra: BTreeMap::new(),
                },
            ],
        };

        let report = standardize_document(&document, &VerificationSidecar::empty());

        assert_eq!(report.duplicate_groups.len(), 1);
        assert_eq!(report.duplicate_groups[0].reference_ids, ["a", "b"]);
        assert_eq!(
            report.sidecar.references["a"].review_status,
            ReviewStatus::Queued
        );
    }

    #[test]
    fn risky_date_transformation_is_queued_for_manual_review() {
        let document = CslDocument {
            items: vec![CslItem {
                id: "date-risk".to_string(),
                item_type: "article-journal".to_string(),
                title: Some("Date risk".to_string()),
                doi: None,
                extra: BTreeMap::from([(
                    "issued".to_string(),
                    Value::String("2024 Spring".to_string()),
                )]),
            }],
        };

        let report = standardize_document(&document, &VerificationSidecar::empty());

        assert!(report.transformations[0].requires_review);
        assert_eq!(
            report.sidecar.references["date-risk"].review_status,
            ReviewStatus::Queued
        );
    }
}
