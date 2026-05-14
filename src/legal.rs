use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LegalCitationReport {
    pub records: Vec<LegalCitationRecord>,
    pub issues: Vec<LegalCitationIssue>,
}

impl LegalCitationReport {
    pub fn to_markdown(&self) -> String {
        let mut markdown = String::from("# Sourceright Legal Citation Report\n\n");
        markdown.push_str(&format!(
            "- Legal citation records: {}\n- Issues: {}\n\n",
            self.records.len(),
            self.issues.len()
        ));
        if self.issues.is_empty() {
            markdown.push_str("No legal citation model issues detected by the current checks.\n");
        } else {
            for issue in &self.issues {
                markdown.push_str(&format!(
                    "- `{}` `{}`: {}\n",
                    issue.issue_type.as_str(),
                    issue.citation_text,
                    issue.message
                ));
            }
        }
        markdown
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LegalCitationRecord {
    pub id: String,
    pub citation_text: String,
    pub citation_type: LegalCitationType,
    pub jurisdiction: Option<String>,
    pub court: Option<String>,
    pub year: Option<u16>,
    pub neutral_citation: Option<String>,
    pub pinpoint: Option<String>,
    pub source_span: String,
    pub providers: Vec<LegalProviderCandidate>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LegalCitationType {
    Case,
    Legislation,
    Regulation,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LegalProviderCandidate {
    pub provider: LegalProvider,
    pub confidence: f64,
    pub identifier: String,
    pub url: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LegalProvider {
    CourtListener,
    CaselawAccessProject,
    Austlii,
    LegislationRegister,
    Other,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LegalCitationIssue {
    pub issue_type: LegalCitationIssueType,
    pub citation_text: String,
    pub message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LegalCitationIssueType {
    MissingJurisdiction,
    MissingYear,
    MissingProviderCandidate,
    AmbiguousCitationType,
}

impl LegalCitationIssueType {
    fn as_str(self) -> &'static str {
        match self {
            Self::MissingJurisdiction => "missing_jurisdiction",
            Self::MissingYear => "missing_year",
            Self::MissingProviderCandidate => "missing_provider_candidate",
            Self::AmbiguousCitationType => "ambiguous_citation_type",
        }
    }
}

pub fn analyze_legal_citations(text: &str) -> LegalCitationReport {
    let records = extract_legal_citations(text);
    let issues = records
        .iter()
        .flat_map(validate_legal_citation)
        .collect::<Vec<_>>();
    LegalCitationReport { records, issues }
}

pub fn extract_legal_citations(text: &str) -> Vec<LegalCitationRecord> {
    let mut records = Vec::new();
    for (line_index, line) in text.lines().enumerate() {
        extract_neutral_citations(line, line_index + 1, &mut records);
        extract_legislation_mentions(line, line_index + 1, &mut records);
    }
    records
}

fn extract_neutral_citations(
    line: &str,
    line_number: usize,
    records: &mut Vec<LegalCitationRecord>,
) {
    let tokens = line.split_whitespace().collect::<Vec<_>>();
    for window in tokens.windows(3) {
        let year = window[0].trim_matches(['[', ']']);
        let court = window[1].trim_matches(['[', ']', ',', '.', ';', ':']);
        let number = window[2].trim_matches(['[', ']', ',', '.', ';', ':']);
        if year.len() == 4
            && year.chars().all(|ch| ch.is_ascii_digit())
            && court
                .chars()
                .all(|ch| ch.is_ascii_uppercase() || ch.is_ascii_digit())
            && number.chars().all(|ch| ch.is_ascii_digit())
        {
            let citation_text = format!("[{year}] {court} {number}");
            records.push(LegalCitationRecord {
                id: format!("legal-{:04}", records.len() + 1),
                citation_text: citation_text.clone(),
                citation_type: LegalCitationType::Case,
                jurisdiction: jurisdiction_for_court(court),
                court: Some(court.to_string()),
                year: year.parse::<u16>().ok(),
                neutral_citation: Some(citation_text),
                pinpoint: None,
                source_span: format!("line:{line_number}"),
                providers: provider_candidates_for_court(court, year, number),
            });
        }
    }
}

fn extract_legislation_mentions(
    line: &str,
    line_number: usize,
    records: &mut Vec<LegalCitationRecord>,
) {
    for marker in [" Act ", " Regulation ", " Regulations "] {
        if let Some(index) = line.find(marker) {
            let start = line[..index]
                .rfind(['.', ';'])
                .map(|pos| pos + 1)
                .unwrap_or(0);
            let end = line[index + marker.len()..]
                .find('.')
                .map(|pos| index + marker.len() + pos)
                .unwrap_or(line.len());
            let citation_text = line[start..end].trim().trim_matches(',').to_string();
            if !citation_text.is_empty() {
                let year = first_year(&citation_text);
                records.push(LegalCitationRecord {
                    id: format!("legal-{:04}", records.len() + 1),
                    citation_text,
                    citation_type: if marker.contains("Regulation") {
                        LegalCitationType::Regulation
                    } else {
                        LegalCitationType::Legislation
                    },
                    jurisdiction: None,
                    court: None,
                    year,
                    neutral_citation: None,
                    pinpoint: None,
                    source_span: format!("line:{line_number}"),
                    providers: Vec::new(),
                });
            }
        }
    }
}

fn first_year(value: &str) -> Option<u16> {
    value
        .split(|ch: char| !ch.is_ascii_digit())
        .find(|part| part.len() == 4)
        .and_then(|year| year.parse::<u16>().ok())
}

fn jurisdiction_for_court(court: &str) -> Option<String> {
    match court {
        "HCA" | "FCA" | "FCAFC" | "NSWCA" | "VSCA" | "QCA" | "SASC" | "WASCA" | "TASSC"
        | "NTSC" | "ACTSC" => Some("AU".to_string()),
        "US" | "SCOTUS" | "USCA" | "USCADC" | "USCA1" | "USCA2" | "USCA3" | "USCA4" | "USCA5"
        | "USCA6" | "USCA7" | "USCA8" | "USCA9" | "USCA10" | "USCA11" | "USCAFED" | "USD"
        | "USBC" | "USCOURT" | "USDCT" => Some("US".to_string()),
        _ => None,
    }
}

fn provider_candidates_for_court(
    court: &str,
    year: &str,
    number: &str,
) -> Vec<LegalProviderCandidate> {
    let mut providers = Vec::new();
    match jurisdiction_for_court(court).as_deref() {
        Some("AU") => providers.push(LegalProviderCandidate {
            provider: LegalProvider::Austlii,
            confidence: 0.75,
            identifier: format!("{year} {court} {number}"),
            url: None,
        }),
        Some("US") => providers.push(LegalProviderCandidate {
            provider: LegalProvider::CourtListener,
            confidence: 0.75,
            identifier: format!("{year} {court} {number}"),
            url: Some(format!(
                "https://www.courtlistener.com/?q={year}+{court}+{number}"
            )),
        }),
        _ => {}
    }
    providers
}

fn validate_legal_citation(record: &LegalCitationRecord) -> Vec<LegalCitationIssue> {
    let mut issues = Vec::new();
    if record.jurisdiction.is_none() {
        issues.push(LegalCitationIssue {
            issue_type: LegalCitationIssueType::MissingJurisdiction,
            citation_text: record.citation_text.clone(),
            message: "Legal citation has no jurisdiction; provider matching needs jurisdictional provenance.".to_string(),
        });
    }
    if record.year.is_none() {
        issues.push(LegalCitationIssue {
            issue_type: LegalCitationIssueType::MissingYear,
            citation_text: record.citation_text.clone(),
            message: "Legal citation has no extracted year.".to_string(),
        });
    }
    if record.providers.is_empty() {
        issues.push(LegalCitationIssue {
            issue_type: LegalCitationIssueType::MissingProviderCandidate,
            citation_text: record.citation_text.clone(),
            message: "Legal citation has no public-provider candidate yet.".to_string(),
        });
    }
    if record.citation_type == LegalCitationType::Unknown {
        issues.push(LegalCitationIssue {
            issue_type: LegalCitationIssueType::AmbiguousCitationType,
            citation_text: record.citation_text.clone(),
            message: "Legal citation type is ambiguous.".to_string(),
        });
    }
    issues
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn neutral_case_citation_uses_separate_legal_model() {
        let report = analyze_legal_citations(
            "The leading decision is Plaintiff M68/2015 v Minister [2016] HCA 1.",
        );

        assert_eq!(report.records.len(), 1);
        assert_eq!(report.records[0].citation_type, LegalCitationType::Case);
        assert_eq!(report.records[0].jurisdiction.as_deref(), Some("AU"));
        assert_eq!(
            report.records[0].providers[0].provider,
            LegalProvider::Austlii
        );
    }

    #[test]
    fn legislation_mentions_are_flagged_for_jurisdictional_review() {
        let report =
            analyze_legal_citations("This turns on the Privacy Act 1988 and related instruments.");

        assert_eq!(
            report.records[0].citation_type,
            LegalCitationType::Legislation
        );
        assert_eq!(report.records[0].year, Some(1988));
        assert!(
            report
                .issues
                .iter()
                .any(|issue| issue.issue_type == LegalCitationIssueType::MissingJurisdiction)
        );
        assert!(report.to_markdown().contains("missing_provider_candidate"));
    }

    #[test]
    fn us_supreme_court_case_uses_courtlistener_provider() {
        let report = analyze_legal_citations(
            "The holding in Dobbs v. Jackson Women's Health Organization [2022] SCOTUS 19.",
        );

        assert_eq!(report.records.len(), 1);
        assert_eq!(report.records[0].citation_type, LegalCitationType::Case);
        assert_eq!(report.records[0].jurisdiction.as_deref(), Some("US"));
        assert_eq!(
            report.records[0].providers[0].provider,
            LegalProvider::CourtListener
        );
        assert_eq!(report.records[0].providers[0].confidence, 0.75);
        assert!(
            report.records[0].providers[0]
                .url
                .as_deref()
                .unwrap_or("")
                .contains("courtlistener.com")
        );
    }

    #[test]
    fn us_circuit_court_case_receives_courtlistener_provider() {
        let report = analyze_legal_citations("See Smith v. United States [2023] USCA9 123.");

        assert_eq!(report.records.len(), 1);
        assert_eq!(report.records[0].jurisdiction.as_deref(), Some("US"));
        assert_eq!(
            report.records[0].providers[0].provider,
            LegalProvider::CourtListener
        );
    }
}
