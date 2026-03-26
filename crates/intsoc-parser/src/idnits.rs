// SPDX-License-Identifier: PMPL-1.0-or-later

//! ID-Nits Output Parser.
//!
//! This module parses the output of the legacy `idnits` tool (the
//! standard IETF draft checker) and converts it into structured
//! `CheckResult` objects used by the transactor.
//!
//! DESIGN: Uses line-by-line pattern matching to identify severity levels
//! and categories.

use intsoc_core::validation::{CheckCategory, CheckResult, Fixability, Severity};

use crate::ParseError;

/// ENTRY POINT: Converts a raw idnits text report into a list of findings.
pub fn parse_idnits_output(output: &str) -> Result<Vec<CheckResult>, ParseError> {
    let mut results = Vec::new();

    for line in output.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        if let Some(result) = parse_idnits_line(trimmed) {
            results.push(result);
        }
    }

    Ok(results)
}

/// INTERNAL: Maps an idnits line to a structured result.
/// Patterns:
/// - "** Error:"   -> Severity::Error
/// - "~~ Warning:" -> Severity::Warning
/// - "-- Comment:" -> Severity::Info
fn parse_idnits_line(line: &str) -> Option<CheckResult> {
    let (severity, rest) = if line.contains("** Error:") || line.contains("**") {
        (
            Severity::Error,
            line.trim_start_matches(|c: char| c == '*' || c.is_whitespace()),
        )
    } else if line.contains("~~ Warning:") || line.contains("~~") {
        (
            Severity::Warning,
            line.trim_start_matches(|c: char| c == '~' || c.is_whitespace()),
        )
    } else if line.contains("-- Comment:") || line.contains("--") && !line.starts_with("---") {
        (
            Severity::Info,
            line.trim_start_matches(|c: char| c == '-' || c.is_whitespace()),
        )
    } else {
        return None;
    };

    let message = rest
        .trim_start_matches("Error:")
        .trim_start_matches("Warning:")
        .trim_start_matches("Comment:")
        .trim()
        .to_string();

    if message.is_empty() {
        return None;
    }

    // CLASSIFICATION: Map the text message to a logical category and fixability tier.
    let (category, fixable) = categorize_idnits_message(&message);

    #[allow(clippy::match_same_arms)]
    Some(CheckResult {
        check_id: format!("idnits-{}", category_to_id(category)),
        severity,
        message,
        location: None,
        category,
        fixable,
        suggestion: None,
    })
}

/// CLASSIFICATION: Maps idnits message text to a category and fixability level.
///
/// Uses keyword matching against the idnits output vocabulary to assign each
/// finding to one of the standard `CheckCategory` values.
fn categorize_idnits_message(message: &str) -> (CheckCategory, Fixability) {
    let lower = message.to_lowercase();

    if lower.contains("boilerplate") || lower.contains("trust") || lower.contains("ipr") {
        (CheckCategory::Boilerplate, Fixability::AutoSafe)
    } else if lower.contains("date") || lower.contains("expir") {
        (CheckCategory::Date, Fixability::AutoSafe)
    } else if lower.contains("reference") || lower.contains("normative") {
        (CheckCategory::References, Fixability::ManualOnly)
    } else if lower.contains("section") || lower.contains("abstract") {
        (CheckCategory::Sections, Fixability::ManualOnly)
    } else if lower.contains("line length") || lower.contains("encoding") || lower.contains("tab") {
        (CheckCategory::TextFormat, Fixability::AutoSafe)
    } else if lower.contains("xml") || lower.contains("schema") {
        (CheckCategory::Xml, Fixability::ManualOnly)
    } else if lower.contains("iana") {
        (CheckCategory::IanaSections, Fixability::ManualOnly)
    } else if lower.contains("draft-") || lower.contains("naming") {
        (CheckCategory::DraftName, Fixability::AutoSafe)
    } else if lower.contains("header") || lower.contains("author") || lower.contains("title") {
        (CheckCategory::Header, Fixability::ManualOnly)
    } else {
        // Default: generic header issue, requires manual review.
        (CheckCategory::Header, Fixability::ManualOnly)
    }
}

/// MAPPING: Converts a `CheckCategory` variant to a short kebab-case identifier
/// used for constructing unique check IDs (e.g. "idnits-boilerplate").
const fn category_to_id(category: CheckCategory) -> &'static str {
    match category {
        CheckCategory::Boilerplate => "boilerplate",
        CheckCategory::Date => "date",
        CheckCategory::Header => "header",
        CheckCategory::References => "references",
        CheckCategory::Sections => "sections",
        CheckCategory::TextFormat => "text-format",
        CheckCategory::Xml => "xml",
        CheckCategory::IanaSections => "iana-sections",
        CheckCategory::DraftName => "draft-name",
        CheckCategory::Ipr => "ipr",
    }
}
