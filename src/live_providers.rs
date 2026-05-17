use std::{
    collections::hash_map::DefaultHasher,
    fs,
    hash::{Hash, Hasher},
    path::PathBuf,
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use reqwest::Url;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::csl::CslItem;
use crate::providers::{
    AcademicProvider, AcademicProviderResult, provider_candidate_from_fields, provider_error,
};

pub const LIVE_PROVIDER_SMOKE_SCHEMA_VERSION: &str = "sourceright.live_provider_smoke.v1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LiveProviderConfig {
    pub enabled: bool,
    pub smoke_enabled: bool,
    pub unpaywall_email: Option<String>,
    pub open_citations_token: Option<String>,
    pub europe_pmc_email: Option<String>,
    pub byo_key: Option<String>,
    pub repository_pmid: Option<String>,
    pub timeout_secs: u64,
    pub min_interval_ms: u64,
    pub max_retries: u8,
    pub cache_dir: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LiveProviderRuntimeControls {
    pub timeout_secs: u64,
    pub min_interval_ms: u64,
    pub max_retries: u8,
    pub cache_enabled: bool,
}

impl LiveProviderConfig {
    pub fn from_env() -> Self {
        Self {
            enabled: env_flag("SOURCERIGHT_LIVE_PROVIDERS"),
            smoke_enabled: env_flag("SOURCERIGHT_LIVE_PROVIDER_SMOKE"),
            unpaywall_email: env_string("UNPAYWALL_EMAIL"),
            open_citations_token: env_string("OPENCITATIONS_ACCESS_TOKEN"),
            europe_pmc_email: env_string("EUROPE_PMC_EMAIL"),
            byo_key: env_string("SOURCERIGHT_BYO_KEY"),
            repository_pmid: env_string("SOURCERIGHT_REPOSITORY_PMID"),
            timeout_secs: env_u64("SOURCERIGHT_PROVIDER_TIMEOUT_SECS", 20),
            min_interval_ms: env_u64("SOURCERIGHT_PROVIDER_MIN_INTERVAL_MS", 1_000),
            max_retries: env_u64("SOURCERIGHT_PROVIDER_MAX_RETRIES", 2).min(u8::MAX as u64) as u8,
            cache_dir: env_string("SOURCERIGHT_PROVIDER_CACHE_DIR"),
        }
    }

    pub fn runtime_controls(&self) -> LiveProviderRuntimeControls {
        LiveProviderRuntimeControls {
            timeout_secs: self.timeout_secs,
            min_interval_ms: self.min_interval_ms,
            max_retries: self.max_retries,
            cache_enabled: self.cache_dir.is_some(),
        }
    }
}

pub fn live_provider_config_from_env() -> LiveProviderConfig {
    LiveProviderConfig::from_env()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LiveProviderExecution {
    Fixture,
    Live,
    Skipped,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LiveProviderOutcome {
    pub provider: String,
    pub execution: LiveProviderExecution,
    pub endpoint: Option<String>,
    pub skip_reason: Option<String>,
    pub result: Option<AcademicProviderResult>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LiveProviderSmokeState {
    pub schema_version: &'static str,
    pub enabled: bool,
    pub smoke_enabled: bool,
    pub runtime_controls: LiveProviderRuntimeControls,
    pub retrieved_at: String,
    pub outcomes: Vec<LiveProviderOutcome>,
}

pub fn live_provider_smoke_report(
    canonical: &CslItem,
    config: &LiveProviderConfig,
) -> LiveProviderSmokeState {
    let retrieved_at = current_retrieved_at();
    LiveProviderSmokeState {
        schema_version: LIVE_PROVIDER_SMOKE_SCHEMA_VERSION,
        enabled: config.enabled,
        smoke_enabled: config.smoke_enabled,
        runtime_controls: config.runtime_controls(),
        retrieved_at: retrieved_at.clone(),
        outcomes: vec![
            smoke_unpaywall(canonical, config, &retrieved_at),
            smoke_open_citations(canonical, config, &retrieved_at),
            smoke_arxiv(canonical, config, &retrieved_at),
            smoke_europe_pmc(canonical, config, &retrieved_at),
            smoke_repository_records(canonical, config, &retrieved_at),
            smoke_licensed_byo_key(canonical, config, &retrieved_at),
        ],
    }
}

pub fn live_provider_smoke_report_from_env(canonical: &CslItem) -> LiveProviderSmokeState {
    live_provider_smoke_report(canonical, &LiveProviderConfig::from_env())
}

pub fn unpaywall_fixture_result(
    retrieved_at: &str,
    canonical: &CslItem,
    payload: &Value,
) -> AcademicProviderResult {
    let doi = payload
        .get("doi")
        .and_then(Value::as_str)
        .or_else(|| payload.get("doi_url").and_then(Value::as_str))
        .unwrap_or_default();
    let title = payload
        .get("title")
        .and_then(Value::as_str)
        .unwrap_or_default();
    provider_candidate_from_fields(
        AcademicProvider::Unpaywall,
        "unpaywall",
        retrieved_at,
        canonical,
        payload,
        doi,
        title,
    )
}

pub fn open_citations_fixture_result(
    retrieved_at: &str,
    canonical: &CslItem,
    payload: &Value,
) -> AcademicProviderResult {
    let first = payload.as_array().and_then(|items| items.first()).cloned();
    if let Some(first) = first {
        let doi = first
            .get("cited")
            .and_then(Value::as_str)
            .or_else(|| first.get("citing").and_then(Value::as_str))
            .unwrap_or_default();
        provider_candidate_from_fields(
            AcademicProvider::OpenCitations,
            "opencitations",
            retrieved_at,
            canonical,
            &first,
            doi,
            "",
        )
    } else {
        provider_error(
            AcademicProvider::OpenCitations,
            "provider.opencitations.malformed_response",
            "OpenCitations response was not a JSON array.",
        )
    }
}

pub fn arxiv_fixture_result(
    retrieved_at: &str,
    canonical: &CslItem,
    atom: &str,
) -> AcademicProviderResult {
    let entry = extract_arxiv_entry(atom);
    match entry {
        Some(entry) => {
            let doi = xml_tag(&entry, "arxiv:doi")
                .or_else(|| xml_tag(&entry, "doi"))
                .unwrap_or_default();
            let title = xml_tag(&entry, "title").unwrap_or_default();
            let data = json!({
                "entry": {
                    "id": xml_tag(&entry, "id"),
                    "title": title,
                    "summary": xml_tag(&entry, "summary"),
                    "published": xml_tag(&entry, "published"),
                    "updated": xml_tag(&entry, "updated"),
                    "doi": doi,
                    "primary_category": xml_attribute(&entry, "arxiv:primary_category", "term"),
                    "categories": xml_category_terms(&entry),
                }
            });
            provider_candidate_from_fields(
                AcademicProvider::Arxiv,
                "arxiv",
                retrieved_at,
                canonical,
                &data,
                &doi,
                &title,
            )
        }
        None => provider_error(
            AcademicProvider::Arxiv,
            "provider.arxiv.malformed_response",
            "arXiv Atom feed did not contain an entry.",
        ),
    }
}

pub fn europe_pmc_fixture_result(
    retrieved_at: &str,
    canonical: &CslItem,
    payload: &Value,
) -> AcademicProviderResult {
    let result = payload
        .get("resultList")
        .and_then(|list| list.get("result"))
        .and_then(Value::as_array)
        .and_then(|items| items.first())
        .cloned();

    match result {
        Some(result) => {
            let doi = result
                .get("doi")
                .and_then(Value::as_str)
                .unwrap_or_default();
            let title = result
                .get("title")
                .and_then(Value::as_str)
                .unwrap_or_default();
            provider_candidate_from_fields(
                AcademicProvider::EuropePmc,
                "europe-pmc",
                retrieved_at,
                canonical,
                &result,
                doi,
                title,
            )
        }
        None => provider_error(
            AcademicProvider::EuropePmc,
            "provider.europe_pmc.malformed_response",
            "Europe PMC response did not include a resultList.result entry.",
        ),
    }
}

pub fn repository_records_fixture_result(
    retrieved_at: &str,
    canonical: &CslItem,
    payload: &Value,
) -> AcademicProviderResult {
    let doi = payload
        .get("doi")
        .and_then(Value::as_str)
        .unwrap_or_default();
    let title = payload
        .get("title")
        .and_then(Value::as_str)
        .or_else(|| payload.get("sorttitle").and_then(Value::as_str))
        .unwrap_or_default();
    provider_candidate_from_fields(
        AcademicProvider::RepositoryRecords,
        "repository-records",
        retrieved_at,
        canonical,
        payload,
        doi,
        title,
    )
}

pub fn licensed_byo_key_fixture_result(
    retrieved_at: &str,
    canonical: &CslItem,
    provider_name: &str,
    payload: &Value,
) -> AcademicProviderResult {
    let doi = payload
        .get("doi")
        .and_then(Value::as_str)
        .unwrap_or_default();
    let title = payload
        .get("title")
        .and_then(Value::as_str)
        .unwrap_or_default();
    provider_candidate_from_fields(
        AcademicProvider::LicensedByoKey,
        provider_name,
        retrieved_at,
        canonical,
        payload,
        doi,
        title,
    )
}

pub fn smoke_unpaywall(
    canonical: &CslItem,
    config: &LiveProviderConfig,
    retrieved_at: &str,
) -> LiveProviderOutcome {
    let Some(doi) = canonical.doi.as_deref() else {
        return skipped("unpaywall", "canonical record has no DOI");
    };
    if !config.enabled || !config.smoke_enabled {
        return skipped(
            "unpaywall",
            "live provider smoke tests are disabled by configuration",
        );
    }
    let Some(email) = config.unpaywall_email.as_deref() else {
        return skipped("unpaywall", "UNPAYWALL_EMAIL is not set");
    };

    let endpoint = unpaywall_endpoint(doi, email);
    match fetch_json(&endpoint, config, None) {
        Ok(payload) => LiveProviderOutcome {
            provider: "unpaywall".to_string(),
            execution: LiveProviderExecution::Live,
            endpoint: Some(endpoint.to_string()),
            skip_reason: None,
            result: Some(unpaywall_fixture_result(retrieved_at, canonical, &payload)),
        },
        Err(error) => failed("unpaywall", endpoint.to_string(), error),
    }
}

pub fn smoke_open_citations(
    canonical: &CslItem,
    config: &LiveProviderConfig,
    retrieved_at: &str,
) -> LiveProviderOutcome {
    let Some(doi) = canonical.doi.as_deref() else {
        return skipped("opencitations", "canonical record has no DOI");
    };
    if !config.enabled || !config.smoke_enabled {
        return skipped(
            "opencitations",
            "live provider smoke tests are disabled by configuration",
        );
    }

    let endpoint = open_citations_endpoint(doi);
    let auth = config
        .open_citations_token
        .as_deref()
        .map(|token| ("authorization", format!("Bearer {token}")));
    let headers = auth.as_ref().map(|(k, v)| [(*k, v.as_str())]);

    match fetch_json(&endpoint, config, headers) {
        Ok(payload) => LiveProviderOutcome {
            provider: "opencitations".to_string(),
            execution: LiveProviderExecution::Live,
            endpoint: Some(endpoint.to_string()),
            skip_reason: None,
            result: Some(open_citations_fixture_result(
                retrieved_at,
                canonical,
                &payload,
            )),
        },
        Err(error) => failed("opencitations", endpoint.to_string(), error),
    }
}

pub fn smoke_arxiv(
    canonical: &CslItem,
    config: &LiveProviderConfig,
    retrieved_at: &str,
) -> LiveProviderOutcome {
    let query = canonical
        .doi
        .as_deref()
        .map(|doi| format!("doi:{doi}"))
        .or_else(|| {
            canonical
                .title
                .as_deref()
                .map(|title| format!("all:{title}"))
        });
    let Some(query) = query else {
        return skipped("arxiv", "canonical record has no DOI or title");
    };
    if !config.enabled || !config.smoke_enabled {
        return skipped(
            "arxiv",
            "live provider smoke tests are disabled by configuration",
        );
    }

    let endpoint = arxiv_endpoint(&query);
    match fetch_text(&endpoint, config) {
        Ok(payload) => LiveProviderOutcome {
            provider: "arxiv".to_string(),
            execution: LiveProviderExecution::Live,
            endpoint: Some(endpoint.to_string()),
            skip_reason: None,
            result: Some(arxiv_fixture_result(retrieved_at, canonical, &payload)),
        },
        Err(error) => failed("arxiv", endpoint.to_string(), error),
    }
}

pub fn smoke_europe_pmc(
    canonical: &CslItem,
    config: &LiveProviderConfig,
    retrieved_at: &str,
) -> LiveProviderOutcome {
    let Some(doi) = canonical.doi.as_deref() else {
        return skipped("europepmc", "canonical record has no DOI");
    };
    if !config.enabled || !config.smoke_enabled {
        return skipped(
            "europepmc",
            "live provider smoke tests are disabled by configuration",
        );
    }
    let endpoint = europe_pmc_endpoint(doi);
    match fetch_json(&endpoint, config, None) {
        Ok(payload) => LiveProviderOutcome {
            provider: "europepmc".to_string(),
            execution: LiveProviderExecution::Live,
            endpoint: Some(endpoint.to_string()),
            skip_reason: None,
            result: Some(europe_pmc_fixture_result(retrieved_at, canonical, &payload)),
        },
        Err(error) => failed("europepmc", endpoint.to_string(), error),
    }
}

pub fn smoke_repository_records(
    canonical: &CslItem,
    config: &LiveProviderConfig,
    retrieved_at: &str,
) -> LiveProviderOutcome {
    let Some(pmid) = config.repository_pmid.as_deref() else {
        return skipped(
            "repository-records",
            "SOURCERIGHT_REPOSITORY_PMID is not set",
        );
    };
    if !config.enabled || !config.smoke_enabled {
        return skipped(
            "repository-records",
            "live provider smoke tests are disabled by configuration",
        );
    }
    let endpoint = ncbi_esummary_endpoint(pmid);
    match fetch_json(&endpoint, config, None) {
        Ok(payload) => LiveProviderOutcome {
            provider: "repository-records".to_string(),
            execution: LiveProviderExecution::Live,
            endpoint: Some(endpoint.to_string()),
            skip_reason: None,
            result: Some(repository_records_fixture_result(
                retrieved_at,
                canonical,
                &payload,
            )),
        },
        Err(error) => failed("repository-records", endpoint.to_string(), error),
    }
}

pub fn smoke_licensed_byo_key(
    canonical: &CslItem,
    config: &LiveProviderConfig,
    retrieved_at: &str,
) -> LiveProviderOutcome {
    if !config.enabled || !config.smoke_enabled {
        return skipped(
            "licensed-byo-key",
            "live provider smoke tests are disabled by configuration",
        );
    }
    let Some(key) = config.byo_key.as_deref() else {
        return skipped("licensed-byo-key", "SOURCERIGHT_BYO_KEY is not set");
    };

    let payload = json!({
        "provider_key": key,
        "title": canonical.title.clone().unwrap_or_default(),
        "doi": canonical.doi.clone().unwrap_or_default(),
        "record_type": "licensed_provider_probe",
    });

    LiveProviderOutcome {
        provider: "licensed-byo-key".to_string(),
        execution: LiveProviderExecution::Fixture,
        endpoint: None,
        skip_reason: None,
        result: Some(licensed_byo_key_fixture_result(
            retrieved_at,
            canonical,
            "licensed-byo-key",
            &payload,
        )),
    }
}

fn skipped(provider: &str, reason: &str) -> LiveProviderOutcome {
    LiveProviderOutcome {
        provider: provider.to_string(),
        execution: LiveProviderExecution::Skipped,
        endpoint: None,
        skip_reason: Some(reason.to_string()),
        result: None,
    }
}

fn failed(provider: &str, endpoint: String, error: String) -> LiveProviderOutcome {
    LiveProviderOutcome {
        provider: provider.to_string(),
        execution: LiveProviderExecution::Skipped,
        endpoint: Some(endpoint),
        skip_reason: Some(error),
        result: None,
    }
}

fn current_retrieved_at() -> String {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => format!("unix-seconds:{}", duration.as_secs()),
        Err(error) => format!("clock-error:{error}"),
    }
}

fn env_flag(name: &str) -> bool {
    matches!(
        std::env::var(name).ok().as_deref(),
        Some("1") | Some("true") | Some("TRUE") | Some("yes") | Some("YES")
    )
}

fn env_string(name: &str) -> Option<String> {
    std::env::var(name)
        .ok()
        .filter(|value| !value.trim().is_empty())
}

fn env_u64(name: &str, fallback: u64) -> u64 {
    std::env::var(name)
        .ok()
        .and_then(|value| value.trim().parse::<u64>().ok())
        .unwrap_or(fallback)
}

fn fetch_json(
    endpoint: &Url,
    config: &LiveProviderConfig,
    headers: Option<[(&str, &str); 1]>,
) -> Result<Value, String> {
    if let Some(cached) = read_cached(endpoint, config, "json")? {
        return serde_json::from_str::<Value>(&cached)
            .map_err(|error| format!("cached JSON for {endpoint} is malformed: {error}"));
    }

    let client = reqwest::blocking::Client::builder()
        .user_agent(concat!("sourceright/", env!("CARGO_PKG_VERSION")))
        .timeout(Duration::from_secs(config.timeout_secs))
        .build()
        .map_err(|error| error.to_string())?;

    let mut last_error = String::new();
    for attempt in 0..=config.max_retries {
        apply_provider_interval(config);
        let mut request = client.get(endpoint.clone());
        if let Some(headers) = headers {
            for (name, value) in headers {
                request = request.header(name, value);
            }
        }
        match request
            .send()
            .and_then(|response| response.error_for_status())
            .and_then(|response| response.text())
        {
            Ok(body) => {
                write_cached(endpoint, config, "json", &body)?;
                return serde_json::from_str::<Value>(&body).map_err(|error| error.to_string());
            }
            Err(error) => {
                last_error = error.to_string();
                if attempt == config.max_retries {
                    break;
                }
            }
        }
    }
    Err(last_error)
}

fn fetch_text(endpoint: &Url, config: &LiveProviderConfig) -> Result<String, String> {
    if let Some(cached) = read_cached(endpoint, config, "txt")? {
        return Ok(cached);
    }

    let client = reqwest::blocking::Client::builder()
        .user_agent(concat!("sourceright/", env!("CARGO_PKG_VERSION")))
        .timeout(Duration::from_secs(config.timeout_secs))
        .build()
        .map_err(|error| error.to_string())?;

    let mut last_error = String::new();
    for attempt in 0..=config.max_retries {
        apply_provider_interval(config);
        match client
            .get(endpoint.clone())
            .send()
            .and_then(|response| response.error_for_status())
            .and_then(|response| response.text())
        {
            Ok(body) => {
                write_cached(endpoint, config, "txt", &body)?;
                return Ok(body);
            }
            Err(error) => {
                last_error = error.to_string();
                if attempt == config.max_retries {
                    break;
                }
            }
        }
    }
    Err(last_error)
}

fn apply_provider_interval(config: &LiveProviderConfig) {
    if config.min_interval_ms > 0 {
        thread::sleep(Duration::from_millis(config.min_interval_ms));
    }
}

fn read_cached(
    endpoint: &Url,
    config: &LiveProviderConfig,
    extension: &str,
) -> Result<Option<String>, String> {
    let Some(path) = cache_path(endpoint, config, extension) else {
        return Ok(None);
    };
    if path.exists() {
        return fs::read_to_string(&path)
            .map(Some)
            .map_err(|error| format!("could not read provider cache {}: {error}", path.display()));
    }
    Ok(None)
}

fn write_cached(
    endpoint: &Url,
    config: &LiveProviderConfig,
    extension: &str,
    body: &str,
) -> Result<(), String> {
    let Some(path) = cache_path(endpoint, config, extension) else {
        return Ok(());
    };
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| {
            format!(
                "could not create provider cache {}: {error}",
                parent.display()
            )
        })?;
    }
    fs::write(&path, body)
        .map_err(|error| format!("could not write provider cache {}: {error}", path.display()))
}

fn cache_path(endpoint: &Url, config: &LiveProviderConfig, extension: &str) -> Option<PathBuf> {
    let cache_dir = config.cache_dir.as_ref()?;
    let mut hasher = DefaultHasher::new();
    endpoint.as_str().hash(&mut hasher);
    Some(PathBuf::from(cache_dir).join(format!("{:016x}.{extension}", hasher.finish())))
}

fn unpaywall_endpoint(doi: &str, email: &str) -> Url {
    let mut url = Url::parse("https://api.unpaywall.org/v2/").expect("unpaywall base URL");
    url.path_segments_mut()
        .expect("unpaywall path segments")
        .push(doi);
    url.query_pairs_mut().append_pair("email", email);
    url
}

fn open_citations_endpoint(doi: &str) -> Url {
    let mut url = Url::parse("https://api.opencitations.net/index/v1/citations/")
        .expect("OpenCitations base URL");
    url.path_segments_mut()
        .expect("OpenCitations path segments")
        .push(doi);
    url.query_pairs_mut().append_pair("format", "json");
    url
}

fn arxiv_endpoint(query: &str) -> Url {
    let mut url = Url::parse("https://export.arxiv.org/api/query").expect("arXiv base URL");
    url.query_pairs_mut()
        .append_pair("search_query", query)
        .append_pair("start", "0")
        .append_pair("max_results", "1");
    url
}

fn europe_pmc_endpoint(doi: &str) -> Url {
    let mut url = Url::parse("https://www.ebi.ac.uk/europepmc/webservices/rest/search")
        .expect("Europe PMC base URL");
    url.query_pairs_mut()
        .append_pair("query", &format!("DOI:{doi}"))
        .append_pair("format", "json")
        .append_pair("resultType", "core")
        .append_pair("pageSize", "1");
    url
}

fn ncbi_esummary_endpoint(pmid: &str) -> Url {
    let mut url = Url::parse("https://eutils.ncbi.nlm.nih.gov/entrez/eutils/esummary.fcgi")
        .expect("NCBI eSummary base URL");
    url.query_pairs_mut()
        .append_pair("db", "pubmed")
        .append_pair("id", pmid)
        .append_pair("retmode", "json");
    url
}

fn extract_arxiv_entry(atom: &str) -> Option<String> {
    let start = atom.find("<entry>")?;
    let end = atom[start..].find("</entry>")?;
    Some(atom[start..start + end + "</entry>".len()].to_string())
}

fn xml_tag(input: &str, tag: &str) -> Option<String> {
    let start = format!("<{tag}>");
    let end = format!("</{tag}>");
    let open = input.find(&start)? + start.len();
    let close = input[open..].find(&end)? + open;
    Some(input[open..close].trim().to_string())
}

fn xml_attribute(input: &str, tag: &str, attribute: &str) -> Option<String> {
    let needle = format!("<{tag} ");
    let start = input.find(&needle)?;
    let end = input[start..].find('>')? + start;
    let attrs = &input[start..end];
    let attr_start = attrs.find(&format!("{attribute}=\""))? + attribute.len() + 2;
    let attr_end = attrs[attr_start..].find('"')? + attr_start;
    Some(attrs[attr_start..attr_end].to_string())
}

fn xml_category_terms(entry: &str) -> Vec<String> {
    let mut terms = Vec::new();
    let mut search = entry;
    let prefix_len = "<category ".len();
    while let Some(index) = search.find("<category ") {
        let after = &search[index..];
        if let Some(term) = xml_attribute(after, "category", "term") {
            terms.push(term);
        }
        search = &after[prefix_len..];
    }
    terms
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;
    use crate::providers::ProviderResultStatus;

    fn canonical_item() -> CslItem {
        CslItem {
            id: "smith-2024".to_string(),
            item_type: "article-journal".to_string(),
            title: Some("Trial paper".to_string()),
            doi: Some("10.1000/example".to_string()),
            extra: BTreeMap::new(),
        }
    }

    #[test]
    fn default_smoke_report_skips_without_credentials() {
        let report = live_provider_smoke_report_from_env(&canonical_item());

        assert_eq!(report.schema_version, LIVE_PROVIDER_SMOKE_SCHEMA_VERSION);
        assert_eq!(
            report.runtime_controls,
            LiveProviderRuntimeControls {
                timeout_secs: 20,
                min_interval_ms: 1_000,
                max_retries: 2,
                cache_enabled: false,
            }
        );
        assert!(
            report
                .outcomes
                .iter()
                .all(|outcome| outcome.result.is_none())
        );
        assert!(
            report
                .outcomes
                .iter()
                .all(|outcome| outcome.execution == LiveProviderExecution::Skipped)
        );
    }

    #[test]
    fn live_provider_config_defaults_to_conservative_runtime_policy() {
        let config = LiveProviderConfig {
            enabled: false,
            smoke_enabled: false,
            unpaywall_email: None,
            open_citations_token: None,
            europe_pmc_email: None,
            byo_key: None,
            repository_pmid: None,
            timeout_secs: 20,
            min_interval_ms: 1_000,
            max_retries: 2,
            cache_dir: None,
        };

        assert!(!config.enabled);
        assert!(!config.smoke_enabled);
        assert_eq!(config.timeout_secs, 20);
        assert_eq!(config.min_interval_ms, 1_000);
        assert_eq!(config.max_retries, 2);
        assert!(config.cache_dir.is_none());
        assert_eq!(
            config.runtime_controls(),
            LiveProviderRuntimeControls {
                timeout_secs: 20,
                min_interval_ms: 1_000,
                max_retries: 2,
                cache_enabled: false,
            }
        );
    }

    #[test]
    fn provider_cache_returns_evidence_payload_without_network() {
        let endpoint = Url::parse("https://example.test/provider?q=cache-hit").expect("URL");
        let cache_root =
            std::env::temp_dir().join(format!("sourceright-provider-cache-{}", std::process::id()));
        let config = LiveProviderConfig {
            enabled: true,
            smoke_enabled: true,
            unpaywall_email: None,
            open_citations_token: None,
            europe_pmc_email: None,
            byo_key: None,
            repository_pmid: None,
            timeout_secs: 1,
            min_interval_ms: 0,
            max_retries: 0,
            cache_dir: Some(cache_root.to_string_lossy().to_string()),
        };
        let cache_path = cache_path(&endpoint, &config, "json").expect("cache path");
        fs::create_dir_all(cache_path.parent().expect("cache parent")).expect("cache dir");
        fs::write(&cache_path, r#"{"title":"Cached provider evidence"}"#).expect("cache file");

        let payload = fetch_json(&endpoint, &config, None).expect("cached payload");

        assert_eq!(
            payload.get("title").and_then(Value::as_str),
            Some("Cached provider evidence")
        );
        let _ = fs::remove_file(cache_path);
        let _ = fs::remove_dir_all(cache_root);
    }

    #[test]
    fn unpaywall_fixture_response_records_sidecar_evidence() {
        let payload: Value =
            serde_json::from_str(include_str!("../fixtures/providers/unpaywall.example.json"))
                .expect("unpaywall fixture");
        let result = unpaywall_fixture_result("unix-seconds:1", &canonical_item(), &payload);

        assert_eq!(result.provider, AcademicProvider::Unpaywall);
        assert_eq!(result.status, ProviderResultStatus::Match);
        assert_eq!(result.candidates[0].provider, "unpaywall");
        assert_eq!(result.candidates[0].confidence, 1.0);
    }

    #[test]
    fn europe_pmc_fixture_response_records_sidecar_evidence() {
        let payload: Value = serde_json::from_str(include_str!(
            "../fixtures/providers/europe-pmc.example.json"
        ))
        .expect("europe pmc fixture");
        let result = europe_pmc_fixture_result("unix-seconds:1", &canonical_item(), &payload);

        assert_eq!(result.provider, AcademicProvider::EuropePmc);
        assert_eq!(result.candidates[0].provider, "europe-pmc");
    }

    #[test]
    fn arxiv_fixture_response_records_sidecar_evidence() {
        let result = arxiv_fixture_result(
            "unix-seconds:1",
            &canonical_item(),
            include_str!("../fixtures/providers/arxiv.example.atom"),
        );

        assert_eq!(result.provider, AcademicProvider::Arxiv);
        assert_eq!(result.candidates[0].provider, "arxiv");
    }
    #[test]
    fn open_citations_fixture_response_records_sidecar_evidence() {
        let payload: Value = serde_json::from_str(include_str!(
            "../fixtures/providers/opencitations.example.json"
        ))
        .expect("opencitations fixture");
        let result = open_citations_fixture_result("unix-seconds:1", &canonical_item(), &payload);

        assert_eq!(result.provider, AcademicProvider::OpenCitations);
        assert_eq!(result.candidates[0].provider, "opencitations");
    }

    #[test]
    fn repository_records_fixture_response_records_sidecar_evidence() {
        let payload: Value = serde_json::from_str(include_str!(
            "../fixtures/providers/repository-records.example.json"
        ))
        .expect("repository records fixture");
        let result =
            repository_records_fixture_result("unix-seconds:1", &canonical_item(), &payload);

        assert_eq!(result.provider, AcademicProvider::RepositoryRecords);
        assert_eq!(result.candidates[0].provider, "repository-records");
    }

    #[test]
    fn licensed_byo_key_fixture_response_records_sidecar_evidence() {
        let payload: Value = serde_json::from_str(include_str!(
            "../fixtures/providers/licensed-byo-key.example.json"
        ))
        .expect("licensed byo-key fixture");
        let result = licensed_byo_key_fixture_result(
            "unix-seconds:1",
            &canonical_item(),
            "licensed-byo-key",
            &payload,
        );

        assert_eq!(result.provider, AcademicProvider::LicensedByoKey);
        assert_eq!(result.candidates[0].provider, "licensed-byo-key");
    }
}
