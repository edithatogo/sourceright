//! Adapter from neutral scholarly extraction output into Sourceright intake and review evidence.

use crate::citeweft::ScholarlyDocument;
use crate::intake::ReferenceCandidate;
use crate::sidecar::{
    ExtractionProvenance, ReferenceVerification, ReviewStatus, VerificationSidecar,
};

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct SourcerightExtraction {
    pub candidates: Vec<ReferenceCandidate>,
    pub sidecar: VerificationSidecar,
}

pub fn adapt_scholarly_document(
    document: &ScholarlyDocument,
    source: &str,
) -> SourcerightExtraction {
    let mut sidecar = VerificationSidecar::empty();
    let candidates = document
        .references
        .iter()
        .map(|reference| {
            let span = reference
                .span
                .as_ref()
                .and_then(|span| span.source_id.as_deref())
                .map(|id| format!("grobid_tei:{id}"))
                .unwrap_or_else(|| "grobid_tei".to_string());
            sidecar.references.insert(
                reference.id.clone(),
                ReferenceVerification {
                    extraction: Some(ExtractionProvenance {
                        source: format!(
                            "{source};backend={};version={}",
                            document.provenance.backend,
                            document
                                .provenance
                                .engine_version
                                .as_deref()
                                .unwrap_or("unknown")
                        ),
                        original_text: Some(reference.raw_text.clone()),
                        span: Some(span.clone()),
                    }),
                    review_status: ReviewStatus::Queued,
                    ..ReferenceVerification::default()
                },
            );
            ReferenceCandidate {
                id: reference.id.clone(),
                text: reference.raw_text.clone(),
                source: source.to_string(),
                span,
            }
        })
        .collect();
    SourcerightExtraction {
        candidates,
        sidecar,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grobid::decode_references_tei;
    #[test]
    fn vertical_tei_to_candidate_and_sidecar_never_creates_canonical_csl() {
        let tei = r#"<TEI><biblStruct xml:id="r1"><title level="a">A title</title><note type="raw_reference">Raw reference evidence</note></biblStruct></TEI>"#;
        let neutral = decode_references_tei(tei, Some("fixture".into())).unwrap();
        let adapted = adapt_scholarly_document(&neutral, "fixture.pdf");
        assert_eq!(adapted.candidates[0].id, "r1");
        assert_eq!(adapted.candidates[0].text, "Raw reference evidence");
        let verification = &adapted.sidecar.references["r1"];
        assert_eq!(verification.review_status, ReviewStatus::Queued);
        assert_eq!(
            verification
                .extraction
                .as_ref()
                .unwrap()
                .original_text
                .as_deref(),
            Some("Raw reference evidence")
        );
    }
}
