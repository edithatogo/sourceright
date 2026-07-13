//! Optional, safe-by-default GROBID reference extraction backend.

use std::net::IpAddr;
use std::time::Duration;

use reqwest::StatusCode;
use reqwest::blocking::{Client, multipart};
use roxmltree::{Document, Node};
use sha2::{Digest, Sha256};
use thiserror::Error;

use crate::citeweft::{
    BackendCapabilities, Diagnostic, DiagnosticSeverity, EngineProvenance, ExtractionError,
    ExtractionOptions, Identifier, ReferenceRecord, SCHEMA_VERSION, ScholarlyDocument,
    ScholarlyDocumentExtractor, TextSpan,
};

const DEFAULT_BASE_URL: &str = "http://127.0.0.1:8070";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GrobidConfig {
    pub enabled: bool,
    pub base_url: String,
    pub allow_remote: bool,
    pub allowed_remote_hosts: Vec<String>,
    pub timeout_seconds: u64,
    pub max_document_bytes: usize,
    pub max_response_bytes: usize,
    pub max_retries: u8,
    pub engine_version: Option<String>,
}

impl Default for GrobidConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            base_url: DEFAULT_BASE_URL.to_string(),
            allow_remote: false,
            allowed_remote_hosts: Vec::new(),
            timeout_seconds: 120,
            max_document_bytes: 64 * 1024 * 1024,
            max_response_bytes: 16 * 1024 * 1024,
            max_retries: 2,
            engine_version: None,
        }
    }
}

impl GrobidConfig {
    pub fn validate(&self) -> Result<reqwest::Url, GrobidError> {
        let url = reqwest::Url::parse(&self.base_url)
            .map_err(|error| GrobidError::UnsafeEndpoint(error.to_string()))?;
        if !self.allow_remote && !is_local_url(&url) {
            return Err(GrobidError::UnsafeEndpoint(
                "remote GROBID endpoints require allow_remote=true".to_string(),
            ));
        }
        if self.allow_remote && !is_local_url(&url) && url.scheme() != "https" {
            return Err(GrobidError::UnsafeEndpoint(
                "remote GROBID endpoints must use HTTPS".to_string(),
            ));
        }
        if self.allow_remote
            && !is_local_url(&url)
            && !self
                .allowed_remote_hosts
                .iter()
                .any(|host| host.eq_ignore_ascii_case(url.host_str().unwrap_or_default()))
        {
            return Err(GrobidError::UnsafeEndpoint(
                "remote GROBID host is not in the explicit allowlist".to_string(),
            ));
        }
        Ok(url)
    }
}

fn is_local_url(url: &reqwest::Url) -> bool {
    match url.host_str() {
        Some("localhost") => true,
        Some(host) => host.parse::<IpAddr>().is_ok_and(|ip| match ip {
            IpAddr::V4(address) => {
                address.is_loopback() || address.is_private() || address.is_link_local()
            }
            IpAddr::V6(address) => address.is_loopback() || address.is_unique_local(),
        }),
        None => false,
    }
}

#[derive(Debug, Error)]
pub enum GrobidError {
    #[error("GROBID extraction is disabled")]
    Disabled,
    #[error("unsafe GROBID endpoint: {0}")]
    UnsafeEndpoint(String),
    #[error("document exceeds configured byte limit")]
    DocumentTooLarge,
    #[error("GROBID returned no extractable content")]
    NoContent,
    #[error("GROBID remained overloaded after bounded retries")]
    Overloaded,
    #[error("GROBID response exceeds configured byte limit")]
    ResponseTooLarge,
    #[error("GROBID connection or timeout failure: {0}")]
    Transport(String),
    #[error("GROBID returned HTTP {0}")]
    Http(StatusCode),
    #[error("malformed GROBID TEI: {0}")]
    MalformedTei(String),
}

pub struct GrobidExtractor {
    pub config: GrobidConfig,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct GrobidHealth {
    pub reachable: bool,
    pub health: Option<String>,
    pub version: Option<String>,
}

impl GrobidExtractor {
    pub fn extract_references(&self, pdf: &[u8]) -> Result<ScholarlyDocument, GrobidError> {
        if !self.config.enabled {
            return Err(GrobidError::Disabled);
        }
        if pdf.len() > self.config.max_document_bytes {
            return Err(GrobidError::DocumentTooLarge);
        }
        let endpoint = endpoint_for(&self.config, "api/processReferences")?;
        let client = Client::builder()
            .timeout(Duration::from_secs(self.config.timeout_seconds))
            .build()
            .map_err(|error| GrobidError::Transport(error.to_string()))?;

        for attempt in 0..=self.config.max_retries {
            let form = reference_request(pdf);
            let response = client
                .post(endpoint.clone())
                .multipart(form)
                .send()
                .map_err(|error| GrobidError::Transport(error.to_string()))?;
            if response.status() == StatusCode::NO_CONTENT {
                return Err(GrobidError::NoContent);
            }
            if response.status() == StatusCode::SERVICE_UNAVAILABLE {
                if attempt == self.config.max_retries {
                    return Err(GrobidError::Overloaded);
                }
                continue;
            }
            if !response.status().is_success() {
                return Err(GrobidError::Http(response.status()));
            }
            if response
                .content_length()
                .is_some_and(|size| size > self.config.max_response_bytes as u64)
            {
                return Err(GrobidError::ResponseTooLarge);
            }
            let bytes = response
                .bytes()
                .map_err(|error| GrobidError::Transport(error.to_string()))?;
            if bytes.len() > self.config.max_response_bytes {
                return Err(GrobidError::ResponseTooLarge);
            }
            let text = std::str::from_utf8(&bytes)
                .map_err(|error| GrobidError::MalformedTei(error.to_string()))?;
            let mut document = decode_references_tei(text, self.config.engine_version.clone())?;
            document.provenance.input_hash = Some(sha256_hex(pdf));
            document.provenance.endpoint_class = Some(endpoint_class(&endpoint));
            return Ok(document);
        }
        Err(GrobidError::Overloaded)
    }

    pub fn health(&self) -> Result<GrobidHealth, GrobidError> {
        if !self.config.enabled {
            return Err(GrobidError::Disabled);
        }
        let health_url = endpoint_for(&self.config, "api/health")?;
        let version_url = endpoint_for(&self.config, "api/version")?;
        let client = Client::builder()
            .timeout(Duration::from_secs(self.config.timeout_seconds))
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .map_err(|error| GrobidError::Transport(error.to_string()))?;
        let health_response = client
            .get(health_url)
            .send()
            .map_err(|error| GrobidError::Transport(error.to_string()))?;
        let version_response = client
            .get(version_url)
            .send()
            .map_err(|error| GrobidError::Transport(error.to_string()))?;
        Ok(GrobidHealth {
            reachable: health_response.status().is_success()
                && version_response.status().is_success(),
            health: Some(health_response.status().to_string()),
            version: version_response
                .text()
                .ok()
                .map(|text| text.trim().to_string())
                .filter(|text| !text.is_empty()),
        })
    }
}

fn endpoint_for(config: &GrobidConfig, suffix: &str) -> Result<reqwest::Url, GrobidError> {
    let mut endpoint = config.validate()?;
    let base = endpoint.path().trim_end_matches('/').to_string();
    endpoint.set_path(&format!("{base}/{suffix}"));
    Ok(endpoint)
}

fn reference_request(pdf: &[u8]) -> multipart::Form {
    multipart::Form::new()
        .text("includeRawCitations", "1")
        .text("consolidateCitations", "0")
        .part(
            "input",
            multipart::Part::bytes(pdf.to_vec()).file_name("document.pdf"),
        )
}

fn endpoint_class(endpoint: &reqwest::Url) -> String {
    if is_local_url(endpoint) {
        "local".to_string()
    } else {
        "remote_allowlisted".to_string()
    }
}

fn sha256_hex(input: &[u8]) -> String {
    Sha256::digest(input)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

impl ScholarlyDocumentExtractor for GrobidExtractor {
    fn capabilities(&self) -> BackendCapabilities {
        BackendCapabilities {
            references: true,
            callouts: false,
            coordinates: false,
            entities: false,
        }
    }
    fn extract(
        &self,
        bytes: &[u8],
        options: &ExtractionOptions,
    ) -> Result<ScholarlyDocument, ExtractionError> {
        if !options.include_references || options.include_entities {
            return Err(ExtractionError::UnsupportedConfiguration(
                "initial GROBID slice supports references only".to_string(),
            ));
        }
        self.extract_references(bytes)
            .map_err(|error| ExtractionError::Backend(error.to_string()))
    }
}

pub fn decode_references_tei(
    tei: &str,
    engine_version: Option<String>,
) -> Result<ScholarlyDocument, GrobidError> {
    let document =
        Document::parse(tei).map_err(|error| GrobidError::MalformedTei(error.to_string()))?;
    let mut diagnostics = Vec::new();
    let references = document
        .descendants()
        .filter(|node| node.has_tag_name("biblStruct"))
        .enumerate()
        .map(|(index, node)| decode_reference(node, index, &mut diagnostics))
        .collect();
    Ok(ScholarlyDocument {
        schema_version: SCHEMA_VERSION.to_string(),
        references,
        diagnostics,
        provenance: EngineProvenance {
            backend: "grobid".to_string(),
            engine_version,
            configuration: "processReferences;includeRawCitations=1;consolidateCitations=0"
                .to_string(),
            input_hash: None,
            endpoint_class: None,
        },
    })
}

fn decode_reference(
    node: Node<'_, '_>,
    index: usize,
    diagnostics: &mut Vec<Diagnostic>,
) -> ReferenceRecord {
    let id = node
        .attribute(("http://www.w3.org/XML/1998/namespace", "id"))
        .or_else(|| node.attribute("id"))
        .map(str::to_string)
        .unwrap_or_else(|| format!("grobid-ref-{:04}", index + 1));
    let raw_text = node
        .descendants()
        .find(|n| n.has_tag_name("note") && n.attribute("type") == Some("raw_reference"))
        .and_then(|n| n.text())
        .unwrap_or_default()
        .trim()
        .to_string();
    if raw_text.is_empty() {
        diagnostics.push(Diagnostic { code: "grobid.raw_reference.missing".to_string(), severity: DiagnosticSeverity::Warning,
            message: "GROBID TEI reference has no raw citation text; parsed fields remain review evidence only.".to_string(), reference_id: Some(id.clone()) });
    }
    let authors = node
        .descendants()
        .filter(|n| n.has_tag_name("author"))
        .filter_map(author_name)
        .collect();
    let identifiers = node
        .descendants()
        .filter(|n| n.has_tag_name("idno"))
        .filter_map(|n| {
            let value = n.text()?.trim();
            if value.is_empty() {
                None
            } else {
                Some(Identifier {
                    scheme: n
                        .attribute("type")
                        .unwrap_or("unknown")
                        .to_ascii_lowercase(),
                    value: value.to_string(),
                })
            }
        })
        .collect();
    ReferenceRecord {
        id: id.clone(),
        raw_text,
        title: text_of(node, "title", Some(("level", "a")))
            .or_else(|| text_of(node, "title", None)),
        authors,
        container_title: text_of(node, "title", Some(("level", "j"))),
        publication_date: node
            .descendants()
            .find(|n| n.has_tag_name("date"))
            .and_then(|n| n.attribute("when").or_else(|| n.text()))
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(str::to_string),
        volume: scope_value(node, "volume"),
        issue: scope_value(node, "issue"),
        pages: pages_value(node),
        identifiers,
        span: Some(TextSpan {
            surface: "grobid_tei".to_string(),
            source_id: Some(id.clone()),
        }),
    }
}

fn text_of(node: Node<'_, '_>, tag: &str, attribute: Option<(&str, &str)>) -> Option<String> {
    node.descendants()
        .find(|n| n.has_tag_name(tag) && attribute.is_none_or(|(k, v)| n.attribute(k) == Some(v)))
        .and_then(|n| n.text())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
}
fn author_name(node: Node<'_, '_>) -> Option<String> {
    let parts: Vec<_> = node
        .descendants()
        .filter(|n| n.has_tag_name("forename") || n.has_tag_name("surname"))
        .filter_map(|n| n.text())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .collect();
    (!parts.is_empty()).then(|| parts.join(" "))
}
fn scope_value(node: Node<'_, '_>, unit: &str) -> Option<String> {
    node.descendants()
        .find(|n| n.has_tag_name("biblScope") && n.attribute("unit") == Some(unit))
        .and_then(|n| n.text())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
}
fn pages_value(node: Node<'_, '_>) -> Option<String> {
    let page = node
        .descendants()
        .find(|n| n.has_tag_name("biblScope") && n.attribute("unit") == Some("page"))?;
    match (page.attribute("from"), page.attribute("to")) {
        (Some(from), Some(to)) => Some(format!("{from}-{to}")),
        _ => page
            .text()
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(str::to_string),
    }
}

#[cfg(test)]
mod tests {
    use std::io::{Read, Write};
    use std::net::TcpListener;
    use std::thread;

    use super::*;

    fn consume_request(stream: &mut std::net::TcpStream) {
        let mut request = Vec::new();
        let mut chunk = [0_u8; 8192];
        loop {
            let size = stream.read(&mut chunk).expect("read request");
            request.extend_from_slice(&chunk[..size]);
            let text = String::from_utf8_lossy(&request);
            let Some(header_end) = text.find("\r\n\r\n") else {
                continue;
            };
            let content_length = text
                .lines()
                .find_map(|line| line.strip_prefix("Content-Length: "))
                .and_then(|value| value.parse::<usize>().ok())
                .unwrap_or_default();
            if request.len() >= header_end + 4 + content_length {
                break;
            }
        }
    }
    const TEI: &str = r#"<TEI><text><back><listBibl><biblStruct xml:id="b0"><analytic><title level="a">Safe extraction</title><author><persName><forename>Ada</forename><surname>Lovelace</surname></persName></author><idno type="DOI">10.1000/test</idno></analytic><monogr><title level="j">Journal</title><imprint><date when="2026"/><biblScope unit="volume">2</biblScope><biblScope unit="issue">1</biblScope><biblScope unit="page" from="10" to="12"/></imprint></monogr><note type="raw_reference">Lovelace, A. (2026). Safe extraction.</note></biblStruct></listBibl></back></text></TEI>"#;
    #[test]
    fn config_is_disabled_and_local_by_default() {
        let c = GrobidConfig::default();
        assert!(!c.enabled);
        assert!(c.validate().is_ok());
        assert!(matches!(
            (GrobidExtractor { config: c }).health(),
            Err(GrobidError::Disabled)
        ));
    }
    #[test]
    fn remote_http_requires_explicit_secure_opt_in() {
        let mut c = GrobidConfig {
            base_url: "http://example.com".into(),
            ..Default::default()
        };
        assert!(c.validate().is_err());
        c.allow_remote = true;
        assert!(c.validate().is_err());
        c.base_url = "https://example.com".into();
        c.allowed_remote_hosts = vec!["example.com".into()];
        assert!(c.validate().is_ok());
    }
    #[test]
    fn tei_decodes_supported_reference_fields() {
        let d = decode_references_tei(TEI, Some("0.9.0".into())).unwrap();
        let r = &d.references[0];
        assert_eq!(r.id, "b0");
        assert_eq!(r.authors, ["Ada Lovelace"]);
        assert_eq!(r.pages.as_deref(), Some("10-12"));
        assert_eq!(r.identifiers[0].scheme, "doi");
        assert!(
            d.provenance
                .configuration
                .contains("consolidateCitations=0")
        );
    }
    #[test]
    fn malformed_and_partial_tei_are_explicit() {
        assert!(matches!(
            decode_references_tei("<TEI>", None),
            Err(GrobidError::MalformedTei(_))
        ));
        let d = decode_references_tei("<TEI><biblStruct/></TEI>", None).unwrap();
        assert_eq!(d.diagnostics[0].code, "grobid.raw_reference.missing");
    }

    #[test]
    fn external_entity_input_is_not_expanded() {
        let xml = r#"<!DOCTYPE foo [<!ENTITY xxe SYSTEM "file:///private.txt">]><TEI><biblStruct xml:id="r1"><note type="raw_reference">&xxe;</note></biblStruct></TEI>"#;
        assert!(decode_references_tei(xml, None).is_err());
    }

    #[test]
    fn local_mock_receives_reference_request_contract() {
        let listener = TcpListener::bind("127.0.0.1:0").expect("bind mock GROBID");
        let address = listener.local_addr().expect("mock address");
        let worker = thread::spawn(move || {
            let (mut stream, _) = listener.accept().expect("accept request");
            let mut request = [0_u8; 8192];
            let size = stream.read(&mut request).expect("read request");
            let request = String::from_utf8_lossy(&request[..size]);
            assert!(request.contains("includeRawCitations"));
            assert!(request.contains("consolidateCitations"));
            let body = TEI.as_bytes();
            write!(
                stream,
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n",
                body.len()
            )
            .unwrap();
            stream.write_all(body).unwrap();
        });
        let extractor = GrobidExtractor {
            config: GrobidConfig {
                enabled: true,
                base_url: format!("http://{address}"),
                timeout_seconds: 5,
                ..GrobidConfig::default()
            },
        };
        let result = extractor
            .extract_references(b"pdf-fixture")
            .expect("mock extraction");
        worker.join().expect("mock worker");
        assert_eq!(result.references[0].id, "b0");
        assert_eq!(result.provenance.endpoint_class.as_deref(), Some("local"));
        assert!(result.provenance.input_hash.is_some());
    }

    #[test]
    fn local_mock_content_length_limit_is_enforced_before_body_read() {
        let listener = TcpListener::bind("127.0.0.1:0").expect("bind mock GROBID");
        let address = listener.local_addr().expect("mock address");
        let worker = thread::spawn(move || {
            let (mut stream, _) = listener.accept().expect("accept request");
            let mut request = Vec::new();
            let mut chunk = [0_u8; 8192];
            let expected = loop {
                let size = stream.read(&mut chunk).expect("read request");
                request.extend_from_slice(&chunk[..size]);
                let text = String::from_utf8_lossy(&request);
                if let Some(header_end) = text.find("\r\n\r\n") {
                    let content_length = text
                        .lines()
                        .find_map(|line| line.strip_prefix("Content-Length: "))
                        .and_then(|value| value.parse::<usize>().ok())
                        .unwrap_or_default();
                    if request.len() >= header_end + 4 + content_length {
                        break header_end + 4 + content_length;
                    }
                }
            };
            assert!(request.len() >= expected);
            stream
                .write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 11\r\nConnection: close\r\n\r\n01234567890")
                .unwrap();
        });
        let extractor = GrobidExtractor {
            config: GrobidConfig {
                enabled: true,
                base_url: format!("http://{address}"),
                max_response_bytes: 10,
                ..GrobidConfig::default()
            },
        };
        let result = extractor.extract_references(b"pdf");
        assert!(
            matches!(result, Err(GrobidError::ResponseTooLarge)),
            "result: {result:?}"
        );
        worker.join().expect("mock worker");
    }

    #[test]
    fn local_mock_maps_no_content_and_bounded_overload() {
        let listener = TcpListener::bind("127.0.0.1:0").expect("bind mock GROBID");
        let address = listener.local_addr().expect("mock address");
        let worker = thread::spawn(move || {
            for _ in 0..=2 {
                let (mut stream, _) = listener.accept().expect("accept request");
                consume_request(&mut stream);
                stream
                    .write_all(b"HTTP/1.1 503 Service Unavailable\r\nContent-Length: 0\r\n\r\n")
                    .unwrap();
            }
        });
        let extractor = GrobidExtractor {
            config: GrobidConfig {
                enabled: true,
                base_url: format!("http://{address}"),
                max_retries: 2,
                ..GrobidConfig::default()
            },
        };
        assert!(matches!(
            extractor.extract_references(b"pdf"),
            Err(GrobidError::Overloaded)
        ));
        worker.join().expect("mock worker");

        let listener = TcpListener::bind("127.0.0.1:0").expect("bind no-content GROBID");
        let address = listener.local_addr().expect("no-content address");
        let worker = thread::spawn(move || {
            let (mut stream, _) = listener.accept().expect("accept no-content request");
            consume_request(&mut stream);
            stream
                .write_all(b"HTTP/1.1 204 No Content\r\nContent-Length: 0\r\n\r\n")
                .unwrap();
        });
        let extractor = GrobidExtractor {
            config: GrobidConfig {
                enabled: true,
                base_url: format!("http://{address}"),
                ..GrobidConfig::default()
            },
        };
        assert!(matches!(
            extractor.extract_references(b"pdf"),
            Err(GrobidError::NoContent)
        ));
        worker.join().expect("no-content worker");
    }
}
