use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::csl::{CslDocument, CslItem, normalize_identifier, normalize_title};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CitationReconciliationReport {
    pub occurrences: Vec<CitationOccurrence>,
    pub matches: Vec<CitationMatch>,
    pub issues: Vec<CitationReconciliationIssue>,
}

impl CitationReconciliationReport {
    pub fn to_markdown(&self) -> String {
        let mut markdown = String::from("# Sourceright Citation Reconciliation Report\n\n");
        markdown.push_str(&format!(
            "- Citation occurrences: {}\n- Matched citations: {}\n- Issues: {}\n\n",
            self.occurrences.len(),
            self.matches.len(),
            self.issues.len()
        ));

        if self.issues.is_empty() {
            markdown.push_str("No in-text citation reconciliation issues detected.\n");
        } else {
            for issue in &self.issues {
                markdown.push_str(&format!(
                    "- `{}` `{}`: {}\n",
                    issue.issue_type.as_str(),
                    issue.reference_id.as_deref().unwrap_or("document"),
                    issue.message
                ));
            }
        }

        markdown
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CitationOccurrence {
    pub text: String,
    pub style: CitationStyle,
    pub key: String,
    pub span: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CitationStyle {
    AuthorDate,
    Identifier,
    Numeric,
    FootnoteLike,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CitationMatch {
    pub occurrence_text: String,
    pub reference_id: String,
    pub confidence: CitationMatchConfidence,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CitationMatchConfidence {
    Exact,
    Probable,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CitationReconciliationIssue {
    pub issue_type: CitationReconciliationIssueType,
    pub reference_id: Option<String>,
    pub citation_text: Option<String>,
    pub message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CitationReconciliationIssueType {
    MissingReference,
    UncitedReference,
    DuplicateCitation,
    NumericOrder,
    AmbiguousMatch,
    MixedStyleManuscript,
    TitleFallbackMatch,
}

impl CitationReconciliationIssueType {
    fn as_str(self) -> &'static str {
        match self {
            Self::MissingReference => "missing_reference",
            Self::UncitedReference => "uncited_reference",
            Self::DuplicateCitation => "duplicate_citation",
            Self::NumericOrder => "numeric_order",
            Self::AmbiguousMatch => "ambiguous_match",
            Self::MixedStyleManuscript => "mixed_style_manuscript",
            Self::TitleFallbackMatch => "title_fallback_match",
        }
    }
}

pub fn reconcile_citations(text: &str, references: &CslDocument) -> CitationReconciliationReport {
    let occurrences = extract_citation_occurrences(text);
    let reference_index = ReferenceIndex::new(references);
    let mut matches = Vec::new();
    let mut issues = Vec::new();
    let mut cited_ids = BTreeSet::new();
    let mut citation_counts = BTreeMap::<String, usize>::new();
    let mut last_numeric = 0;
    let has_author_date = occurrences
        .iter()
        .any(|occurrence| occurrence.style == CitationStyle::AuthorDate);
    let has_numeric_family = occurrences.iter().any(|occurrence| {
        matches!(
            occurrence.style,
            CitationStyle::Numeric | CitationStyle::FootnoteLike
        )
    });

    if has_author_date && has_numeric_family {
        issues.push(CitationReconciliationIssue {
            issue_type: CitationReconciliationIssueType::MixedStyleManuscript,
            reference_id: None,
            citation_text: None,
            message: "Manuscript mixes author-date and numeric citation styles; review style consistency."
                .to_string(),
        });
    }

    for occurrence in &occurrences {
        if occurrence.style == CitationStyle::Numeric
            && let Ok(number) = occurrence.key.parse::<usize>()
        {
            if number < last_numeric {
                issues.push(CitationReconciliationIssue {
                    issue_type: CitationReconciliationIssueType::NumericOrder,
                    reference_id: None,
                    citation_text: Some(occurrence.text.clone()),
                    message: "Numeric citation appears after a higher numbered citation."
                        .to_string(),
                });
            }
            last_numeric = last_numeric.max(number);
        }

        match reference_index.match_occurrence(occurrence) {
            MatchResult::Matched {
                reference_id,
                confidence,
                match_source,
            } => {
                cited_ids.insert(reference_id.clone());
                if occurrence.style != CitationStyle::Identifier {
                    *citation_counts.entry(reference_id.clone()).or_default() += 1;
                }
                if match_source == ReferenceMatchSource::TitleFallback {
                    issues.push(CitationReconciliationIssue {
                        issue_type: CitationReconciliationIssueType::TitleFallbackMatch,
                        reference_id: Some(reference_id.clone()),
                        citation_text: Some(occurrence.text.clone()),
                        message: "Citation matched via a title-derived fallback key; verify the reference-list entry."
                            .to_string(),
                    });
                }
                matches.push(CitationMatch {
                    occurrence_text: occurrence.text.clone(),
                    reference_id,
                    confidence,
                });
            }
            MatchResult::Ambiguous(ids) => issues.push(CitationReconciliationIssue {
                issue_type: CitationReconciliationIssueType::AmbiguousMatch,
                reference_id: None,
                citation_text: Some(occurrence.text.clone()),
                message: format!(
                    "Citation could match multiple references: {}",
                    ids.join(", ")
                ),
            }),
            MatchResult::Missing => issues.push(CitationReconciliationIssue {
                issue_type: CitationReconciliationIssueType::MissingReference,
                reference_id: None,
                citation_text: Some(occurrence.text.clone()),
                message: "Citation does not match a reference-list entry.".to_string(),
            }),
        }
    }

    for (reference_id, count) in citation_counts {
        if count > 1 {
            issues.push(CitationReconciliationIssue {
                issue_type: CitationReconciliationIssueType::DuplicateCitation,
                reference_id: Some(reference_id),
                citation_text: None,
                message: "Reference is cited more than once; check whether repeated citation is intended."
                    .to_string(),
            });
        }
    }

    for item in &references.items {
        if !cited_ids.contains(&item.id) {
            issues.push(CitationReconciliationIssue {
                issue_type: CitationReconciliationIssueType::UncitedReference,
                reference_id: Some(item.id.clone()),
                citation_text: None,
                message: "Reference-list entry has no detected in-text citation.".to_string(),
            });
        }
    }

    CitationReconciliationReport {
        occurrences,
        matches,
        issues,
    }
}

pub fn extract_citation_occurrences(text: &str) -> Vec<CitationOccurrence> {
    let mut occurrences = Vec::new();
    for (line_index, line) in text.lines().enumerate() {
        let masked_ranges = extract_latex_citations(line, line_index + 1, &mut occurrences);
        let masked_line = mask_ranges(line, &masked_ranges);
        extract_parenthetical(&masked_line, line_index + 1, &mut occurrences);
        extract_numeric(&masked_line, line_index + 1, &mut occurrences);
    }
    if occurrences
        .iter()
        .any(|occurrence| occurrence.style == CitationStyle::Identifier)
    {
        occurrences.retain(|occurrence| occurrence.style == CitationStyle::Identifier);
    }
    occurrences
}

fn extract_latex_citations(
    line: &str,
    line_number: usize,
    occurrences: &mut Vec<CitationOccurrence>,
) -> Vec<(usize, usize)> {
    let bytes = line.as_bytes();
    let mut cursor = 0;
    let mut masked_ranges = Vec::new();

    while cursor < bytes.len() {
        if bytes[cursor] != b'\\' {
            cursor += 1;
            continue;
        }

        let command_start = cursor;
        cursor += 1;
        let command_name_start = cursor;
        while cursor < bytes.len() && bytes[cursor].is_ascii_alphabetic() {
            cursor += 1;
        }

        if command_name_start == cursor {
            continue;
        }

        let command_name = &line[command_name_start..cursor];
        if !is_latex_citation_command(command_name) {
            continue;
        }

        if cursor < bytes.len() && bytes[cursor] == b'*' {
            cursor += 1;
        }

        cursor = skip_latex_whitespace(line, cursor);
        cursor = skip_latex_optional_arguments(line, cursor);
        cursor = skip_latex_whitespace(line, cursor);

        let Some((group_end, key_group)) = find_balanced_latex_group(line, cursor, b'{', b'}')
        else {
            continue;
        };

        for key in split_latex_citation_keys(key_group) {
            occurrences.push(CitationOccurrence {
                text: line[command_start..group_end].to_string(),
                style: CitationStyle::Identifier,
                key,
                span: format!("line:{line_number}:{}-{group_end}", command_start + 1),
            });
        }

        masked_ranges.push((command_start, group_end));
        cursor = group_end;
    }

    masked_ranges
}

fn is_latex_citation_command(command_name: &str) -> bool {
    matches!(
        command_name,
        "cite"
            | "Cite"
            | "parencite"
            | "Parencite"
            | "textcite"
            | "Textcite"
            | "autocite"
            | "Autocite"
            | "citep"
            | "Citep"
            | "citet"
            | "Citet"
            | "citealp"
            | "citeauthor"
            | "citeyear"
            | "citeyearpar"
            | "footcite"
            | "Footcite"
            | "fullcite"
            | "smartcite"
            | "Smartcite"
            | "supercite"
            | "Supercite"
            | "nocite"
    )
}

fn skip_latex_whitespace(line: &str, mut cursor: usize) -> usize {
    let bytes = line.as_bytes();
    while cursor < bytes.len() && bytes[cursor].is_ascii_whitespace() {
        cursor += 1;
    }
    cursor
}

fn skip_latex_optional_arguments(line: &str, mut cursor: usize) -> usize {
    loop {
        cursor = skip_latex_whitespace(line, cursor);
        let Some((end, _)) = find_balanced_latex_group(line, cursor, b'[', b']') else {
            return cursor;
        };
        cursor = end;
    }
}

fn find_balanced_latex_group(
    line: &str,
    start: usize,
    open: u8,
    close: u8,
) -> Option<(usize, &str)> {
    let bytes = line.as_bytes();
    if bytes.get(start).copied() != Some(open) {
        return None;
    }

    let mut depth = 0usize;
    let mut cursor = start;
    while cursor < bytes.len() {
        match bytes[cursor] {
            b'\\' => {
                cursor += 2;
                continue;
            }
            byte if byte == open => depth += 1,
            byte if byte == close => {
                depth = depth.checked_sub(1)?;
                if depth == 0 {
                    let end = cursor + 1;
                    return Some((end, &line[start + 1..cursor]));
                }
            }
            _ => {}
        }
        cursor += 1;
    }

    None
}

fn split_latex_citation_keys(key_group: &str) -> Vec<String> {
    key_group
        .split(',')
        .map(str::trim)
        .filter(|key| !key.is_empty())
        .map(ToOwned::to_owned)
        .collect()
}

fn mask_ranges(line: &str, ranges: &[(usize, usize)]) -> String {
    if ranges.is_empty() {
        return line.to_string();
    }

    let mut masked = String::with_capacity(line.len());
    for (index, ch) in line.char_indices() {
        if ranges
            .iter()
            .any(|(start, end)| index >= *start && index < *end)
        {
            masked.push(' ');
        } else {
            masked.push(ch);
        }
    }
    masked
}

fn extract_parenthetical(
    line: &str,
    line_number: usize,
    occurrences: &mut Vec<CitationOccurrence>,
) {
    let mut cursor = 0;
    while let Some(start) = line[cursor..].find('(') {
        let absolute_start = cursor + start;
        let Some(end) = line[absolute_start..].find(')') else {
            break;
        };
        let absolute_end = absolute_start + end + 1;
        let candidate = &line[absolute_start..absolute_end];
        if candidate.contains(',') && candidate.chars().any(|ch| ch.is_ascii_digit()) {
            occurrences.push(CitationOccurrence {
                text: candidate.to_string(),
                style: CitationStyle::AuthorDate,
                key: citation_author_phrase(candidate),
                span: format!("line:{line_number}:{}-{absolute_end}", absolute_start + 1),
            });
        }
        cursor = absolute_end;
    }
}

fn extract_numeric(line: &str, line_number: usize, occurrences: &mut Vec<CitationOccurrence>) {
    let mut cursor = 0;
    while let Some(start) = line[cursor..].find('[') {
        let absolute_start = cursor + start;
        let Some(end) = line[absolute_start..].find(']') else {
            break;
        };
        let absolute_end = absolute_start + end + 1;
        let candidate = &line[absolute_start..absolute_end];
        let key = candidate.trim_matches(['[', ']']).trim();
        if key.chars().all(|ch| ch.is_ascii_digit()) && !key.is_empty() {
            occurrences.push(CitationOccurrence {
                text: candidate.to_string(),
                style: CitationStyle::Numeric,
                key: key.to_string(),
                span: format!("line:{line_number}:{}-{absolute_end}", absolute_start + 1),
            });
        }
        cursor = absolute_end;
    }
}

fn citation_author_phrase(candidate: &str) -> String {
    let candidate = candidate.trim_matches(['(', ')']).trim();
    let author_segment = candidate
        .rsplit_once(',')
        .map(|(author, _)| author)
        .unwrap_or(candidate);
    normalize_identifier(author_segment).to_ascii_lowercase()
}

fn strip_author_suffix(author_segment: &str) -> &str {
    let mut segment = author_segment.trim();
    for suffix in [" et al.", " et al", " and others"] {
        if let Some(prefix) = segment.strip_suffix(suffix) {
            segment = prefix.trim_end_matches([',', ';', ':']).trim();
        }
    }
    segment
}

fn citation_author_key_candidates(author_phrase: &str) -> Vec<String> {
    let mut keys = Vec::new();

    let normalized = normalize_identifier(author_phrase).to_ascii_lowercase();
    if !normalized.is_empty() {
        keys.push(normalized);
    }

    let stripped_phrase = strip_author_suffix(author_phrase);
    let components = citation_author_components(stripped_phrase);
    let has_et_al_suffix = stripped_phrase.len() != author_phrase.len();

    if components.len() > 1 {
        let sequence_key = components.join(" ");
        if !sequence_key.is_empty() && !keys.contains(&sequence_key) {
            keys.push(sequence_key);
        }
    }

    if has_et_al_suffix && let Some(first) = components.first() {
        let et_al_key = format!("{first} et al");
        if !keys.contains(&et_al_key) {
            keys.push(et_al_key);
        }
    }

    if let Some(first) = components.first()
        && !keys.contains(first)
    {
        keys.push(first.clone());
    }

    keys
}

fn citation_author_components(author_phrase: &str) -> Vec<String> {
    let mut components = Vec::new();

    let normalized_phrase = author_phrase.replace(" and ", ", ");

    for comma_chunk in normalized_phrase.split(',') {
        let comma_chunk = comma_chunk.trim();
        if comma_chunk.is_empty() {
            continue;
        }

        for amp_chunk in comma_chunk.split('&') {
            let amp_chunk = amp_chunk.trim();
            if amp_chunk.is_empty() {
                continue;
            }

            for and_chunk in amp_chunk.split(" and ") {
                let and_chunk = and_chunk.trim();
                if and_chunk.is_empty() {
                    continue;
                }

                let normalized = normalize_identifier(and_chunk).to_ascii_lowercase();
                if !normalized.is_empty() {
                    components.push(normalized);
                }
            }
        }
    }

    if components.is_empty() {
        let normalized = normalize_identifier(author_phrase).to_ascii_lowercase();
        if !normalized.is_empty() {
            components.push(normalized);
        }
    }

    components
}

fn extract_year_details(candidate: &str) -> (Option<i32>, Option<char>) {
    let candidate = candidate.trim_matches(['(', ')']).trim();
    let Some((_, year_segment)) = candidate.rsplit_once(',') else {
        return (None, None);
    };

    let year_segment = year_segment.trim();
    let year_digits = year_segment
        .chars()
        .take_while(|ch| ch.is_ascii_digit())
        .collect::<String>();
    let year = year_digits.parse::<i32>().ok();
    let suffix = year_segment
        .chars()
        .skip(year_digits.len())
        .find(|ch| ch.is_ascii_alphabetic())
        .map(|ch| ch.to_ascii_lowercase());
    (year, suffix)
}

enum MatchResult {
    Matched {
        reference_id: String,
        confidence: CitationMatchConfidence,
        match_source: ReferenceMatchSource,
    },
    Ambiguous(Vec<String>),
    Missing,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ReferenceMatchSource {
    AuthorKey,
    TitleFallback,
    IdFallback,
}

struct ReferenceIndex<'a> {
    references: &'a CslDocument,
    author_keys: BTreeMap<String, Vec<ReferenceEntry>>,
    fallback_keys: BTreeMap<String, Vec<ReferenceEntry>>,
    id_keys: BTreeMap<String, Vec<ReferenceEntry>>,
}

impl<'a> ReferenceIndex<'a> {
    fn new(references: &'a CslDocument) -> Self {
        let mut author_keys = BTreeMap::<String, Vec<ReferenceEntry>>::new();
        let mut fallback_keys = BTreeMap::<String, Vec<ReferenceEntry>>::new();
        let mut id_keys = BTreeMap::<String, Vec<ReferenceEntry>>::new();
        for (order, item) in references.items.iter().enumerate() {
            let author_entry = ReferenceEntry {
                id: item.id.clone(),
                year: reference_year(item),
                order,
                source: ReferenceMatchSource::AuthorKey,
            };
            let id_entry = ReferenceEntry {
                id: item.id.clone(),
                year: reference_year(item),
                order,
                source: ReferenceMatchSource::IdFallback,
            };
            for key in reference_author_keys(item) {
                author_keys
                    .entry(key)
                    .or_default()
                    .push(author_entry.clone());
            }

            id_keys
                .entry(item.id.clone())
                .or_default()
                .push(id_entry.clone());
            let normalized_id = normalize_identifier(&item.id).to_ascii_lowercase();
            if normalized_id != item.id && !normalized_id.is_empty() {
                id_keys.entry(normalized_id).or_default().push(id_entry);
            }

            for (key, source) in reference_fallback_keys(item) {
                fallback_keys.entry(key).or_default().push(ReferenceEntry {
                    id: item.id.clone(),
                    year: reference_year(item),
                    order,
                    source,
                });
            }
        }
        Self {
            references,
            author_keys,
            fallback_keys,
            id_keys,
        }
    }

    fn match_occurrence(&self, occurrence: &CitationOccurrence) -> MatchResult {
        match occurrence.style {
            CitationStyle::Identifier => {
                let mut candidate_entries = self
                    .id_keys
                    .get(&occurrence.key)
                    .cloned()
                    .unwrap_or_default();
                if candidate_entries.is_empty() {
                    let normalized_key = normalize_identifier(&occurrence.key).to_ascii_lowercase();
                    candidate_entries = self
                        .id_keys
                        .get(&normalized_key)
                        .cloned()
                        .unwrap_or_default();
                }

                let candidate_entries = dedupe_reference_entries(candidate_entries);
                match candidate_entries.as_slice() {
                    [] => MatchResult::Missing,
                    [entry] => MatchResult::Matched {
                        reference_id: entry.id.clone(),
                        confidence: CitationMatchConfidence::Exact,
                        match_source: ReferenceMatchSource::IdFallback,
                    },
                    entries => MatchResult::Ambiguous(
                        entries.iter().map(|entry| entry.id.clone()).collect(),
                    ),
                }
            }
            CitationStyle::Numeric | CitationStyle::FootnoteLike => occurrence
                .key
                .parse::<usize>()
                .ok()
                .and_then(|number| number.checked_sub(1))
                .and_then(|index| self.references.items.get(index))
                .map(|item| MatchResult::Matched {
                    reference_id: item.id.clone(),
                    confidence: CitationMatchConfidence::Exact,
                    match_source: ReferenceMatchSource::AuthorKey,
                })
                .unwrap_or(MatchResult::Missing),
            CitationStyle::AuthorDate => {
                let (year, suffix) = extract_year_details(&occurrence.text);
                let mut candidate_entries = Vec::new();
                let mut match_source = None;
                let mut fallback_candidate: Option<(Vec<ReferenceEntry>, ReferenceMatchSource)> =
                    None;
                for key in citation_author_key_candidates(&occurrence.key) {
                    let Some(entries) = self.author_keys.get(&key) else {
                        if let Some(entries) = self.fallback_keys.get(&key) {
                            let title_entries = dedupe_reference_entries(
                                entries
                                    .iter()
                                    .filter(|entry| {
                                        entry.source == ReferenceMatchSource::TitleFallback
                                    })
                                    .cloned()
                                    .collect(),
                            );
                            if !title_entries.is_empty() {
                                let should_replace = match fallback_candidate.as_ref() {
                                    None => true,
                                    Some((_, source)) => {
                                        *source == ReferenceMatchSource::IdFallback
                                    }
                                };
                                if should_replace {
                                    fallback_candidate =
                                        Some((title_entries, ReferenceMatchSource::TitleFallback));
                                }
                                continue;
                            }

                            let id_entries = dedupe_reference_entries(
                                entries
                                    .iter()
                                    .filter(|entry| {
                                        entry.source == ReferenceMatchSource::IdFallback
                                    })
                                    .cloned()
                                    .collect(),
                            );
                            if !id_entries.is_empty() && fallback_candidate.is_none() {
                                fallback_candidate =
                                    Some((id_entries, ReferenceMatchSource::IdFallback));
                            }
                        }
                        continue;
                    };
                    candidate_entries = dedupe_reference_entries(entries.clone());
                    match_source = Some(ReferenceMatchSource::AuthorKey);
                    break;
                }

                if candidate_entries.is_empty() {
                    if let Some((entries, source)) = fallback_candidate {
                        candidate_entries = entries;
                        match_source = Some(source);
                    } else {
                        return MatchResult::Missing;
                    }
                }

                if let Some(year) = year {
                    let year_matches = candidate_entries
                        .iter()
                        .filter(|entry| entry.year == Some(year))
                        .cloned()
                        .collect::<Vec<_>>();
                    if !year_matches.is_empty() {
                        candidate_entries = year_matches;
                    } else if candidate_entries.iter().any(|entry| entry.year.is_some()) {
                        return MatchResult::Missing;
                    }
                }

                if candidate_entries.len() == 1 {
                    return MatchResult::Matched {
                        reference_id: candidate_entries[0].id.clone(),
                        confidence: CitationMatchConfidence::Probable,
                        match_source: match_source.unwrap_or(ReferenceMatchSource::AuthorKey),
                    };
                }

                if let Some(suffix) = suffix
                    && let Some(entry) = disambiguate_same_year_suffix(&candidate_entries, suffix)
                {
                    return MatchResult::Matched {
                        reference_id: entry.id.clone(),
                        confidence: CitationMatchConfidence::Probable,
                        match_source: match_source.unwrap_or(ReferenceMatchSource::AuthorKey),
                    };
                }

                MatchResult::Ambiguous(
                    candidate_entries
                        .into_iter()
                        .map(|entry| entry.id)
                        .collect(),
                )
            }
        }
    }
}

fn reference_author_keys(item: &CslItem) -> Vec<String> {
    let mut author_components = Vec::new();
    if let Some(authors) = item.extra.get("author").and_then(|value| value.as_array()) {
        for author in authors {
            if let Some(literal) = author.get("literal").and_then(|value| value.as_str()) {
                let key = normalize_identifier(literal).to_ascii_lowercase();
                if !key.is_empty() {
                    author_components.push(key);
                    continue;
                }
            }
            if let Some(family) = author.get("family").and_then(|value| value.as_str()) {
                let key = normalize_identifier(family).to_ascii_lowercase();
                if !key.is_empty() {
                    author_components.push(key);
                }
            }
        }
    }

    let mut keys = Vec::new();

    if author_components.len() > 1 {
        keys.push(author_components.join(" "));
    }

    if author_components.len() >= 3 {
        keys.push(format!("{} et al", author_components[0]));
    }

    keys.extend(author_components);
    keys
}

fn reference_fallback_keys(item: &CslItem) -> Vec<(String, ReferenceMatchSource)> {
    let mut keys = Vec::new();

    if item
        .extra
        .get("author")
        .and_then(|value| value.as_array())
        .is_some_and(|authors| !authors.is_empty())
    {
        return keys;
    }

    if let Some(title) = item.title.as_deref() {
        let title_key = normalize_title(title)
            .to_ascii_lowercase()
            .split_whitespace()
            .next()
            .unwrap_or_default()
            .to_string();
        if !title_key.is_empty() {
            keys.push((title_key, ReferenceMatchSource::TitleFallback));
        }
    }

    let id = normalize_identifier(&item.id).to_ascii_lowercase();
    if let Some(first) = id.split(['-', ' ', '_']).find(|part| !part.is_empty()) {
        keys.push((first.to_string(), ReferenceMatchSource::IdFallback));
    }

    keys
}

fn reference_year(item: &CslItem) -> Option<i32> {
    let issued = item.extra.get("issued")?;
    let date_parts = issued.get("date-parts")?.as_array()?;
    let first_part = date_parts.first()?.as_array()?;
    let first_year = first_part.first()?;
    first_year
        .as_i64()
        .and_then(|year| {
            (i32::MIN as i64..=i32::MAX as i64)
                .contains(&year)
                .then_some(year as i32)
        })
        .or_else(|| {
            first_year
                .as_str()
                .and_then(|value| value.parse::<i32>().ok())
        })
}

fn dedupe_reference_entries(entries: Vec<ReferenceEntry>) -> Vec<ReferenceEntry> {
    let mut by_id = BTreeMap::<String, ReferenceEntry>::new();
    for entry in entries {
        by_id
            .entry(entry.id.clone())
            .and_modify(|existing| {
                if entry.order < existing.order {
                    *existing = entry.clone();
                }
            })
            .or_insert(entry);
    }

    let mut entries = by_id.into_values().collect::<Vec<_>>();
    entries.sort_by_key(|entry| entry.order);
    entries
}

#[derive(Clone)]
struct ReferenceEntry {
    id: String,
    year: Option<i32>,
    order: usize,
    source: ReferenceMatchSource,
}

fn disambiguate_same_year_suffix(
    candidates: &[ReferenceEntry],
    suffix: char,
) -> Option<ReferenceEntry> {
    let suffix_index = suffix.to_ascii_lowercase() as usize;
    if suffix_index < 'a' as usize {
        return None;
    }
    let target_index = suffix_index - 'a' as usize;
    let mut ordered = candidates.to_vec();
    ordered.sort_by_key(|entry| entry.order);
    ordered.get(target_index).cloned()
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use serde_json::json;

    use super::*;
    use crate::csl::CslItem;

    #[test]
    fn latex_citations_match_reference_ids() {
        let references = CslDocument {
            items: vec![
                CslItem {
                    id: "smith2024".to_string(),
                    item_type: "article-journal".to_string(),
                    title: Some("Smith study".to_string()),
                    doi: None,
                    extra: BTreeMap::new(),
                },
                CslItem {
                    id: "doe-2023".to_string(),
                    item_type: "article-journal".to_string(),
                    title: Some("Doe study".to_string()),
                    doi: None,
                    extra: BTreeMap::new(),
                },
            ],
        };

        let report = reconcile_citations(
            r"Evidence draws on \cite{smith2024, doe-2023}.",
            &references,
        );

        assert_eq!(report.occurrences.len(), 2);
        assert_eq!(
            report
                .matches
                .iter()
                .map(|citation| citation.reference_id.as_str())
                .collect::<Vec<_>>(),
            ["smith2024", "doe-2023"]
        );
        assert!(report.issues.is_empty());
    }

    #[test]
    fn latex_citations_skip_optional_arguments_and_starred_forms() {
        let references = CslDocument {
            items: vec![
                CslItem {
                    id: "smith2024".to_string(),
                    item_type: "article-journal".to_string(),
                    title: Some("Smith study".to_string()),
                    doi: None,
                    extra: BTreeMap::new(),
                },
                CslItem {
                    id: "doe-2023".to_string(),
                    item_type: "article-journal".to_string(),
                    title: Some("Doe study".to_string()),
                    doi: None,
                    extra: BTreeMap::new(),
                },
            ],
        };

        let report = reconcile_citations(
            r"See \parencite[see][12]{smith2024} and \textcite*{doe-2023}.",
            &references,
        );

        assert_eq!(
            report
                .occurrences
                .iter()
                .map(|occurrence| (&occurrence.style, occurrence.key.as_str()))
                .collect::<Vec<_>>(),
            [
                (&CitationStyle::Identifier, "smith2024"),
                (&CitationStyle::Identifier, "doe-2023")
            ]
        );
        assert_eq!(report.matches.len(), 2);
        assert!(report.issues.is_empty());
    }

    #[test]
    fn latex_unknown_keys_report_missing_reference() {
        let references = CslDocument {
            items: vec![CslItem {
                id: "smith2024".to_string(),
                item_type: "article-journal".to_string(),
                title: Some("Smith study".to_string()),
                doi: None,
                extra: BTreeMap::new(),
            }],
        };

        let report = reconcile_citations(r"Evidence draws on \cite{missing2024}.", &references);

        assert!(
            report
                .issues
                .iter()
                .any(|issue| issue.issue_type == CitationReconciliationIssueType::MissingReference)
        );
        assert!(
            report
                .issues
                .iter()
                .any(|issue| issue.issue_type == CitationReconciliationIssueType::UncitedReference)
        );
    }

    #[test]
    fn latex_optional_numeric_arguments_are_not_numeric_citations() {
        let references = CslDocument {
            items: vec![CslItem {
                id: "smith2024".to_string(),
                item_type: "article-journal".to_string(),
                title: Some("Smith study".to_string()),
                doi: None,
                extra: BTreeMap::new(),
            }],
        };

        let report = reconcile_citations(r"See \parencite[see][12]{smith2024}.", &references);

        assert_eq!(report.occurrences.len(), 1);
        assert_eq!(report.occurrences[0].style, CitationStyle::Identifier);
        assert_eq!(report.matches[0].reference_id, "smith2024");
        assert!(report.issues.is_empty());
    }

    #[test]
    fn latex_mode_ignores_non_citation_parentheticals() {
        let references = CslDocument {
            items: vec![CslItem {
                id: "smith2024".to_string(),
                item_type: "article-journal".to_string(),
                title: Some("Smith study".to_string()),
                doi: None,
                extra: BTreeMap::new(),
            }],
        };

        let report = reconcile_citations(
            r"Model inputs (mean, 2024 dollars) are documented in \cite{smith2024}.",
            &references,
        );

        assert_eq!(report.occurrences.len(), 1);
        assert_eq!(report.occurrences[0].key, "smith2024");
        assert_eq!(report.matches.len(), 1);
        assert!(report.issues.is_empty());
    }

    #[test]
    fn latex_repeated_reference_keys_are_not_duplicate_citation_issues() {
        let references = CslDocument {
            items: vec![CslItem {
                id: "smith2024".to_string(),
                item_type: "article-journal".to_string(),
                title: Some("Smith study".to_string()),
                doi: None,
                extra: BTreeMap::new(),
            }],
        };

        let report = reconcile_citations(
            r"First use \cite{smith2024}. Later use \parencite{smith2024}.",
            &references,
        );

        assert_eq!(report.occurrences.len(), 2);
        assert_eq!(report.matches.len(), 2);
        assert!(report.issues.is_empty());
    }

    #[test]
    fn author_date_citations_match_reference_author_keys() {
        let references = CslDocument {
            items: vec![CslItem {
                id: "smith-2024".to_string(),
                item_type: "article-journal".to_string(),
                title: Some("Trial".to_string()),
                doi: None,
                extra: BTreeMap::from([("author".to_string(), json!([{"family": "Smith"}]))]),
            }],
        };

        let report = reconcile_citations("Evidence supports this (Smith, 2024).", &references);

        assert_eq!(report.occurrences.len(), 1);
        assert_eq!(report.matches[0].reference_id, "smith-2024");
        assert!(report.issues.is_empty());
    }

    #[test]
    fn institutional_authors_match_full_phrase_without_first_token_false_positives() {
        let references = CslDocument {
            items: vec![
                CslItem {
                    id: "institution-2024".to_string(),
                    item_type: "report".to_string(),
                    title: Some("Institutional report".to_string()),
                    doi: None,
                    extra: BTreeMap::from([
                        (
                            "author".to_string(),
                            json!([{"literal": "National Institute for Health and Care Excellence"}]),
                        ),
                        ("issued".to_string(), json!({"date-parts": [[2024]]})),
                    ]),
                },
                CslItem {
                    id: "national-2024".to_string(),
                    item_type: "article-journal".to_string(),
                    title: Some("A person with a common surname".to_string()),
                    doi: None,
                    extra: BTreeMap::from([
                        ("author".to_string(), json!([{"family": "National"}])),
                        ("issued".to_string(), json!({"date-parts": [[2024]]})),
                    ]),
                },
            ],
        };

        let report = reconcile_citations(
            "The guidance was updated by (National Institute for Health and Care Excellence, 2024).",
            &references,
        );

        assert_eq!(report.matches.len(), 1);
        assert_eq!(report.matches[0].reference_id, "institution-2024");
        assert!(
            report.issues.iter().all(|issue| matches!(
                issue.issue_type,
                CitationReconciliationIssueType::UncitedReference
            )),
            "institutional match should not introduce missing or ambiguous citation issues: {:?}",
            report.issues
        );
    }

    #[test]
    fn same_author_same_year_suffixes_are_resolved_deterministically() {
        let references = CslDocument {
            items: vec![
                CslItem {
                    id: "smith-2024a".to_string(),
                    item_type: "article-journal".to_string(),
                    title: Some("Alpha".to_string()),
                    doi: None,
                    extra: BTreeMap::from([
                        ("author".to_string(), json!([{"family": "Smith"}])),
                        ("issued".to_string(), json!({"date-parts": [[2024]]})),
                    ]),
                },
                CslItem {
                    id: "smith-2024b".to_string(),
                    item_type: "article-journal".to_string(),
                    title: Some("Beta".to_string()),
                    doi: None,
                    extra: BTreeMap::from([
                        ("author".to_string(), json!([{"family": "Smith"}])),
                        ("issued".to_string(), json!({"date-parts": [[2024]]})),
                    ]),
                },
            ],
        };

        let report = reconcile_citations(
            "Prior work (Smith, 2024a) and related findings (Smith, 2024b) shaped the analysis.",
            &references,
        );

        assert_eq!(
            report
                .matches
                .iter()
                .map(|citation| citation.reference_id.as_str())
                .collect::<Vec<_>>(),
            ["smith-2024a", "smith-2024b"]
        );
        assert!(report.issues.is_empty());
    }

    #[test]
    fn multi_author_author_date_citations_match_full_sequences_before_first_author_fallback() {
        let references = CslDocument {
            items: vec![
                CslItem {
                    id: "smith-jones-brown-2024".to_string(),
                    item_type: "article-journal".to_string(),
                    title: Some("Multi-author study".to_string()),
                    doi: None,
                    extra: BTreeMap::from([
                        (
                            "author".to_string(),
                            json!([
                                {"family": "Smith"},
                                {"family": "Jones"},
                                {"family": "Brown"}
                            ]),
                        ),
                        ("issued".to_string(), json!({"date-parts": [[2024]]})),
                    ]),
                },
                CslItem {
                    id: "smith-2024".to_string(),
                    item_type: "article-journal".to_string(),
                    title: Some("Single-author study".to_string()),
                    doi: None,
                    extra: BTreeMap::from([
                        ("author".to_string(), json!([{"family": "Smith"}])),
                        ("issued".to_string(), json!({"date-parts": [[2024]]})),
                    ]),
                },
            ],
        };

        let report = reconcile_citations(
            "Earlier work (Smith, Jones, and Brown, 2024) established the baseline.",
            &references,
        );

        assert_eq!(report.matches.len(), 1);
        assert_eq!(report.matches[0].reference_id, "smith-jones-brown-2024");
        assert!(
            report
                .issues
                .iter()
                .all(|issue| issue.issue_type == CitationReconciliationIssueType::UncitedReference),
            "multi-author citation should not create ambiguity or missing-reference noise: {:?}",
            report.issues
        );
    }

    #[test]
    fn et_al_variants_prefer_three_author_matches_over_single_author_fallbacks() {
        let references = CslDocument {
            items: vec![
                CslItem {
                    id: "smith-three-authors-2024".to_string(),
                    item_type: "article-journal".to_string(),
                    title: Some("Three-author study".to_string()),
                    doi: None,
                    extra: BTreeMap::from([
                        (
                            "author".to_string(),
                            json!([
                                {"family": "Smith"},
                                {"family": "Jones"},
                                {"family": "Brown"}
                            ]),
                        ),
                        ("issued".to_string(), json!({"date-parts": [[2024]]})),
                    ]),
                },
                CslItem {
                    id: "smith-two-authors-2024".to_string(),
                    item_type: "article-journal".to_string(),
                    title: Some("Two-author study".to_string()),
                    doi: None,
                    extra: BTreeMap::from([
                        (
                            "author".to_string(),
                            json!([{"family": "Smith"}, {"family": "Jones"}]),
                        ),
                        ("issued".to_string(), json!({"date-parts": [[2024]]})),
                    ]),
                },
            ],
        };

        let report = reconcile_citations(
            "Related findings (Smith et al., 2024) were later expanded.",
            &references,
        );

        assert_eq!(report.matches.len(), 1);
        assert_eq!(report.matches[0].reference_id, "smith-three-authors-2024");
        assert!(
            report
                .issues
                .iter()
                .all(|issue| issue.issue_type == CitationReconciliationIssueType::UncitedReference),
            "et al. citation should resolve without ambiguity noise: {:?}",
            report.issues
        );
    }

    #[test]
    fn numeric_citations_report_missing_uncited_duplicate_and_order_issues() {
        let references = CslDocument {
            items: vec![
                CslItem {
                    id: "first".to_string(),
                    item_type: "article-journal".to_string(),
                    title: Some("First".to_string()),
                    doi: None,
                    extra: BTreeMap::new(),
                },
                CslItem {
                    id: "second".to_string(),
                    item_type: "article-journal".to_string(),
                    title: Some("Second".to_string()),
                    doi: None,
                    extra: BTreeMap::new(),
                },
            ],
        };

        let report = reconcile_citations(
            "Prior work [2] disagrees with [1] and [3]. [1]",
            &references,
        );
        let issue_types = report
            .issues
            .iter()
            .map(|issue| issue.issue_type)
            .collect::<Vec<_>>();

        assert!(issue_types.contains(&CitationReconciliationIssueType::NumericOrder));
        assert!(issue_types.contains(&CitationReconciliationIssueType::MissingReference));
        assert!(issue_types.contains(&CitationReconciliationIssueType::DuplicateCitation));
        assert!(!issue_types.contains(&CitationReconciliationIssueType::UncitedReference));
    }

    #[test]
    fn ambiguous_author_matches_are_reported_for_manual_review() {
        let references = CslDocument {
            items: vec![
                CslItem {
                    id: "smith-a".to_string(),
                    item_type: "article-journal".to_string(),
                    title: Some("A".to_string()),
                    doi: None,
                    extra: BTreeMap::from([("author".to_string(), json!([{"family": "Smith"}]))]),
                },
                CslItem {
                    id: "smith-b".to_string(),
                    item_type: "article-journal".to_string(),
                    title: Some("B".to_string()),
                    doi: None,
                    extra: BTreeMap::from([("author".to_string(), json!([{"family": "Smith"}]))]),
                },
            ],
        };

        let report = reconcile_citations("(Smith, 2024)", &references);

        assert_eq!(
            report.issues[0].issue_type,
            CitationReconciliationIssueType::AmbiguousMatch
        );
        assert!(report.to_markdown().contains("ambiguous_match"));
    }

    #[test]
    fn mixed_style_manuscripts_emit_one_style_drift_issue() {
        let references = CslDocument {
            items: vec![
                CslItem {
                    id: "smith-2024".to_string(),
                    item_type: "article-journal".to_string(),
                    title: Some("Author-date".to_string()),
                    doi: None,
                    extra: BTreeMap::from([
                        ("author".to_string(), json!([{"family": "Smith"}])),
                        ("issued".to_string(), json!({"date-parts": [[2024]]})),
                    ]),
                },
                CslItem {
                    id: "report-2024".to_string(),
                    item_type: "report".to_string(),
                    title: Some("Report".to_string()),
                    doi: None,
                    extra: BTreeMap::from([(
                        "issued".to_string(),
                        json!({"date-parts": [[2024]]}),
                    )]),
                },
            ],
        };

        let report = reconcile_citations(
            "The results were consistent with prior work (Smith, 2024) and the later summary [2].",
            &references,
        );

        assert_eq!(report.matches.len(), 2);
        assert!(
            report
                .issues
                .iter()
                .any(|issue| issue.issue_type
                    == CitationReconciliationIssueType::MixedStyleManuscript)
        );
        assert!(report.issues.iter().all(|issue| matches!(
            issue.issue_type,
            CitationReconciliationIssueType::MixedStyleManuscript
                | CitationReconciliationIssueType::UncitedReference
        )));
    }

    #[test]
    fn title_fallback_matches_are_reported_explicitly() {
        let references = CslDocument {
            items: vec![CslItem {
                id: "guideline-2024".to_string(),
                item_type: "report".to_string(),
                title: Some("Guideline".to_string()),
                doi: None,
                extra: BTreeMap::from([("issued".to_string(), json!({"date-parts": [[2024]]}))]),
            }],
        };

        let report =
            reconcile_citations("The update was codified in (Guideline, 2024).", &references);

        assert_eq!(report.matches.len(), 1);
        assert_eq!(report.matches[0].reference_id, "guideline-2024");
        assert!(
            report.issues.iter().any(
                |issue| issue.issue_type == CitationReconciliationIssueType::TitleFallbackMatch
            )
        );
        assert!(report.issues.iter().all(|issue| matches!(
            issue.issue_type,
            CitationReconciliationIssueType::TitleFallbackMatch
                | CitationReconciliationIssueType::UncitedReference
        )));
    }

    #[test]
    fn author_matches_take_precedence_over_title_fallback_keys() {
        let references = CslDocument {
            items: vec![
                CslItem {
                    id: "smith-2024".to_string(),
                    item_type: "article-journal".to_string(),
                    title: Some("Real author-backed reference".to_string()),
                    doi: None,
                    extra: BTreeMap::from([
                        ("author".to_string(), json!([{"family": "Smith"}])),
                        ("issued".to_string(), json!({"date-parts": [[2024]]})),
                    ]),
                },
                CslItem {
                    id: "title-smith-2024".to_string(),
                    item_type: "report".to_string(),
                    title: Some("Smith".to_string()),
                    doi: None,
                    extra: BTreeMap::from([(
                        "issued".to_string(),
                        json!({"date-parts": [[2024]]}),
                    )]),
                },
            ],
        };

        let report =
            reconcile_citations("The baseline was described in (Smith, 2024).", &references);

        assert_eq!(report.matches.len(), 1);
        assert_eq!(report.matches[0].reference_id, "smith-2024");
        assert!(
            report.issues.iter().all(
                |issue| issue.issue_type != CitationReconciliationIssueType::TitleFallbackMatch
            )
        );
    }
}
