use std::fs;
use std::path::Path;

use serde_json::Value;

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"))
}

fn read_json(path: &str) -> Value {
    serde_json::from_str(&read(path)).unwrap_or_else(|err| panic!("failed to parse {path}: {err}"))
}

#[test]
fn submission_inventory_exists_and_has_required_schema() {
    let inventory = read_json("conductor/submission-inventory.json");

    assert_eq!(
        inventory["schema"],
        "sourceright.conductor.submission-inventory.v1"
    );
    assert!(inventory["generated_at"].is_string());
    assert!(inventory["health_target"].as_f64().unwrap_or(0.0) >= 9.0);
    assert!(inventory["surface_count"].as_u64().unwrap_or(0) >= 10);
}

#[test]
fn submission_inventory_has_all_ten_surfaces() {
    let inventory = read_json("conductor/submission-inventory.json");
    let surfaces = inventory["surfaces"]
        .as_array()
        .expect("surfaces should be an array");

    let required_ids = [
        "73-mcp-directory-submission-hardening",
        "74-citation-manager-publication-hardening",
        "75-journal-platform-publication-hardening",
        "76-ai-client-extension-publication-hardening",
        "77-vscode-openvsx-publication-hardening",
        "78-arxiv-upstream-requirements-reconnaissance",
        "79-arxiv-submit-ce-maturity-hardening",
        "80-arxiv-submission-core-maturity-hardening",
        "81-arxiv-upstream-submission-and-acceptance",
        "82-self-improving-submission-and-health-loop",
    ];

    for required_id in required_ids {
        let surface = surfaces
            .iter()
            .find(|s| s["track_id"].as_str() == Some(required_id))
            .unwrap_or_else(|| panic!("missing surface {required_id}"));
        assert!(
            surface["readiness"]["total_gates"].as_u64().unwrap_or(0) > 0,
            "surface {required_id} has total_gates == 0"
        );
        assert!(
            surface["readiness"]["health_contribution"]
                .as_f64()
                .unwrap_or(-1.0)
                >= 0.0,
            "surface {required_id} has negative health_contribution"
        );
        assert!(
            surface["readiness"]["health_contribution"]
                .as_f64()
                .unwrap_or(2.0)
                <= 1.0,
            "surface {required_id} has health_contribution > 1.0"
        );
        assert!(
            surface["last_updated"].is_string(),
            "surface {required_id} missing last_updated"
        );

        let track_path = format!("conductor/tracks/{required_id}");
        assert!(
            Path::new(&track_path).is_dir(),
            "surface {required_id} does not map to an on-disk conductor track"
        );
        for required_file in ["metadata.json", "plan.md", "test-matrix.md"] {
            let required_path = format!("{track_path}/{required_file}");
            assert!(
                Path::new(&required_path).exists(),
                "surface {required_id} missing track file {required_file}"
            );
        }
    }

    assert_eq!(surfaces.len() as u64, 10);
}

#[test]
fn submission_inventory_health_score_is_consistent() {
    let inventory = read_json("conductor/submission-inventory.json");
    let surfaces = inventory["surfaces"]
        .as_array()
        .expect("surfaces should be an array");

    let computed_score: f64 = surfaces
        .iter()
        .map(|s| {
            s["readiness"]["health_contribution"]
                .as_f64()
                .unwrap_or(0.0)
                * 10.0
        })
        .sum::<f64>()
        / surfaces.len() as f64;

    let stored_score = inventory["health_score"].as_f64().unwrap_or(0.0);
    let diff = (computed_score - stored_score).abs();

    assert!(
        diff < 0.1,
        "health score mismatch: computed {computed_score:.2}, stored {stored_score:.2}"
    );
}

#[test]
fn submission_inventory_surfaces_have_required_readiness_fields() {
    let inventory = read_json("conductor/submission-inventory.json");
    let surfaces = inventory["surfaces"]
        .as_array()
        .expect("surfaces should be an array");

    for surface in surfaces {
        let track_id = surface["track_id"].as_str().unwrap_or("unknown");
        let readiness = &surface["readiness"];

        for field in &[
            "gates_passed",
            "total_gates",
            "evidence_level",
            "health_contribution",
        ] {
            assert!(
                readiness.get(*field).is_some(),
                "surface {track_id} readiness missing field {field}"
            );
        }

        assert!(
            readiness["gates_passed"].as_u64().unwrap_or(0)
                <= readiness["total_gates"].as_u64().unwrap_or(0),
            "surface {track_id} has gates_passed > total_gates"
        );
    }
}

#[test]
fn submission_inventory_surfaces_have_blockers() {
    let inventory = read_json("conductor/submission-inventory.json");
    let surfaces = inventory["surfaces"]
        .as_array()
        .expect("surfaces should be an array");

    for surface in surfaces {
        let track_id = surface["track_id"].as_str().unwrap_or("unknown");
        let blockers = surface["blockers"].as_array();

        assert!(
            blockers.is_some(),
            "surface {track_id} missing blockers array"
        );
    }
}

#[test]
fn submission_health_readiness_script_and_ci_are_wired() {
    let workflow = read(".github/workflows/submission-readiness.yml");
    let script = read("scripts/check-submission-readiness.ps1");
    let local_windows_gnu = read("scripts/verify-local-windows-gnu.ps1");

    assert!(workflow.contains("scripts/check-submission-readiness.ps1"));
    assert!(workflow.contains("cargo test --locked --test submission_health_policy"));
    assert!(workflow.contains("persist-credentials: false"));

    assert!(script.contains("schema_valid"));
    assert!(script.contains("surface_count"));
    assert!(script.contains("MinimumEvidenceLevel"));
    assert!(script.contains("conductor\\tracks\\$trackId"));
    assert!(script.contains("health_contribution"));
    assert!(script.contains("gates_passed / total_gates"));
    assert!(script.contains("ConvertTo-Json -Depth 10"));

    assert!(local_windows_gnu.contains("verify-submission-readiness.ps1"));
}

#[test]
fn submission_health_meets_minimum_threshold_when_gated() {
    if std::env::var("SOURCERIGHT_CLAIM_GATE").as_deref() == Ok("1") {
        let inventory = read_json("conductor/submission-inventory.json");
        let health_score = inventory["health_score"].as_f64().unwrap_or(0.0);
        assert!(
            health_score >= 9.5,
            "health score {health_score} is below gated threshold 9.5"
        );
    }
}
