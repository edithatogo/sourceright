use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub const EXTRACTION_BENCHMARK_SCHEMA_VERSION: &str = "sourceright.extraction_benchmark.v1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtractionBenchmarkManifest {
    pub schema_version: String,
    pub runner_status: String,
    pub live_network: bool,
    pub redistribution: String,
    pub fixtures: Vec<ExtractionBenchmarkFixture>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtractionBenchmarkFixture {
    pub id: String,
    pub input: String,
    pub gold: String,
    pub prediction: String,
    pub source: String,
    pub license: String,
    pub access: String,
    pub content_sha256: String,
    pub split: String,
    pub cohort: ExtractionCohort,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtractionCohort {
    pub language: String,
    pub layout: String,
    pub scan_status: String,
    pub domain: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtractionStageOutput {
    pub document_id: String,
    pub backend: BackendMetadata,
    pub operations: OperationMetadata,
    pub reference_segmentation: CountMetric,
    pub field_extraction: CountMetric,
    pub callout_linking: CountMetric,
    pub coordinates: Availability,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OperationMetadata {
    pub status: String,
    pub duration_ms: u64,
    pub peak_memory_bytes: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BackendMetadata {
    pub name: String,
    pub version: String,
    pub model: String,
    pub config_fingerprint: String,
    pub hardware: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct CountMetric {
    pub true_positive: u32,
    pub false_positive: u32,
    pub false_negative: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Availability {
    Available,
    Unavailable,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExtractionBenchmarkReport {
    pub schema_version: String,
    pub manifest_path: String,
    pub fixture_count: usize,
    pub passed_count: usize,
    pub failed_count: usize,
    pub leakage_violations: Vec<String>,
    pub fixtures: Vec<FixtureReport>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FixtureReport {
    pub id: String,
    pub split: String,
    pub cohort: ExtractionCohort,
    pub backend: BackendMetadata,
    pub operations: OperationMetadata,
    pub reference_segmentation: Score,
    pub field_extraction: Score,
    pub callout_linking: Score,
    pub coordinates: Availability,
    pub passed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Score {
    pub precision: Option<f64>,
    pub recall: Option<f64>,
    pub f1: Option<f64>,
}

#[derive(Debug, thiserror::Error)]
pub enum ExtractionBenchmarkError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("extraction benchmark manifest validation error: {0}")]
    Manifest(String),
}

impl ExtractionBenchmarkManifest {
    pub fn load(path: impl AsRef<Path>) -> Result<Self, ExtractionBenchmarkError> {
        let input = fs::read_to_string(path)?;
        let manifest: Self = serde_json::from_str(&input)?;
        manifest.validate()?;
        Ok(manifest)
    }

    pub fn validate(&self) -> Result<(), ExtractionBenchmarkError> {
        if self.schema_version != EXTRACTION_BENCHMARK_SCHEMA_VERSION {
            return Err(ExtractionBenchmarkError::Manifest(format!(
                "unsupported schema version `{}`",
                self.schema_version
            )));
        }
        if self.live_network {
            return Err(ExtractionBenchmarkError::Manifest(
                "default extraction benchmark must not use live network".to_string(),
            ));
        }
        if self.fixtures.is_empty() {
            return Err(ExtractionBenchmarkError::Manifest(
                "at least one fixture is required".to_string(),
            ));
        }
        for fixture in &self.fixtures {
            for (field, value) in [
                ("id", &fixture.id),
                ("source", &fixture.source),
                ("license", &fixture.license),
                ("access", &fixture.access),
                ("content_sha256", &fixture.content_sha256),
                ("split", &fixture.split),
                ("language", &fixture.cohort.language),
                ("layout", &fixture.cohort.layout),
                ("scan_status", &fixture.cohort.scan_status),
                ("domain", &fixture.cohort.domain),
            ] {
                if value.trim().is_empty() {
                    return Err(ExtractionBenchmarkError::Manifest(format!(
                        "fixture `{}` is missing `{field}`",
                        fixture.id
                    )));
                }
            }
            if fixture.content_sha256.len() != 64
                || !fixture
                    .content_sha256
                    .chars()
                    .all(|c| c.is_ascii_hexdigit())
            {
                return Err(ExtractionBenchmarkError::Manifest(format!(
                    "fixture `{}` has an invalid SHA-256 hash",
                    fixture.id
                )));
            }
        }
        let mut hashes = HashMap::new();
        for fixture in &self.fixtures {
            if let Some(previous) = hashes.insert(&fixture.content_sha256, &fixture.split)
                && previous != &fixture.split
            {
                return Err(ExtractionBenchmarkError::Manifest(format!(
                    "content hash for fixture `{}` crosses split boundaries",
                    fixture.id
                )));
            }
        }
        Ok(())
    }
}

pub fn run_extraction_benchmark(
    manifest_path: impl AsRef<Path>,
) -> Result<ExtractionBenchmarkReport, ExtractionBenchmarkError> {
    let manifest_path = manifest_path.as_ref();
    let manifest = ExtractionBenchmarkManifest::load(manifest_path)?;
    let root = manifest_path.parent().unwrap_or_else(|| Path::new("."));
    let mut reports = Vec::with_capacity(manifest.fixtures.len());

    for fixture in &manifest.fixtures {
        let input_path = root.join(&fixture.input);
        let actual_hash = sha256_file(&input_path)?;
        if actual_hash != fixture.content_sha256 {
            return Err(ExtractionBenchmarkError::Manifest(format!(
                "fixture `{}` content hash mismatch: expected {}, got {}",
                fixture.id, fixture.content_sha256, actual_hash
            )));
        }
        let gold: ExtractionStageOutput = read_json(root.join(&fixture.gold))?;
        let prediction: ExtractionStageOutput = read_json(root.join(&fixture.prediction))?;
        if gold.document_id != fixture.id || prediction.document_id != fixture.id {
            return Err(ExtractionBenchmarkError::Manifest(format!(
                "fixture `{}` stage outputs have mismatched document IDs",
                fixture.id
            )));
        }
        reports.push(FixtureReport {
            id: fixture.id.clone(),
            split: fixture.split.clone(),
            cohort: fixture.cohort.clone(),
            backend: prediction.backend,
            operations: prediction.operations,
            reference_segmentation: score(
                gold.reference_segmentation,
                prediction.reference_segmentation,
            ),
            field_extraction: score(gold.field_extraction, prediction.field_extraction),
            callout_linking: score(gold.callout_linking, prediction.callout_linking),
            coordinates: prediction.coordinates,
            passed: true,
        });
    }

    let passed_count = reports.iter().filter(|report| report.passed).count();
    Ok(ExtractionBenchmarkReport {
        schema_version: "sourceright.extraction_benchmark_report.v1".to_string(),
        manifest_path: manifest_path.display().to_string(),
        fixture_count: reports.len(),
        passed_count,
        failed_count: reports.len() - passed_count,
        leakage_violations: Vec::new(),
        fixtures: reports,
    })
}

fn read_json<T: for<'de> Deserialize<'de>>(path: PathBuf) -> Result<T, ExtractionBenchmarkError> {
    Ok(serde_json::from_str(&fs::read_to_string(path)?)?)
}

fn sha256_file(path: &Path) -> Result<String, ExtractionBenchmarkError> {
    let mut digest = Sha256::new();
    digest.update(fs::read(path)?);
    let digest = digest.finalize();
    Ok(digest.iter().map(|byte| format!("{byte:02x}")).collect())
}

fn score(gold: CountMetric, prediction: CountMetric) -> Score {
    let true_positive = f64::from(prediction.true_positive.min(gold.true_positive));
    let predicted = f64::from(prediction.true_positive + prediction.false_positive);
    let relevant = f64::from(gold.true_positive + gold.false_negative);
    let precision = (predicted > 0.0).then(|| true_positive / predicted);
    let recall = (relevant > 0.0).then(|| true_positive / relevant);
    let f1 = match (precision, recall) {
        (Some(p), Some(r)) if p + r > 0.0 => Some(2.0 * p * r / (p + r)),
        _ => None,
    };
    Score {
        precision,
        recall,
        f1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn score_uses_gold_and_prediction_counts() {
        let result = score(
            CountMetric {
                true_positive: 3,
                false_positive: 0,
                false_negative: 1,
            },
            CountMetric {
                true_positive: 3,
                false_positive: 1,
                false_negative: 0,
            },
        );
        assert_eq!(result.precision, Some(0.75));
        assert_eq!(result.recall, Some(0.75));
        assert_eq!(result.f1, Some(0.75));
    }

    #[test]
    fn unavailable_coordinates_are_not_a_zero_score() {
        let output = ExtractionStageOutput {
            document_id: "fixture".to_string(),
            backend: BackendMetadata {
                name: "test".to_string(),
                version: "1".to_string(),
                model: "none".to_string(),
                config_fingerprint: "x".to_string(),
                hardware: "cpu".to_string(),
            },
            operations: OperationMetadata {
                status: "success".to_string(),
                duration_ms: 0,
                peak_memory_bytes: None,
            },
            reference_segmentation: CountMetric {
                true_positive: 0,
                false_positive: 0,
                false_negative: 0,
            },
            field_extraction: CountMetric {
                true_positive: 0,
                false_positive: 0,
                false_negative: 0,
            },
            callout_linking: CountMetric {
                true_positive: 0,
                false_positive: 0,
                false_negative: 0,
            },
            coordinates: Availability::Unavailable,
        };
        assert_eq!(output.coordinates, Availability::Unavailable);
        assert_eq!(
            score(output.reference_segmentation, output.reference_segmentation).f1,
            None
        );
    }
}
