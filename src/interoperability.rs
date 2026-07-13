//! Deterministic, loss-aware comparisons for optional citation-format oracles.
//!
//! Oracle output is treated as an observation. It is never written to the
//! canonical CSL document or verification sidecar by this module.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

pub const INTEROPERABILITY_REPORT_SCHEMA_VERSION: &str = "sourceright.interoperability_report.v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DifferenceClass {
    InformationLoss,
    UnsupportedConstruct,
    ParserDiscrepancy,
    ReviewRequired,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SemanticDifference {
    pub item_id: String,
    pub path: String,
    pub class: DifferenceClass,
    pub canonical: Option<Value>,
    pub oracle: Option<Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InteroperabilityReport {
    pub schema_version: String,
    pub source: String,
    pub canonical_items: usize,
    pub oracle_items: usize,
    pub equivalent_items: usize,
    pub differences: Vec<SemanticDifference>,
}

impl InteroperabilityReport {
    pub fn is_equivalent(&self) -> bool {
        self.differences.is_empty() && self.canonical_items == self.oracle_items
    }

    pub fn markdown(&self) -> String {
        let mut output = format!(
            "# Interoperability comparison\n\n- Schema: `{}`\n- Source: `{}`\n- Canonical items: {}\n- Oracle items: {}\n- Equivalent items: {}\n\n",
            self.schema_version,
            self.source,
            self.canonical_items,
            self.oracle_items,
            self.equivalent_items
        );
        if self.differences.is_empty() {
            output.push_str("No semantic differences detected.\n");
            return output;
        }
        output.push_str(
            "| Item | Path | Class | Canonical | Oracle |\n| --- | --- | --- | --- | --- |\n",
        );
        for difference in &self.differences {
            output.push_str(&format!(
                "| {} | {} | {:?} | {} | {} |\n",
                difference.item_id,
                difference.path,
                difference.class,
                compact_value(difference.canonical.as_ref()),
                compact_value(difference.oracle.as_ref())
            ));
        }
        output
    }
}

pub fn compare_csl_json(
    canonical: &Value,
    oracle: &Value,
    source: impl Into<String>,
) -> InteroperabilityReport {
    let canonical_items = csl_items(canonical);
    let oracle_items = csl_items(oracle);
    let count = canonical_items.len().min(oracle_items.len());
    let mut differences = Vec::new();
    let semantic_fields = [
        "type",
        "title",
        "DOI",
        "container-title",
        "author",
        "issued",
        "URL",
    ];

    for index in 0..count {
        let canonical_item = canonical_items[index];
        let oracle_item = oracle_items[index];
        let item_id = item_id(canonical_item, index);
        for field in semantic_fields {
            let canonical_value = canonical_item.get(field).or_else(|| {
                if field == "DOI" {
                    canonical_item.get("doi")
                } else {
                    None
                }
            });
            let oracle_value = oracle_item.get(field).or_else(|| {
                if field == "DOI" {
                    oracle_item.get("doi")
                } else {
                    None
                }
            });
            if semantic_value(canonical_value) == semantic_value(oracle_value) {
                continue;
            }
            let class = match (canonical_value, oracle_value) {
                (Some(_), None) => DifferenceClass::InformationLoss,
                (None, Some(_)) => DifferenceClass::UnsupportedConstruct,
                (Some(_), Some(_)) => DifferenceClass::ReviewRequired,
                (None, None) => continue,
            };
            differences.push(SemanticDifference {
                item_id: item_id.clone(),
                path: format!("$[{index}].{field}"),
                class,
                canonical: canonical_value.cloned(),
                oracle: oracle_value.cloned(),
            });
        }

        let canonical_keys: BTreeSet<_> = canonical_item.keys().collect();
        for key in oracle_item
            .keys()
            .filter(|key| !canonical_keys.contains(key))
        {
            if semantic_fields.contains(&key.as_str())
                || ["doi", "id", "citation-key"].contains(&key.as_str())
            {
                continue;
            }
            differences.push(SemanticDifference {
                item_id: item_id.clone(),
                path: format!("$[{index}].{key}"),
                class: DifferenceClass::UnsupportedConstruct,
                canonical: None,
                oracle: oracle_item.get(key).cloned(),
            });
        }
    }

    if canonical_items.len() != oracle_items.len() {
        differences.push(SemanticDifference {
            item_id: "document".to_string(),
            path: "$.items.length".to_string(),
            class: if canonical_items.len() > oracle_items.len() {
                DifferenceClass::InformationLoss
            } else {
                DifferenceClass::UnsupportedConstruct
            },
            canonical: Some(Value::from(canonical_items.len())),
            oracle: Some(Value::from(oracle_items.len())),
        });
    }
    differences.sort_by(|left, right| {
        left.item_id
            .cmp(&right.item_id)
            .then(left.path.cmp(&right.path))
    });
    InteroperabilityReport {
        schema_version: INTEROPERABILITY_REPORT_SCHEMA_VERSION.to_string(),
        source: source.into(),
        canonical_items: canonical_items.len(),
        oracle_items: oracle_items.len(),
        equivalent_items: count.saturating_sub(
            differences
                .iter()
                .filter(|difference| difference.item_id != "document")
                .map(|difference| difference.item_id.as_str())
                .collect::<BTreeSet<_>>()
                .len(),
        ),
        differences,
    }
}

fn csl_items(value: &Value) -> Vec<&Map<String, Value>> {
    match value {
        Value::Array(items) => items.iter().filter_map(Value::as_object).collect(),
        Value::Object(object) => object
            .get("items")
            .and_then(Value::as_array)
            .map(|items| items.iter().filter_map(Value::as_object).collect())
            .unwrap_or_else(|| {
                if object.contains_key("type") {
                    vec![object]
                } else {
                    Vec::new()
                }
            }),
        _ => Vec::new(),
    }
}

fn item_id(item: &Map<String, Value>, index: usize) -> String {
    item.get("id")
        .and_then(Value::as_str)
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| format!("index-{index}"))
}

fn semantic_value(value: Option<&Value>) -> Option<Value> {
    match value {
        Some(Value::String(text)) => Some(Value::String(
            text.split_whitespace().collect::<Vec<_>>().join(" "),
        )),
        Some(Value::Array(values)) => {
            Some(Value::Array(values.iter().map(normalize_json).collect()))
        }
        Some(value) => Some(normalize_json(value)),
        None => None,
    }
}

fn normalize_json(value: &Value) -> Value {
    match value {
        Value::String(text) => Value::String(text.split_whitespace().collect::<Vec<_>>().join(" ")),
        Value::Array(values) => Value::Array(values.iter().map(normalize_json).collect()),
        Value::Object(object) => Value::Object(
            object
                .iter()
                .map(|(key, value)| (key.clone(), normalize_json(value)))
                .collect(),
        ),
        _ => value.clone(),
    }
}

fn compact_value(value: Option<&Value>) -> String {
    value
        .map(|value| serde_json::to_string(value).unwrap_or_else(|_| "<invalid-json>".to_string()))
        .unwrap_or_else(|| "null".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn whitespace_is_semantically_equivalent() {
        let canonical = json!([{"id":"a","type":"article-journal","title":"A  title"}]);
        let oracle = json!([{"id":"a","type":"article-journal","title":"A title"}]);
        assert!(compare_csl_json(&canonical, &oracle, "fixture").is_equivalent());
    }

    #[test]
    fn loss_and_unsupported_fields_are_classified() {
        let canonical = json!([{"id":"a","type":"article-journal","title":"A","DOI":"10.1/a"}]);
        let oracle = json!([{"id":"a","type":"article-journal","title":"A","extra":"x"}]);
        let report = compare_csl_json(&canonical, &oracle, "fixture");
        assert!(
            report
                .differences
                .iter()
                .any(|d| d.class == DifferenceClass::InformationLoss)
        );
        assert!(
            report
                .differences
                .iter()
                .any(|d| d.class == DifferenceClass::UnsupportedConstruct)
        );
        assert!(!report.markdown().is_empty());
    }
}
