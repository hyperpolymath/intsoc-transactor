// SPDX-License-Identifier: PMPL-1.0-or-later

//! Internet Society Document Domain Model.
//!
//! This module defines the primary entities and value objects used to represent 
//! IETF Internet-Drafts and RFCs. It encodes the formal requirements of 
//! RFC 7991 (XML v3) and RFC 8179 (IPR) into Rust's type system.

use crate::stream::Stream;
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

/// Supported source formats for document processing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DocumentFormat {
    /// XML v3 (Current standard - RFC 7991)
    XmlV3,
    /// XML v2 (Legacy standard - RFC 2629)
    XmlV2,
    /// Canonical plain text output
    PlainText,
}

/// IPR (Intellectual Property Rights) declarations as defined in RFC 8179.
/// These identifiers must match the `ipr` attribute in the `<rfc>` tag.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IprDeclaration {
    /// Default: standard IETF Trust provisions
    Trust200902,
    /// No modifications to the document are permitted
    NoModificationTrust200902,
    /// No derivative works are permitted
    NoDerivativesTrust200902,
    /// Historical pre-RFC 5378 status
    Pre5378Trust200902,
}

/// Intended status of the document.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Category {
    /// Standards Track (Proposed/Internet Standard)
    StandardsTrack,
    /// Informational (non-normative)
    Informational,
    /// Experimental (testing new ideas)
    Experimental,
    /// Best Current Practice
    BestCurrentPractice,
    /// Historic (deprecated or superseded)
    Historic,
}

/// Metadata for a document contributor.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Author {
    pub fullname: String,
    pub initials: Option<String>,
    pub surname: String,
    pub organization: Option<String>,
    pub email: Option<String>,
    pub role: Option<String>, // e.g., "editor"
}

/// The core Document record.
/// This is the central "aggregate root" for the transactor system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    /// The unique draft name (e.g. "draft-ietf-httpbis-priority-00")
    pub name: String,

    /// The human-readable title
    pub title: String,

    /// Format of the source content
    pub format: DocumentFormat,

    /// Administrative stream (IETF, IRTF, etc.)
    pub stream: Stream,

    /// Revision count (0-99)
    pub version: u32,

    /// List of authors/editors
    pub authors: Vec<Author>,

    /// Formal IPR declaration
    pub ipr: Option<IprDeclaration>,

    /// Document date (usually current or submission date)
    pub date: Option<NaiveDate>,

    /// Automatic expiry (185 days after `date`)
    pub expires: Option<NaiveDate>,

    /// Mandatory references for implementation
    pub normative_references: Vec<Reference>,

    /// Supporting/contextual references
    pub informative_references: Vec<Reference>,

    /// Raw source text or XML
    pub source: String,
}

impl Document {
    /// FACTORY: Creates a initialized document with default settings.
    #[must_use]
    pub fn new(name: String, stream: Stream) -> Self {
        Self {
            name,
            title: String::new(),
            format: DocumentFormat::XmlV3,
            stream,
            category: None,
            version: 0,
            ipr: Some(IprDeclaration::Trust200902),
            authors: Vec::new(),
            abstract_text: None,
            date: None,
            expires: None,
            normative_references: Vec::new(),
            informative_references: Vec::new(),
            iana_considerations: None,
            has_boilerplate: false,
            source: String::new(),
            submission_history: Vec::new(),
            obsoletes: Vec::new(),
            updates: Vec::new(),
        }
    }
}
