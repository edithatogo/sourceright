//! Optional, source-grounded entity recognition and linking contracts.
//!
//! Entity output is independent from references, CSL, providers, and legal
//! models. The deterministic recognizer is a lexicon baseline for fixtures and
//! domain packs, not a general scientific NER claim.

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::reference_model::SourceSpan;

pub const ENTITY_MODEL_SCHEMA_VERSION: &str = "citeweft.entity-model.v1";
pub const ENTITY_VOCABULARY_VERSION: &str = "citeweft.entity-vocabulary.v1";
pub const DEFAULT_ENTITY_MODEL_MAX_INPUT_BYTES: usize = 25 * 1024 * 1024;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntityClass {
    pub scheme: String,
    pub id: String,
    pub label: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MappingRelation {
    Exact,
    Broad,
    Narrow,
    Heuristic,
    Unmapped,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntityMapping {
    pub original_label: String,
    pub class: Option<EntityClass>,
    pub relation: MappingRelation,
    pub rationale: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntityMention {
    pub id: String,
    pub text: String,
    pub span: SourceSpan,
    pub original_label: String,
    pub class: Option<EntityClass>,
    pub mapping_relation: MappingRelation,
    pub confidence: f32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<EntityLinkCandidate>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntityLinkCandidate {
    pub registry: String,
    pub version: String,
    pub query: String,
    pub identifier: Option<String>,
    pub method: String,
    pub score: Option<f32>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntityModelProvenance {
    pub backend: String,
    pub version: String,
    pub domain: String,
    pub language: String,
    pub configuration: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntityModelReport {
    pub schema_version: String,
    pub vocabulary_version: String,
    pub provenance: EntityModelProvenance,
    pub mentions: Vec<EntityMention>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub diagnostics: Vec<EntityDiagnostic>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntityDiagnostic {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Error)]
pub enum EntityModelError {
    #[error("entity model input exceeds {limit} bytes")]
    InputTooLarge { limit: usize },
    #[error("entity model pattern cannot be empty")]
    EmptyPattern,
    #[error("entity model input is not valid UTF-8")]
    InvalidUtf8,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EntityPattern {
    pub surface: String,
    pub original_label: String,
    pub class: Option<EntityClass>,
    pub relation: MappingRelation,
    pub confidence: f32,
    pub links: Vec<EntityLinkCandidate>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct DeterministicEntityRecognizer {
    pub version: String,
    pub domain: String,
    pub language: String,
    pub patterns: Vec<EntityPattern>,
}

impl DeterministicEntityRecognizer {
    pub fn scholarly(patterns: Vec<EntityPattern>) -> Self {
        Self {
            version: "deterministic-lexicon-v1".to_string(),
            domain: "general-scholarly".to_string(),
            language: "en".to_string(),
            patterns,
        }
    }

    pub fn recognize(&self, input: &[u8]) -> Result<EntityModelReport, EntityModelError> {
        if input.len() > DEFAULT_ENTITY_MODEL_MAX_INPUT_BYTES {
            return Err(EntityModelError::InputTooLarge {
                limit: DEFAULT_ENTITY_MODEL_MAX_INPUT_BYTES,
            });
        }
        if self
            .patterns
            .iter()
            .any(|pattern| pattern.surface.is_empty())
        {
            return Err(EntityModelError::EmptyPattern);
        }
        let text = std::str::from_utf8(input).map_err(|_| EntityModelError::InvalidUtf8)?;
        let mut candidates = self
            .patterns
            .iter()
            .flat_map(|pattern| {
                text.match_indices(&pattern.surface)
                    .map(move |(start, _)| (start, pattern))
            })
            .collect::<Vec<_>>();
        candidates.sort_by(|left, right| {
            left.0
                .cmp(&right.0)
                .then_with(|| right.1.surface.len().cmp(&left.1.surface.len()))
                .then_with(|| left.1.original_label.cmp(&right.1.original_label))
        });

        let mut mentions = Vec::new();
        let mut occupied_until = 0;
        for (start, pattern) in candidates {
            let end = start + pattern.surface.len();
            if start < occupied_until {
                continue;
            }
            occupied_until = end;
            mentions.push(EntityMention {
                id: format!("e{}", mentions.len() + 1),
                text: pattern.surface.clone(),
                span: SourceSpan {
                    start,
                    end,
                    text: pattern.surface.clone(),
                },
                original_label: pattern.original_label.clone(),
                class: pattern.class.clone(),
                mapping_relation: pattern.relation.clone(),
                confidence: pattern.confidence,
                links: pattern.links.clone(),
            });
        }
        Ok(EntityModelReport {
            schema_version: ENTITY_MODEL_SCHEMA_VERSION.to_string(),
            vocabulary_version: ENTITY_VOCABULARY_VERSION.to_string(),
            provenance: EntityModelProvenance {
                backend: "deterministic-lexicon".to_string(),
                version: self.version.clone(),
                domain: self.domain.clone(),
                language: self.language.clone(),
                configuration: "exact-surface-patterns".to_string(),
            },
            mentions,
            diagnostics: Vec::new(),
        })
    }
}

pub fn general_scholarly_vocabulary() -> Vec<EntityMapping> {
    [
        ("person", "Person", MappingRelation::Exact),
        ("organisation", "Organisation", MappingRelation::Exact),
        ("place", "Place", MappingRelation::Exact),
        ("work", "Scholarly work", MappingRelation::Broad),
        ("identifier", "Identifier", MappingRelation::Exact),
        ("date", "Date or period", MappingRelation::Broad),
        ("measure", "Measure", MappingRelation::Exact),
        ("concept", "Concept", MappingRelation::Exact),
        ("method", "Method", MappingRelation::Exact),
        ("software", "Software", MappingRelation::Exact),
        ("dataset", "Dataset", MappingRelation::Exact),
        ("instrument", "Instrument", MappingRelation::Exact),
        ("organism", "Organism", MappingRelation::Exact),
        ("substance", "Substance", MappingRelation::Exact),
        (
            "legal_authority",
            "Legal authority",
            MappingRelation::Narrow,
        ),
        ("event", "Event", MappingRelation::Exact),
        ("award", "Award", MappingRelation::Exact),
        ("project", "Project", MappingRelation::Exact),
        ("funder", "Funder", MappingRelation::Exact),
        ("unknown", "Unknown", MappingRelation::Unmapped),
    ]
    .into_iter()
    .map(|(id, label, relation)| EntityMapping {
        original_label: id.to_string(),
        class: (relation != MappingRelation::Unmapped).then(|| EntityClass {
            scheme: "citeweft".to_string(),
            id: id.to_string(),
            label: label.to_string(),
        }),
        relation,
        rationale: "stable extensible general-scholarly vocabulary mapping".to_string(),
    })
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn person_pattern() -> EntityPattern {
        EntityPattern {
            surface: "Ada Lovelace".to_string(),
            original_label: "person".to_string(),
            class: Some(EntityClass {
                scheme: "citeweft".to_string(),
                id: "person".to_string(),
                label: "Person".to_string(),
            }),
            relation: MappingRelation::Exact,
            confidence: 0.95,
            links: vec![EntityLinkCandidate {
                registry: "orcid".to_string(),
                version: "fixture-2026".to_string(),
                query: "Ada Lovelace".to_string(),
                identifier: Some("https://orcid.org/0000-0000-0000-0000".to_string()),
                method: "recorded_fixture_exact".to_string(),
                score: Some(1.0),
            }],
        }
    }

    #[test]
    fn recognizer_preserves_span_mapping_and_separate_link_evidence() {
        let report = DeterministicEntityRecognizer::scholarly(vec![person_pattern()])
            .recognize(b"Ada Lovelace developed a method.")
            .expect("recognize fixture");
        let mention = &report.mentions[0];
        assert_eq!(mention.text, "Ada Lovelace");
        assert_eq!(
            &b"Ada Lovelace developed a method."[mention.span.start..mention.span.end],
            b"Ada Lovelace"
        );
        assert_eq!(mention.mapping_relation, MappingRelation::Exact);
        assert_eq!(mention.links[0].query, mention.text);
    }

    #[test]
    fn overlapping_patterns_prefer_longest_surface_at_same_offset() {
        let mut shorter = person_pattern();
        shorter.surface = "Ada".to_string();
        let report = DeterministicEntityRecognizer::scholarly(vec![shorter, person_pattern()])
            .recognize(b"Ada Lovelace")
            .expect("recognize fixture");
        assert_eq!(report.mentions.len(), 1);
        assert_eq!(report.mentions[0].text, "Ada Lovelace");
    }

    #[test]
    fn empty_patterns_are_rejected() {
        let mut pattern = person_pattern();
        pattern.surface.clear();
        assert!(matches!(
            DeterministicEntityRecognizer::scholarly(vec![pattern]).recognize(b"text"),
            Err(EntityModelError::EmptyPattern)
        ));
    }
}
