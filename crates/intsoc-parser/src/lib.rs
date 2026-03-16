// SPDX-License-Identifier: PMPL-1.0-or-later

//! Internet Society Document Parser
//!
//! Parses RFC XML v3, plain-text Internet-Drafts, and idnits output
//! into the unified `Document` model.

#![forbid(unsafe_code)]
pub mod idnits;
pub mod plain_text;
pub mod xml;

use intsoc_core::document::Document;
use thiserror::Error;

/// Parser errors.
#[derive(Debug, Error)]
pub enum ParseError {
    #[error("XML parse error: {0}")]
    Xml(#[from] quick_xml::Error),

    #[error("XML deserialization error: {0}")]
    XmlDeserialize(#[from] quick_xml::DeError),

    #[error("plain-text parse error at line {line}: {message}")]
    PlainText { line: u32, message: String },

    #[error("idnits parse error: {0}")]
    Idnits(String),

    #[error("unsupported format: {0}")]
    UnsupportedFormat(String),
}

/// Parse a document from its source, auto-detecting format.
pub fn parse(source: &str) -> Result<Document, ParseError> {
    let trimmed = source.trim_start();
    if trimmed.starts_with("<?xml") || trimmed.starts_with("<rfc") {
        xml::parse_xml(source)
    } else {
        plain_text::parse_plain_text(source)
    }
}
