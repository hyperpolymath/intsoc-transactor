// SPDX-License-Identifier: PMPL-1.0-or-later

//! Plain-text Internet-Draft parser using winnow.

use intsoc_core::document::{Author, Document, DocumentFormat};
use intsoc_core::stream::Stream;

use crate::ParseError;

/// Parse a plain-text Internet-Draft.
pub fn parse_plain_text(source: &str) -> Result<Document, ParseError> {
    let mut doc = Document::new(String::new(), Stream::IetfIndividual);
    doc.format = DocumentFormat::PlainText;
    doc.source = source.to_string();

    let lines: Vec<&str> = source.lines().collect();

    // Extract metadata from the header block
    parse_header(&lines, &mut doc);

    // Extract abstract
    parse_abstract(&lines, &mut doc);

    // Detect boilerplate
    doc.has_boilerplate = source.contains("Internet Engineering Task Force")
        || source.contains("Request for Comments")
        || source.contains("Internet-Draft");

    Ok(doc)
}

/// Parse the header block (first page) of a plain-text draft.
fn parse_header(lines: &[&str], doc: &mut Document) {
    let mut found_title = false;

    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();

        // Draft name detection
        if trimmed.starts_with("draft-") || trimmed.contains("draft-") {
            if let Some(name) = extract_draft_name(trimmed) {
                doc.name = name;
            }
        }

        // Title detection: centered text that's not a header label
        if !found_title && i > 0 && i < 20 {
            let leading_spaces = line.len() - line.trim_start().len();
            if leading_spaces > 15 && !trimmed.is_empty() && !trimmed.contains(':') {
                doc.title = trimmed.to_string();
                found_title = true;
            }
        }

        // Author detection from header
        if trimmed.starts_with("Authors:") || trimmed.starts_with("Author:") {
            if let Some(author_name) = trimmed.split(':').nth(1) {
                let name = author_name.trim();
                if !name.is_empty() {
                    doc.authors.push(Author {
                        fullname: name.to_string(),
                        initials: None,
                        surname: name.split_whitespace().last().unwrap_or("").to_string(),
                        organization: None,
                        email: None,
                        role: None,
                    });
                }
            }
        }

        // Date detection
        if let Some(date) = try_parse_date(trimmed) {
            doc.date = Some(date);
        }

        // Stop scanning after form feed or too many lines
        if trimmed == "\x0c" || i > 50 {
            break;
        }
    }
}

/// Parse the abstract section.
fn parse_abstract(lines: &[&str], doc: &mut Document) {
    let mut in_abstract = false;
    let mut abstract_lines = Vec::new();

    for line in lines {
        let trimmed = line.trim();

        if trimmed == "Abstract" || trimmed == "abstract" {
            in_abstract = true;
            continue;
        }

        if in_abstract {
            // End of abstract: next section header or blank lines followed by numbered section
            if trimmed.starts_with("1.") || trimmed.starts_with("Table of Contents") {
                break;
            }
            abstract_lines.push(trimmed);
        }
    }

    if !abstract_lines.is_empty() {
        let abstract_text = abstract_lines
            .into_iter()
            .collect::<Vec<_>>()
            .join(" ")
            .trim()
            .to_string();
        if !abstract_text.is_empty() {
            doc.abstract_text = Some(abstract_text);
        }
    }
}

/// Extract a draft name from a line of text.
fn extract_draft_name(text: &str) -> Option<String> {
    // Match "draft-...-NN" pattern
    let start = text.find("draft-")?;
    let rest = &text[start..];
    let end = rest
        .find(|c: char| c.is_whitespace() || c == '>' || c == '"' || c == ']')
        .unwrap_or(rest.len());
    let name = &rest[..end];
    if name.len() > 6 {
        Some(name.to_string())
    } else {
        None
    }
}

/// Try to parse a date from a line.
fn try_parse_date(text: &str) -> Option<chrono::NaiveDate> {
    // Common formats: "January 2026", "February 8, 2026", "2026-02-08"
    let months = [
        ("January", 1),
        ("February", 2),
        ("March", 3),
        ("April", 4),
        ("May", 5),
        ("June", 6),
        ("July", 7),
        ("August", 8),
        ("September", 9),
        ("October", 10),
        ("November", 11),
        ("December", 12),
    ];

    for (name, num) in &months {
        if text.contains(name) {
            // Try "Month Day, Year" or "Month Year"
            let parts: Vec<&str> = text.split_whitespace().collect();
            for (i, part) in parts.iter().enumerate() {
                if *part == *name {
                    if let Some(year_str) = parts.get(i + 1).or(parts.get(i + 2)) {
                        let year_str = year_str.trim_end_matches(',');
                        if let Ok(year) = year_str.parse::<i32>() {
                            if (2000..=2100).contains(&year) {
                                return chrono::NaiveDate::from_ymd_opt(year, *num, 1);
                            }
                        }
                    }
                }
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_draft_name() {
        assert_eq!(
            extract_draft_name("Internet-Draft draft-jewell-http-430-00"),
            Some("draft-jewell-http-430-00".to_string())
        );
    }

    #[test]
    fn test_parse_empty() {
        let doc = parse_plain_text("").expect("TODO: handle error");
        assert_eq!(doc.format, DocumentFormat::PlainText);
    }
}
