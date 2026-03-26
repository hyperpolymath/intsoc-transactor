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

impl Category {
    /// Returns the XML v3 attribute value for this category (RFC 7991).
    #[must_use]
    pub const fn xml_value(self) -> &'static str {
        match self {
            Self::StandardsTrack => "std",
            Self::Informational => "info",
            Self::Experimental => "exp",
            Self::BestCurrentPractice => "bcp",
            Self::Historic => "historic",
        }
    }
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

/// A bibliographic reference (normative or informative).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reference {
    /// Anchor identifier used in the document (e.g. "RFC8174")
    pub anchor: String,
    /// Full display title of the referenced document
    pub title: String,
    /// Target URI (e.g. the DOI or RFC URL)
    pub target: Option<String>,
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

    /// Intended status category (Standards Track, Informational, etc.)
    pub category: Option<Category>,

    /// Abstract text extracted from the document
    pub abstract_text: Option<String>,

    /// IANA considerations section content
    pub iana_considerations: Option<String>,

    /// Whether the document contains the required IETF Trust boilerplate
    pub has_boilerplate: bool,

    /// Raw source text or XML
    pub source: String,

    /// History of previous submissions/revisions
    pub submission_history: Vec<String>,

    /// List of document names this document obsoletes
    pub obsoletes: Vec<String>,

    /// List of document names this document updates
    pub updates: Vec<String>,
}

/// Parsed components of an IETF draft name (e.g. "draft-ietf-httpbis-priority-00").
#[derive(Debug, Clone)]
pub struct DraftNameParts {
    /// The source stream and working-group portion (e.g. "ietf-httpbis-priority")
    pub source_and_name: String,
    /// The revision number suffix (e.g. 0)
    pub version: u32,
}

impl Document {
    /// Parses a draft name into its constituent parts.
    ///
    /// Expects the format `draft-<source-and-name>-<version>` where version
    /// is a two-digit number.
    #[must_use]
    pub fn parse_draft_name(name: &str) -> Option<DraftNameParts> {
        let stripped = name.strip_prefix("draft-")?;
        let last_dash = stripped.rfind('-')?;
        let version_str = &stripped[last_dash + 1..];
        let version: u32 = version_str.parse().ok()?;
        let source_and_name = stripped[..last_dash].to_string();
        Some(DraftNameParts {
            source_and_name,
            version,
        })
    }

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
