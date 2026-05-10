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
    normalize_string_extra(
        item,
        &reference_id,
        transformations,
        "container-title",
        false,
    );
    normalize_string_extra(item, &reference_id, transformations, "publisher", false);
    normalize_issn(item, &reference_id, transformations);
    normalize_isbn(item, &reference_id, transformations);
    normalize_names(item, &reference_id, transformations);
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

fn normalize_string_extra(
    item: &mut CslItem,
    reference_id: &str,
    transformations: &mut Vec<CleaningTransformation>,
    field: &str,
    requires_review: bool,
) {
    let Some(Value::String(before)) = item.extra.get(field).cloned() else {
        return;
    };
    let after = normalize_title(&before);
    if before != after {
        transformations.push(CleaningTransformation {
            reference_id: reference_id.to_string(),
            field: field.to_string(),
            before,
            after: after.clone(),
            requires_review,
        });
        item.extra.insert(field.to_string(), Value::String(after));
    }
}

fn normalize_issn(
    item: &mut CslItem,
    reference_id: &str,
    transformations: &mut Vec<CleaningTransformation>,
) {
    normalize_identifier_extra(item, reference_id, transformations, "ISSN", |value| {
        let raw = value.replace([' ', '-'], "").to_ascii_uppercase();
        if raw.len() == 8 {
            format!("{}-{}", &raw[0..4], &raw[4..8])
        } else {
            normalize_title(value)
        }
    });
}

fn normalize_isbn(
    item: &mut CslItem,
    reference_id: &str,
    transformations: &mut Vec<CleaningTransformation>,
) {
    normalize_identifier_extra(item, reference_id, transformations, "ISBN", |value| {
        value
            .chars()
            .filter(|ch| !ch.is_whitespace() && *ch != '-' && *ch != '–' && *ch != '—')
            .collect::<String>()
            .to_ascii_uppercase()
    });
}

fn normalize_identifier_extra(
    item: &mut CslItem,
    reference_id: &str,
    transformations: &mut Vec<CleaningTransformation>,
    field: &str,
    normalize: impl Fn(&str) -> String,
) {
    let Some(Value::String(before)) = item.extra.get(field).cloned() else {
        return;
    };
    let after = normalize(&before);
    if before != after {
        transformations.push(CleaningTransformation {
            reference_id: reference_id.to_string(),
            field: field.to_string(),
            before,
            after: after.clone(),
            requires_review: false,
        });
        item.extra.insert(field.to_string(), Value::String(after));
    }
}

fn normalize_names(
    item: &mut CslItem,
    reference_id: &str,
    transformations: &mut Vec<CleaningTransformation>,
) {
    let Some(Value::Array(names)) = item.extra.get_mut("author") else {
        return;
    };

    for (index, name) in names.iter_mut().enumerate() {
        let Some(object) = name.as_object_mut() else {
            continue;
        };
        for part in ["family", "given"] {
            let Some(Value::String(before)) = object.get(part).cloned() else {
                continue;
            };
            let after = normalize_title(&before);
            if before != after {
                transformations.push(CleaningTransformation {
                    reference_id: reference_id.to_string(),
                    field: format!("author[{index}].{part}"),
                    before,
                    after: after.clone(),
                    requires_review: false,
                });
                object.insert(part.to_string(), Value::String(after));
            }
        }
    }
}

fn normalize_issued_date(
    item: &mut CslItem,
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
    item.extra.insert(
        "issued".to_string(),
        serde_json::json!({"date-parts": [[year.parse::<u16>().expect("validated year")]]}),
    );
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
    fn container_identifiers_and_author_names_are_normalized() {
        let document = CslDocument {
            items: vec![CslItem {
                id: "book".to_string(),
                item_type: "book".to_string(),
                title: Some("Book".to_string()),
                doi: None,
                extra: BTreeMap::from([
                    (
                        "container-title".to_string(),
                        Value::String("  Example   Journal ".to_string()),
                    ),
                    ("ISSN".to_string(), Value::String("1234 567X".to_string())),
                    (
                        "ISBN".to_string(),
                        Value::String("978-1-4028-9462-6".to_string()),
                    ),
                    (
                        "author".to_string(),
                        serde_json::json!([{"family": " Smith ", "given": " Jane   Q. "}]),
                    ),
                ]),
            }],
        };

        let report = standardize_document(&document, &VerificationSidecar::empty());
        let item = &report.document.items[0];

        assert_eq!(
            item.extra["container-title"],
            Value::String("Example Journal".to_string())
        );
        assert_eq!(item.extra["ISSN"], Value::String("1234-567X".to_string()));
        assert_eq!(
            item.extra["ISBN"],
            Value::String("9781402894626".to_string())
        );
        assert_eq!(item.extra["author"][0]["family"], "Smith");
        assert_eq!(item.extra["author"][0]["given"], "Jane Q.");
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
            report.document.items[0].extra["issued"],
            serde_json::json!({"date-parts": [[2024]]})
        );
        assert_eq!(
            report.sidecar.references["date-risk"].review_status,
            ReviewStatus::Queued
        );
    }
}
