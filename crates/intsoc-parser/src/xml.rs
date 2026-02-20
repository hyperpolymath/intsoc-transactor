// SPDX-License-Identifier: PMPL-1.0-or-later

//! RFC XML v3 (RFC 7991) parser using quick-xml.

use intsoc_core::document::{
    Author, Category, Document, DocumentFormat, IprDeclaration,
};
use intsoc_core::stream::Stream;

use crate::ParseError;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;

/// Parse an RFC XML v3 document.
#[allow(unused_assignments, unused_variables)]
pub fn parse_xml(source: &str) -> Result<Document, ParseError> {
    let mut reader = Reader::from_str(source);
    reader.config_mut().trim_text(true);

    let mut doc = Document::new(String::new(), Stream::IetfIndividual);
    doc.format = DocumentFormat::XmlV3;
    doc.source = source.to_string();

    let mut in_front = false;
    let mut in_middle = false;
    let mut in_back = false;
    let mut in_abstract = false;
    let mut in_title = false;
    let mut in_references = false;
    let mut current_ref_type = ReferenceType::None;
    let mut current_section_name = String::new();
    let mut depth = 0u32;
    let mut abstract_depth = 0u32;
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Eof) => break,
            Ok(Event::Start(ref e)) => {
                depth += 1;
                match e.name().as_ref() {
                    b"rfc" => parse_rfc_attrs(e, &mut doc)?,
                    b"front" => in_front = true,
                    b"middle" => in_middle = true,
                    b"back" => {
                        in_back = true;
                        in_middle = false;
                    }
                    b"title" if in_front && !in_references => in_title = true,
                    b"abstract" if in_front => {
                        in_abstract = true;
                        abstract_depth = depth;
                    }
                    b"author" if in_front && !in_references => {
                        let author = parse_author_attrs(e)?;
                        doc.authors.push(author);
                    }
                    b"date" if in_front && !in_references => {
                        parse_date_attrs(e, &mut doc)?;
                    }
                    b"references" if in_back => {
                        in_references = true;
                        current_ref_type = ReferenceType::None;
                    }
                    b"name" if in_references => {
                        // Will capture text in Text event
                    }
                    b"section" if in_middle => {
                        current_section_name = get_attr_str(e, b"title").unwrap_or_default();
                    }
                    _ => {}
                }
            }
            Ok(Event::End(ref e)) => {
                match e.name().as_ref() {
                    b"front" => in_front = false,
                    b"middle" => in_middle = false,
                    b"back" => in_back = false,
                    b"title" => in_title = false,
                    b"abstract" => in_abstract = false,
                    b"references" => {
                        in_references = false;
                        current_ref_type = ReferenceType::None;
                    }
                    _ => {}
                }
                depth = depth.saturating_sub(1);
            }
            Ok(Event::Text(ref e)) => {
                let text = e.unescape().map_err(quick_xml::Error::from)?.into_owned();
                if in_title && in_front && !in_references {
                    doc.title = text;
                    in_title = false;
                } else if in_abstract && depth > abstract_depth {
                    if let Some(ref mut abs) = doc.abstract_text {
                        abs.push(' ');
                        abs.push_str(&text);
                    } else {
                        doc.abstract_text = Some(text);
                    }
                } else if in_references && current_section_name.is_empty() {
                    // Capture references section name for classifying normative vs informative
                    let lower = text.to_lowercase();
                    if lower.contains("normative") {
                        current_ref_type = ReferenceType::Normative;
                    } else if lower.contains("informative") {
                        current_ref_type = ReferenceType::Informative;
                    }
                }
            }
            Ok(Event::Empty(ref e)) => {
                match e.name().as_ref() {
                    b"author" if in_front && !in_references => {
                        let author = parse_author_attrs(e)?;
                        doc.authors.push(author);
                    }
                    b"date" if in_front && !in_references => {
                        parse_date_attrs(e, &mut doc)?;
                    }
                    b"seriesInfo" if in_front => {
                        // Check for draft name
                        if let (Some(name), Some(value)) =
                            (get_attr_str(e, b"name"), get_attr_str(e, b"value"))
                        {
                            if name == "Internet-Draft" {
                                doc.name = value;
                            }
                        }
                    }
                    _ => {}
                }
            }
            Err(e) => return Err(ParseError::Xml(e)),
            _ => {}
        }
        buf.clear();
    }

    // Detect stream from document name
    if doc.name.starts_with("draft-ietf-") {
        let parts: Vec<&str> = doc.name.strip_prefix("draft-ietf-").unwrap().splitn(2, '-').collect();
        if !parts.is_empty() {
            doc.stream = Stream::IetfWorkingGroup {
                wg: parts[0].to_string(),
            };
        }
    } else if doc.name.starts_with("draft-irtf-") {
        let parts: Vec<&str> = doc.name.strip_prefix("draft-irtf-").unwrap().splitn(2, '-').collect();
        if !parts.is_empty() {
            doc.stream = Stream::IrtfResearchGroup {
                rg: parts[0].to_string(),
            };
        }
    }
    // Otherwise remains IetfIndividual

    // Detect boilerplate presence
    doc.has_boilerplate = doc.ipr.is_some();

    Ok(doc)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ReferenceType {
    None,
    Normative,
    Informative,
}

fn parse_rfc_attrs(e: &BytesStart, doc: &mut Document) -> Result<(), ParseError> {
    if let Some(ipr) = get_attr_str(e, b"ipr") {
        doc.ipr = match ipr.as_str() {
            "trust200902" => Some(IprDeclaration::Trust200902),
            "noModificationTrust200902" => Some(IprDeclaration::NoModificationTrust200902),
            "noDerivativesTrust200902" => Some(IprDeclaration::NoDerivativesTrust200902),
            "pre5378Trust200902" => Some(IprDeclaration::Pre5378Trust200902),
            _ => None,
        };
    }
    if let Some(cat) = get_attr_str(e, b"category") {
        doc.category = match cat.as_str() {
            "std" => Some(Category::StandardsTrack),
            "info" => Some(Category::Informational),
            "exp" => Some(Category::Experimental),
            "bcp" => Some(Category::BestCurrentPractice),
            "historic" => Some(Category::Historic),
            _ => None,
        };
    }
    if let Some(name) = get_attr_str(e, b"docName") {
        doc.name = name;
    }
    if let Some(obsoletes) = get_attr_str(e, b"obsoletes") {
        doc.obsoletes = obsoletes
            .split(',')
            .filter_map(|s| s.trim().parse().ok())
            .collect();
    }
    if let Some(updates) = get_attr_str(e, b"updates") {
        doc.updates = updates
            .split(',')
            .filter_map(|s| s.trim().parse().ok())
            .collect();
    }
    Ok(())
}

fn parse_author_attrs(e: &BytesStart) -> Result<Author, ParseError> {
    Ok(Author {
        fullname: get_attr_str(e, b"fullname").unwrap_or_default(),
        initials: get_attr_str(e, b"initials"),
        surname: get_attr_str(e, b"surname").unwrap_or_default(),
        organization: None,
        email: None,
        role: get_attr_str(e, b"role"),
    })
}

fn parse_date_attrs(e: &BytesStart, doc: &mut Document) -> Result<(), ParseError> {
    let year = get_attr_str(e, b"year")
        .and_then(|y| y.parse::<i32>().ok());
    let month = get_attr_str(e, b"month")
        .and_then(|m| parse_month(&m));
    let day = get_attr_str(e, b"day")
        .and_then(|d| d.parse::<u32>().ok())
        .unwrap_or(1);

    if let (Some(year), Some(month)) = (year, month) {
        doc.date = chrono::NaiveDate::from_ymd_opt(year, month, day);
    }
    Ok(())
}

fn parse_month(m: &str) -> Option<u32> {
    match m.to_lowercase().as_str() {
        "january" | "jan" | "1" => Some(1),
        "february" | "feb" | "2" => Some(2),
        "march" | "mar" | "3" => Some(3),
        "april" | "apr" | "4" => Some(4),
        "may" | "5" => Some(5),
        "june" | "jun" | "6" => Some(6),
        "july" | "jul" | "7" => Some(7),
        "august" | "aug" | "8" => Some(8),
        "september" | "sep" | "9" => Some(9),
        "october" | "oct" | "10" => Some(10),
        "november" | "nov" | "11" => Some(11),
        "december" | "dec" | "12" => Some(12),
        _ => m.parse().ok(),
    }
}

fn get_attr_str(e: &BytesStart, name: &[u8]) -> Option<String> {
    e.attributes()
        .filter_map(|a| a.ok())
        .find(|a| a.key.as_ref() == name)
        .map(|a| String::from_utf8_lossy(&a.value).into_owned())
}
