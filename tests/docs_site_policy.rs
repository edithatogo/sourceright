use std::fs;

fn read(path: &str) -> String {
    fs::read_to_string(path).expect("fixture should be present")
}

#[test]
fn docs_site_uses_starlight_and_builds_for_pages() {
    let package: serde_json::Value =
        serde_json::from_str(&read("docs-site/package.json")).expect("parse docs-site package");
    let deps = package
        .get("dependencies")
        .and_then(|value| value.as_object())
        .expect("docs-site dependencies");

    assert!(deps.contains_key("astro"));
    assert!(deps.contains_key("@astrojs/starlight"));

    let astro = read("docs-site/astro.config.mjs");
    let pages = read(".github/workflows/pages.yml");

    assert!(astro.contains("starlight"));
    assert!(astro.contains("/sourceright/"));
    assert!(pages.contains("docs-site/dist"));
    assert!(pages.contains("npm run build"));
}
