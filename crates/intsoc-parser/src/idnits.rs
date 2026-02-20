// SPDX-License-Identifier: PMPL-1.0-or-later

//! Parser for idnits (ID-Nits) output.
//!
//! Converts idnits checker output into `CheckResult` entries.

use intsoc_core::validation::{
    CheckCategory, CheckResult, Fixability, Severity,
};

use crate::ParseError;

/// Parse idnits output into check results.
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

fn parse_idnits_line(line: &str) -> Option<CheckResult> {
    // idnits format: "  ** Error: <message>"
    //                "  ~~ Warning: <message>"
    //                "  -- Comment: <message>"
    let (severity, rest) = if line.contains("** Error:") || line.contains("**") {
        (Severity::Error, line.trim_start_matches(|c: char| c == '*' || c.is_whitespace()))
    } else if line.contains("~~ Warning:") || line.contains("~~") {
        (Severity::Warning, line.trim_start_matches(|c: char| c == '~' || c.is_whitespace()))
    } else if line.contains("-- Comment:") || line.contains("--") && !line.starts_with("---") {
        (Severity::Info, line.trim_start_matches(|c: char| c == '-' || c.is_whitespace()))
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

    let (category, fixable) = categorize_idnits_message(&message);

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

fn categorize_idnits_message(msg: &str) -> (CheckCategory, Fixability) {
    let lower = msg.to_lowercase();

    if lower.contains("boilerplate") {
        (CheckCategory::Boilerplate, Fixability::AutoSafe)
    } else if lower.contains("date") || lower.contains("expir") {
        (CheckCategory::Date, Fixability::AutoSafe)
    } else if lower.contains("reference") || lower.contains("normative") || lower.contains("informative") {
        (CheckCategory::References, Fixability::Recommended)
    } else if lower.contains("section") || lower.contains("iana") {
        (CheckCategory::Sections, Fixability::ManualOnly)
    } else if lower.contains("header") || lower.contains("title") {
        (CheckCategory::Header, Fixability::Recommended)
    } else if lower.contains("draft name") || lower.contains("filename") {
        (CheckCategory::DraftName, Fixability::AutoSafe)
    } else if lower.contains("ipr") {
        (CheckCategory::Ipr, Fixability::Recommended)
    } else {
        (CheckCategory::TextFormat, Fixability::ManualOnly)
    }
}

fn category_to_id(cat: CheckCategory) -> &'static str {
    match cat {
        CheckCategory::Boilerplate => "boilerplate",
        CheckCategory::Date => "date",
        CheckCategory::Header => "header",
        CheckCategory::References => "references",
        CheckCategory::Sections => "sections",
        CheckCategory::TextFormat => "text-format",
        CheckCategory::Xml => "xml",
        CheckCategory::IanaSections => "iana",
        CheckCategory::DraftName => "draft-name",
        CheckCategory::Ipr => "ipr",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_error_line() {
        let result = parse_idnits_line("  ** Error: Missing boilerplate text");
        assert!(result.is_some());
        let r = result.unwrap();
        assert_eq!(r.severity, Severity::Error);
        assert_eq!(r.category, CheckCategory::Boilerplate);
    }

    #[test]
    fn test_parse_warning_line() {
        let result = parse_idnits_line("  ~~ Warning: Outdated reference to RFC 1234");
        assert!(result.is_some());
        let r = result.unwrap();
        assert_eq!(r.severity, Severity::Warning);
    }
}
