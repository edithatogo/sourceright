use serde_json::Value;
use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"))
}

fn read_json(path: &str) -> Value {
    serde_json::from_str(&read(path)).unwrap_or_else(|err| panic!("failed to parse {path}: {err}"))
}

#[test]
fn platform_registry_has_structured_candidate_tracks_for_every_platform() {
    let registry = read_json("conductor/platform-registry.json");
    let schema = read_json("conductor/platform-registry.schema.json");

    assert_eq!(
        registry["$schema"],
        "sourceright.conductor.platform-registry.v1"
    );
    assert_eq!(
        registry["schema_version"],
        "sourceright.conductor.platform_registry.v1"
    );
    assert_eq!(schema["$id"], "sourceright.conductor.platform-registry.v1");
    assert_eq!(
        schema["$schema"],
        "https://json-schema.org/draft/2020-12/schema"
    );

    let policy = &registry["policy"];
    assert_eq!(policy["candidate_track_required"], true);
    assert_eq!(policy["ad_hoc_notes_allowed"], false);
    assert_eq!(policy["human_review_required"], true);
    assert_eq!(policy["auto_open_tracks"], false);

    let required_candidate_fields = registry["candidate_track_template"]["required_fields"]
        .as_array()
        .expect("candidate template should list required fields")
        .iter()
        .map(|value| value.as_str().expect("required field should be string"))
        .collect::<Vec<_>>();
    assert!(required_candidate_fields.contains(&"track_slug"));
    assert!(required_candidate_fields.contains(&"acceptance_gates"));

    let platforms = registry["platforms"]
        .as_array()
        .expect("platforms should be an array");
    assert!(
        platforms.len() >= 5,
        "initial registry should cover Janeway and proprietary matrix platforms"
    );

    let mut platform_ids = BTreeSet::new();
    let mut candidate_slugs = BTreeSet::new();

    for platform in platforms {
        let platform_id = platform["platform_id"]
            .as_str()
            .expect("platform_id should be a string");
        assert!(
            platform_ids.insert(platform_id.to_string()),
            "duplicate platform_id {platform_id}"
        );

        let owner_track = platform["owner_track"]
            .as_str()
            .expect("owner_track should be a string");
        assert!(
            Path::new("conductor/tracks").join(owner_track).is_dir(),
            "owner_track should exist for {platform_id}: {owner_track}"
        );

        assert_eq!(platform["approval_state"], "needs-human-review");
        let confidence = platform["confidence"]
            .as_f64()
            .expect("confidence should be numeric");
        assert!((0.0..=1.0).contains(&confidence));
        assert!(
            platform["evidence_sources"]
                .as_array()
                .is_some_and(|sources| !sources.is_empty()),
            "{platform_id} should carry evidence sources"
        );
        assert!(
            platform["blockers"]
                .as_array()
                .is_some_and(|blockers| !blockers.is_empty()),
            "{platform_id} should carry blockers"
        );

        let candidate = &platform["candidate_track"];
        for field in &required_candidate_fields {
            assert!(
                candidate.get(*field).is_some(),
                "{platform_id} candidate track missing {field}"
            );
        }
        let track_slug = candidate["track_slug"]
            .as_str()
            .expect("track_slug should be a string");
        assert!(track_slug.starts_with("candidate-"));
        assert!(
            candidate_slugs.insert(track_slug.to_string()),
            "duplicate candidate slug {track_slug}"
        );
        assert_eq!(candidate["proposed_status"], "candidate");
        assert_eq!(candidate["human_review_required"], true);
        assert_eq!(candidate["auto_open"], false);
        assert!(
            candidate["scope"]
                .as_array()
                .is_some_and(|scope| !scope.is_empty()),
            "{platform_id} candidate should include scope"
        );
        assert!(
            candidate["acceptance_gates"]
                .as_array()
                .is_some_and(|gates| !gates.is_empty()),
            "{platform_id} candidate should include acceptance gates"
        );
    }

    for required in [
        "janeway",
        "scholarone",
        "editorial-manager",
        "ejournalpress",
        "manuscript-manager",
    ] {
        assert!(
            platform_ids.contains(required),
            "missing platform {required}"
        );
    }
}

#[test]
fn platform_registry_workflow_emits_candidates_without_opening_tracks() {
    let script = read("scripts/propose-platform-track-candidates.ps1");
    let spec = read("conductor/tracks/87-self-improving-platform-registry/spec.md");
    let plan = read("conductor/tracks/87-self-improving-platform-registry/plan.md");
    let metadata = read("conductor/tracks/87-self-improving-platform-registry/metadata.json");
    let matrix = read("conductor/tracks/87-self-improving-platform-registry/test-matrix.md");

    assert!(script.contains("candidate_tracks"));
    assert!(script.contains("ad hoc notes"));
    assert!(script.contains("auto-open tracks"));
    assert!(script.contains("auto_open -ne $false"));
    assert!(!script.contains("git commit"));
    assert!(!script.contains("git push"));
    assert!(!script.contains("New-Item -ItemType Directory"));

    assert!(spec.contains("conductor/platform-registry.json"));
    assert!(spec.contains("scripts/propose-platform-track-candidates.ps1"));
    assert!(spec.contains("structured candidate track"));
    assert!(spec.contains("The registry can suggest, but humans still approve."));

    assert!(plan.contains("Turn the registry into the source of truth"));
    assert!(metadata.contains("conductor/platform-registry.json"));
    assert!(metadata.contains("tests/platform_registry_policy.rs"));
    assert!(matrix.contains("structured candidate track"));
    assert!(matrix.contains("no ad hoc notes"));
}
