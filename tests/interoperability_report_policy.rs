use std::fs;

#[test]
fn report_schema_and_runner_contract_are_present() {
    let schema: serde_json::Value = serde_json::from_str(
        &fs::read_to_string("schemas/sourceright.interoperability-report.schema.json")
            .expect("interoperability report schema"),
    )
    .expect("valid report schema");
    assert_eq!(
        schema["$id"],
        "https://edithatogo.github.io/sourceright/schemas/sourceright.interoperability-report.schema.json"
    );

    let runners = fs::read_to_string(
        "conductor/tracks/90-citation-ecosystem-differential-interoperability/optional-runners.toml",
    )
    .expect("optional runner contract");
    assert!(runners.contains("status = \"enabled-local-fixture-smoke\""));
    assert!(runners.contains("status = \"deferred-until-lockfile-and-license-evidence\""));
    assert!(runners.contains("citation-js"));
    assert!(runners.contains("biblatex-csl-converter"));
    assert!(runners.contains("retorquere-bibtex-parser"));
}
