use std::collections::HashSet;
use std::fs;

#[test]
fn registry_covers_each_local_trial_track_once() {
    let registry: serde_json::Value = serde_json::from_str(
        &fs::read_to_string("conductor/open-publishing-platform-registry.json")
            .expect("platform registry"),
    )
    .expect("valid platform registry");
    assert_eq!(
        registry["schema_version"],
        "sourceright.open-publishing-platform-registry.v1"
    );
    assert_eq!(registry["trial_contract"], "read-only-fixture-backed-v1");

    let platforms = registry["platforms"].as_array().expect("platform array");
    let ids: HashSet<_> = platforms
        .iter()
        .map(|p| p["id"].as_str().unwrap())
        .collect();
    let tracks: HashSet<_> = platforms
        .iter()
        .map(|p| p["track"].as_u64().unwrap())
        .collect();
    assert_eq!(platforms.len(), 10);
    assert_eq!(ids.len(), 10);
    assert_eq!(tracks, (98..=107).map(|n| n as u64).collect());
    assert!(platforms.iter().all(|platform| {
        platform["upstream"]
            .as_str()
            .unwrap()
            .starts_with("https://github.com/")
    }));
}

#[test]
fn registry_keeps_deferred_hosted_or_unverified_surfaces_explicit() {
    let registry: serde_json::Value = serde_json::from_str(
        &fs::read_to_string("conductor/open-publishing-platform-registry.json")
            .expect("platform registry"),
    )
    .expect("valid platform registry");
    let deferred = registry["deferred"].as_array().expect("deferred array");
    for id in ["osf-preprints", "orvium", "preprints.org"] {
        assert!(deferred.iter().any(|value| value == id));
    }
}
