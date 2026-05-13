use std::fs;
use std::path::Path;

fn read(path: &str) -> String {
    fs::read_to_string(path).expect("fixture should be present")
}

fn schema_files() -> Vec<String> {
    let mut schemas = fs::read_dir("schemas")
        .expect("schemas directory should be readable")
        .map(|entry| {
            let entry = entry.expect("schema directory entry should be readable");
            let path = entry.path();
            assert_eq!(
                path.extension().and_then(|value| value.to_str()),
                Some("json"),
                "schema files should be JSON: {}",
                path.display()
            );
            path.file_name()
                .and_then(|value| value.to_str())
                .expect("schema file name should be UTF-8")
                .to_string()
        })
        .collect::<Vec<_>>();
    schemas.sort();
    schemas
}

#[test]
fn schema_contract_files_are_valid_json_and_packaged() {
    let cargo_toml = read("Cargo.toml");
    assert!(
        cargo_toml.contains("\"/schemas/**\""),
        "Cargo package include list should ship schema contracts"
    );

    for schema in schema_files() {
        let path = Path::new("schemas").join(&schema);
        let content = fs::read_to_string(&path).expect("schema should be readable");
        let parsed: serde_json::Value =
            serde_json::from_str(&content).expect("schema should be valid JSON");

        assert_eq!(
            parsed["$schema"], "https://json-schema.org/draft/2020-12/schema",
            "schema should declare the draft 2020-12 dialect: {schema}"
        );
        assert!(
            parsed["title"]
                .as_str()
                .is_some_and(|value| !value.is_empty()),
            "schema should have a non-empty title: {schema}"
        );
    }
}

#[test]
fn schema_contract_docs_name_every_public_schema() {
    let mdbook = read("docs/src/schema-contracts.md");
    let docs_site = read("docs-site/src/content/docs/schema-contracts.md");

    for schema in schema_files() {
        assert!(
            mdbook.contains(&schema),
            "mdBook schema-contracts page should mention {schema}"
        );
        assert!(
            docs_site.contains(&schema),
            "docs-site schema-contracts page should mention {schema}"
        );
    }
}
