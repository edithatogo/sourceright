use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::csl::{CslDocument, CslItem};

pub const EXPORT_SCHEMA_VERSION: &str = "sourceright.export.v1";
pub const EXPORT_MANIFEST_SCHEMA_VERSION: &str = "sourceright.export_manifest.v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExportFormat {
    Yaml,
    Xml,
    Ris,
    Enw,
    Biblatex,
}

impl ExportFormat {
    pub fn filename(self) -> &'static str {
        match self {
            Self::Yaml => "references.yaml",
            Self::Xml => "references.xml",
            Self::Ris => "references.ris",
            Self::Enw => "references.enw",
            Self::Biblatex => "references.bib",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value.to_ascii_lowercase().as_str() {
            "yaml" | "yml" => Some(Self::Yaml),
            "xml" => Some(Self::Xml),
            "ris" => Some(Self::Ris),
            "enw" | "endnote" => Some(Self::Enw),
            "biblatex" | "bib" => Some(Self::Biblatex),
            _ => None,
        }
    }

    pub fn content_type(self) -> &'static str {
        match self {
            Self::Yaml => "application/x-yaml",
            Self::Xml => "application/xml",
            Self::Ris => "application/x-research-info-systems",
            Self::Enw => "application/x-endnote-refer",
            Self::Biblatex => "application/x-bibtex",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExportArtifact {
    pub format: ExportFormat,
    pub filename: String,
    pub content: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExportManifest {
    pub schema_version: String,
    pub source: ExportManifestSource,
    pub artifacts: Vec<ExportManifestArtifact>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExportManifestSource {
    pub references_csl_json: String,
    pub verification_sidecar_json: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExportManifestArtifact {
    pub format: ExportFormat,
    pub filename: String,
    pub content_type: String,
    pub schema_version: String,
}

pub fn export_document(document: &CslDocument, format: ExportFormat) -> ExportArtifact {
    let content = match format {
        ExportFormat::Yaml => export_yaml(document),
        ExportFormat::Xml => export_xml(document),
        ExportFormat::Ris => export_ris(document),
        ExportFormat::Enw => export_enw(document),
        ExportFormat::Biblatex => export_biblatex(document),
    };

    ExportArtifact {
        format,
        filename: format.filename().to_string(),
        content,
    }
}

pub fn export_suite(document: &CslDocument) -> Vec<ExportArtifact> {
    [
        ExportFormat::Yaml,
        ExportFormat::Xml,
        ExportFormat::Ris,
        ExportFormat::Enw,
        ExportFormat::Biblatex,
    ]
    .into_iter()
    .map(|format| export_document(document, format))
    .collect()
}

fn export_yaml(document: &CslDocument) -> String {
    let mut output = format!("schema_version: {EXPORT_SCHEMA_VERSION}\nreferences:\n");
    for item in &document.items {
        output.push_str(&format!("  - id: {}\n", yaml_scalar(&item.id)));
        output.push_str(&format!("    type: {}\n", yaml_scalar(&item.item_type)));
        if let Some(title) = &item.title {
            output.push_str(&format!("    title: {}\n", yaml_scalar(title)));
        }
        if let Some(doi) = &item.doi {
            output.push_str(&format!("    DOI: {}\n", yaml_scalar(doi)));
        }
        append_yaml_extra(&mut output, item);
    }
    output
}

fn export_xml(document: &CslDocument) -> String {
    let mut output = format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<references schema-version=\"{EXPORT_SCHEMA_VERSION}\">\n"
    );
    for item in &document.items {
        output.push_str(&format!(
            "  <reference id=\"{}\" type=\"{}\">\n",
            xml_escape(&item.id),
            xml_escape(&item.item_type)
        ));
        if let Some(title) = &item.title {
            output.push_str(&format!("    <title>{}</title>\n", xml_escape(title)));
        }
        if let Some(doi) = &item.doi {
            output.push_str(&format!("    <doi>{}</doi>\n", xml_escape(doi)));
        }
        for (key, value) in &item.extra {
            if internal_field(key) {
                continue;
            }
            if let Some(text) = value.as_str() {
                output.push_str(&format!(
                    "    <field name=\"{}\">{}</field>\n",
                    xml_escape(key),
                    xml_escape(text)
                ));
            }
        }
        output.push_str("  </reference>\n");
    }
    output.push_str("</references>\n");
    output
}

fn export_ris(document: &CslDocument) -> String {
    let mut output = String::new();
    for item in &document.items {
        output.push_str(&format!("TY  - {}\n", ris_type(&item.item_type)));
        output.push_str(&format!("ID  - {}\n", item.id));
        if let Some(title) = &item.title {
            output.push_str(&format!("TI  - {title}\n"));
        }
        if let Some(container) = string_extra(item, "container-title") {
            output.push_str(&format!("JO  - {container}\n"));
        }
        if let Some(pages) = string_extra(item, "page") {
            output.push_str(&format!("SP  - {pages}\n"));
        }
        if let Some(doi) = &item.doi {
            output.push_str(&format!("DO  - {doi}\n"));
        }
        if let Some(url) = string_extra(item, "URL").or_else(|| string_extra(item, "url")) {
            output.push_str(&format!("UR  - {url}\n"));
        }
        output.push_str("ER  - \n");
    }
    output
}

fn export_enw(document: &CslDocument) -> String {
    let mut output = String::new();
    for item in &document.items {
        output.push_str(&format!("%0 {}\n", enw_type(&item.item_type)));
        output.push_str(&format!("%F {}\n", item.id));
        if let Some(title) = &item.title {
            output.push_str(&format!("%T {title}\n"));
        }
        if let Some(container) = string_extra(item, "container-title") {
            output.push_str(&format!("%J {container}\n"));
        }
        if let Some(doi) = &item.doi {
            output.push_str(&format!("%R {doi}\n"));
        }
        if let Some(url) = string_extra(item, "URL").or_else(|| string_extra(item, "url")) {
            output.push_str(&format!("%U {url}\n"));
        }
        output.push('\n');
    }
    output
}

fn export_biblatex(document: &CslDocument) -> String {
    let mut output = String::new();
    for item in &document.items {
        output.push_str(&format!(
            "@{}{{{},\n",
            biblatex_type(&item.item_type),
            bib_key(&item.id)
        ));
        output.push_str(&format!("  ids = {{{}}},\n", bib_escape(&item.id)));
        if let Some(title) = &item.title {
            output.push_str(&format!("  title = {{{}}},\n", bib_escape(title)));
        }
        if let Some(container) = string_extra(item, "container-title") {
            let field = if item.item_type == "article-journal" {
                "journaltitle"
            } else {
                "booktitle"
            };
            output.push_str(&format!("  {field} = {{{}}},\n", bib_escape(container)));
        }
        if let Some(pages) = string_extra(item, "page") {
            output.push_str(&format!("  pages = {{{}}},\n", bib_escape(pages)));
        }
        if let Some(doi) = &item.doi {
            output.push_str(&format!("  doi = {{{}}},\n", bib_escape(doi)));
        }
        if let Some(url) = string_extra(item, "URL").or_else(|| string_extra(item, "url")) {
            output.push_str(&format!("  url = {{{}}},\n", bib_escape(url)));
        }
        trim_trailing_comma(&mut output);
        output.push_str("}\n\n");
    }
    output
}

fn append_yaml_extra(output: &mut String, item: &CslItem) {
    for (key, value) in &item.extra {
        if internal_field(key) {
            continue;
        }
        match value {
            Value::String(text) => output.push_str(&format!("    {key}: {}\n", yaml_scalar(text))),
            Value::Number(number) => output.push_str(&format!("    {key}: {number}\n")),
            Value::Bool(value) => output.push_str(&format!("    {key}: {value}\n")),
            Value::Array(_) | Value::Object(_) => {
                output.push_str(&format!("    {key}: {}\n", yaml_scalar(&value.to_string())));
            }
            Value::Null => {}
        }
    }
}

fn string_extra<'a>(item: &'a CslItem, key: &str) -> Option<&'a str> {
    item.extra.get(key).and_then(Value::as_str)
}

fn internal_field(key: &str) -> bool {
    matches!(
        key,
        "confidence" | "provider" | "provider_candidates" | "review_status" | "review_decisions"
    )
}

fn yaml_scalar(value: &str) -> String {
    format!("\"{}\"", value.replace('\\', "\\\\").replace('"', "\\\""))
}

fn xml_escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

fn bib_escape(value: &str) -> String {
    value
        .replace('\\', "\\textbackslash{}")
        .replace('{', "\\{")
        .replace('}', "\\}")
        .replace('&', "\\&")
        .replace('%', "\\%")
}

fn ris_type(item_type: &str) -> &'static str {
    match item_type {
        "article-journal" => "JOUR",
        "book" => "BOOK",
        "chapter" => "CHAP",
        "webpage" | "post-weblog" => "ELEC",
        "thesis" => "THES",
        "report" => "RPRT",
        _ => "GEN",
    }
}

fn enw_type(item_type: &str) -> &'static str {
    match item_type {
        "article-journal" => "Journal Article",
        "book" => "Book",
        "chapter" => "Book Section",
        "webpage" | "post-weblog" => "Web Page",
        "thesis" => "Thesis",
        "report" => "Report",
        _ => "Generic",
    }
}

fn biblatex_type(item_type: &str) -> &'static str {
    match item_type {
        "article-journal" => "article",
        "book" => "book",
        "chapter" => "inbook",
        "webpage" | "post-weblog" => "online",
        "thesis" => "thesis",
        "report" => "report",
        _ => "misc",
    }
}

fn bib_key(id: &str) -> String {
    id.chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' {
                ch
            } else {
                '-'
            }
        })
        .collect()
}

fn trim_trailing_comma(output: &mut String) {
    if let Some(position) = output.rfind(",\n") {
        output.replace_range(position..position + 2, "\n");
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;

    fn fixture() -> CslDocument {
        CslDocument {
            items: vec![
                CslItem {
                    id: "smith-2024".to_string(),
                    item_type: "article-journal".to_string(),
                    title: Some("A & B <Trial>".to_string()),
                    doi: Some("10.1000/example".to_string()),
                    extra: BTreeMap::from([
                        (
                            "container-title".to_string(),
                            Value::String("Journal of Tests".to_string()),
                        ),
                        ("page".to_string(), Value::String("1-4".to_string())),
                        (
                            "confidence".to_string(),
                            Value::String("internal".to_string()),
                        ),
                    ]),
                },
                CslItem {
                    id: "web-2025".to_string(),
                    item_type: "webpage".to_string(),
                    title: Some("Web Reference".to_string()),
                    doi: None,
                    extra: BTreeMap::from([(
                        "URL".to_string(),
                        Value::String("https://example.test".to_string()),
                    )]),
                },
            ],
        }
    }

    #[test]
    fn full_suite_generates_stable_filenames_and_order() {
        let artifacts = export_suite(&fixture());

        assert_eq!(
            artifacts
                .iter()
                .map(|artifact| artifact.filename.as_str())
                .collect::<Vec<_>>(),
            [
                "references.yaml",
                "references.xml",
                "references.ris",
                "references.enw",
                "references.bib",
            ]
        );
        assert!(
            artifacts
                .iter()
                .all(|artifact| artifact.content.contains("smith-2024"))
        );
    }

    #[test]
    fn clean_exports_omit_internal_metadata() {
        for artifact in export_suite(&fixture()) {
            assert!(!artifact.content.contains("confidence"));
            assert!(!artifact.content.contains("internal"));
        }
    }

    #[test]
    fn xml_ris_enw_biblatex_and_yaml_have_expected_structure() {
        let document = fixture();

        let xml = export_document(&document, ExportFormat::Xml).content;
        assert!(xml.contains("<references schema-version=\"sourceright.export.v1\">"));
        assert!(xml.contains("<title>A &amp; B &lt;Trial&gt;</title>"));

        let ris = export_document(&document, ExportFormat::Ris).content;
        assert_eq!(ris.matches("TY  - ").count(), 2);
        assert_eq!(ris.matches("ER  - ").count(), 2);
        assert!(ris.contains("DO  - 10.1000/example"));

        let enw = export_document(&document, ExportFormat::Enw).content;
        assert_eq!(enw.matches("%0 ").count(), 2);
        assert!(enw.contains("%R 10.1000/example"));

        let bib = export_document(&document, ExportFormat::Biblatex).content;
        assert!(bib.contains("@article{smith-2024,"));
        assert!(bib.contains("title = {A \\& B <Trial>}"));

        let yaml = export_document(&document, ExportFormat::Yaml).content;
        assert!(yaml.starts_with("schema_version: sourceright.export.v1"));
        assert_eq!(yaml.matches("  - id: ").count(), 2);
    }

    #[test]
    fn ris_export_matches_endnote_handoff_fixture() {
        let document = fixture();
        let ris = export_document(&document, ExportFormat::Ris).content;
        let expected = include_str!("../fixtures/export/endnote-export.ris");
        let normalize = |content: &str| {
            content
                .replace("\r\n", "\n")
                .lines()
                .map(str::trim_end)
                .collect::<Vec<_>>()
                .join("\n")
        };
        let ris_normalized = normalize(&ris);
        let expected_normalized = normalize(expected);
        assert_eq!(
            ris_normalized.trim(),
            expected_normalized.trim(),
            "RIS export did not match endnote-export.ris fixture"
        );
    }

    #[test]
    fn enw_export_matches_endnote_handoff_fixture() {
        let document = fixture();
        let enw = export_document(&document, ExportFormat::Enw).content;
        let expected = include_str!("../fixtures/export/endnote-export.enw");
        let enw_normalized = enw.replace("\r\n", "\n");
        let expected_normalized = expected.replace("\r\n", "\n");
        assert_eq!(
            enw_normalized.trim(),
            expected_normalized.trim(),
            "ENW export did not match endnote-export.enw fixture"
        );
    }
}
