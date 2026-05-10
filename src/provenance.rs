use serde::{Deserialize, Serialize};

use crate::reconcile::{CitationOccurrence, extract_citation_occurrences};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProvenanceReport {
    pub graph: EvidenceGraph,
    pub issues: Vec<ProvenanceIssue>,
}

impl ProvenanceReport {
    pub fn to_markdown(&self) -> String {
        let mut markdown = String::from("# Sourceright Claim Source Provenance Report\n\n");
        markdown.push_str(&format!(
            "- Claims: {}\n- Source links: {}\n- Issues: {}\n\n",
            self.graph.claims.len(),
            self.graph.links.len(),
            self.issues.len()
        ));
        if self.issues.is_empty() {
            markdown
                .push_str("No claim/source provenance issues detected by the current checks.\n");
        } else {
            for issue in &self.issues {
                markdown.push_str(&format!(
                    "- `{}` `{}`: {}\n",
                    issue.issue_type.as_str(),
                    issue.claim_id,
                    issue.message
                ));
            }
        }
        markdown
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceGraph {
    pub claims: Vec<ClaimNode>,
    pub sources: Vec<SourceNode>,
    pub links: Vec<ClaimSourceLink>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClaimNode {
    pub id: String,
    pub text: String,
    pub span: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceNode {
    pub id: String,
    pub citation_text: String,
    pub span: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClaimSourceLink {
    pub claim_id: String,
    pub source_id: String,
    pub link_type: ClaimSourceLinkType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaimSourceLinkType {
    CitationSupport,
    QuoteSupport,
    NeedsReview,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProvenanceIssue {
    pub issue_type: ProvenanceIssueType,
    pub claim_id: String,
    pub message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProvenanceIssueType {
    UnsupportedClaim,
    QuoteWithoutCitation,
    WeakSourceLink,
}

impl ProvenanceIssueType {
    fn as_str(self) -> &'static str {
        match self {
            Self::UnsupportedClaim => "unsupported_claim",
            Self::QuoteWithoutCitation => "quote_without_citation",
            Self::WeakSourceLink => "weak_source_link",
        }
    }
}

pub fn analyze_claim_source_provenance(text: &str) -> ProvenanceReport {
    let graph = build_evidence_graph(text);
    let issues = graph
        .claims
        .iter()
        .filter(|claim| !graph.links.iter().any(|link| link.claim_id == claim.id))
        .map(|claim| ProvenanceIssue {
            issue_type: ProvenanceIssueType::UnsupportedClaim,
            claim_id: claim.id.clone(),
            message: "Claim sentence has no detected citation or quote-source link.".to_string(),
        })
        .chain(quote_without_citation_issues(text, &graph))
        .collect::<Vec<_>>();
    ProvenanceReport { graph, issues }
}

pub fn build_evidence_graph(text: &str) -> EvidenceGraph {
    let citations = extract_citation_occurrences(text);
    let claims = extract_claims(text);
    let sources = citations
        .iter()
        .enumerate()
        .map(|(index, citation)| SourceNode {
            id: format!("source-{:04}", index + 1),
            citation_text: citation.text.clone(),
            span: citation.span.clone(),
        })
        .collect::<Vec<_>>();
    let links = link_claims_to_sources(&claims, &citations, &sources);

    EvidenceGraph {
        claims,
        sources,
        links,
    }
}

fn extract_claims(text: &str) -> Vec<ClaimNode> {
    let mut claims = Vec::new();
    for (line_index, line) in text.lines().enumerate() {
        for sentence in line.split_terminator('.') {
            let sentence = sentence.trim();
            if sentence.split_whitespace().count() >= 4 {
                claims.push(ClaimNode {
                    id: format!("claim-{:04}", claims.len() + 1),
                    text: sentence.to_string(),
                    span: format!("line:{}", line_index + 1),
                });
            }
        }
    }
    claims
}

fn link_claims_to_sources(
    claims: &[ClaimNode],
    citations: &[CitationOccurrence],
    sources: &[SourceNode],
) -> Vec<ClaimSourceLink> {
    let mut links = Vec::new();
    for claim in claims {
        for (index, citation) in citations.iter().enumerate() {
            if claim.span.split(':').last() == citation.span.split(':').nth(1)
                || claim.text.contains(&citation.text)
            {
                links.push(ClaimSourceLink {
                    claim_id: claim.id.clone(),
                    source_id: sources[index].id.clone(),
                    link_type: ClaimSourceLinkType::CitationSupport,
                });
            }
        }
    }
    links
}

fn quote_without_citation_issues<'a>(
    text: &'a str,
    graph: &'a EvidenceGraph,
) -> impl Iterator<Item = ProvenanceIssue> + 'a {
    graph.claims.iter().filter_map(move |claim| {
        let has_quote = claim.text.contains('"') || claim.text.contains('\'');
        let has_link = graph.links.iter().any(|link| link.claim_id == claim.id);
        (has_quote && !has_link).then(|| ProvenanceIssue {
            issue_type: ProvenanceIssueType::QuoteWithoutCitation,
            claim_id: claim.id.clone(),
            message: format!(
                "Quoted claim has no detected citation in the current text span: {}",
                text.lines().next().unwrap_or_default()
            ),
        })
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn citations_create_claim_source_links_without_claim_truth_assertions() {
        let report = analyze_claim_source_provenance(
            "The intervention improved adherence in the trial (Smith, 2024).",
        );

        assert_eq!(report.graph.claims.len(), 1);
        assert_eq!(report.graph.sources.len(), 1);
        assert_eq!(
            report.graph.links[0].link_type,
            ClaimSourceLinkType::CitationSupport
        );
        assert!(report.issues.is_empty());
    }

    #[test]
    fn unsupported_claims_are_reported() {
        let report =
            analyze_claim_source_provenance("The intervention improved adherence in the trial.");

        assert_eq!(report.graph.claims.len(), 1);
        assert_eq!(
            report.issues[0].issue_type,
            ProvenanceIssueType::UnsupportedClaim
        );
        assert!(report.to_markdown().contains("unsupported_claim"));
    }
}
