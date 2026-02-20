// SPDX-License-Identifier: PMPL-1.0-or-later

//! Document model for Internet Society submissions.

use crate::stream::Stream;
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

/// Document format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DocumentFormat {
    /// RFC XML v3 (RFC 7991)
    XmlV3,
    /// RFC XML v2 (RFC 2629, legacy)
    XmlV2,
    /// Plain text (RFC format)
    PlainText,
}

/// IPR declaration type (RFC 8179).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IprDeclaration {
    /// IETF Trust Legal Provisions (TLP) 5.0
    Trust200902,
    /// No modification allowed
    NoModificationTrust200902,
    /// No derivatives allowed
    NoDerivativesTrust200902,
    /// Pre-5378 (historical)
    Pre5378Trust200902,
}

impl IprDeclaration {
    /// Returns the XML attribute value for this IPR type.
    #[must_use]
    pub fn xml_value(&self) -> &'static str {
        match self {
            Self::Trust200902 => "trust200902",
            Self::NoModificationTrust200902 => "noModificationTrust200902",
            Self::NoDerivativesTrust200902 => "noDerivativesTrust200902",
            Self::Pre5378Trust200902 => "pre5378Trust200902",
        }
    }
}

/// Document category (intended status).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Category {
    StandardsTrack,
    Informational,
    Experimental,
    BestCurrentPractice,
    Historic,
}

impl Category {
    /// Returns the XML attribute value.
    #[must_use]
    pub fn xml_value(&self) -> &'static str {
        match self {
            Self::StandardsTrack => "std",
            Self::Informational => "info",
            Self::Experimental => "exp",
            Self::BestCurrentPractice => "bcp",
            Self::Historic => "historic",
        }
    }
}

/// Author information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Author {
    pub fullname: String,
    pub initials: Option<String>,
    pub surname: String,
    pub organization: Option<String>,
    pub email: Option<String>,
    pub role: Option<String>,
}

/// Reference to another RFC or Internet-Draft.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reference {
    pub anchor: String,
    pub title: String,
    pub target: Option<String>,
    pub series_info: Option<SeriesInfo>,
    pub authors: Vec<Author>,
    pub date: Option<NaiveDate>,
}

/// Series information for a reference (e.g., RFC 1234, draft-foo-bar-00).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeriesInfo {
    pub name: String,
    pub value: String,
}

/// IANA considerations section.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IanaConsiderations {
    pub registries: Vec<IanaRegistry>,
    pub has_actions: bool,
}

/// A single IANA registry action.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IanaRegistry {
    pub name: String,
    pub action: IanaAction,
}

/// Type of IANA registry action.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IanaAction {
    CreateNew,
    AddEntries,
    UpdateExisting,
    NoAction,
}

/// The core document model representing any Internet Society document.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    /// Document name (e.g., "draft-jewell-http-430-consent-required-00")
    pub name: String,

    /// Document title
    pub title: String,

    /// Document format
    pub format: DocumentFormat,

    /// Submission stream
    pub stream: Stream,

    /// Document category / intended status
    pub category: Option<Category>,

    /// Version number (draft revision, e.g., 0, 1, 2...)
    pub version: u32,

    /// IPR declaration
    pub ipr: Option<IprDeclaration>,

    /// Authors
    pub authors: Vec<Author>,

    /// Abstract text
    pub abstract_text: Option<String>,

    /// Document date
    pub date: Option<NaiveDate>,

    /// Expiry date (drafts expire 185 days after submission)
    pub expires: Option<NaiveDate>,

    /// Normative references
    pub normative_references: Vec<Reference>,

    /// Informative references
    pub informative_references: Vec<Reference>,

    /// IANA considerations
    pub iana_considerations: Option<IanaConsiderations>,

    /// Whether the document contains the required boilerplate
    pub has_boilerplate: bool,

    /// Raw source content
    pub source: String,

    /// Submission history
    pub submission_history: Vec<SubmissionEvent>,

    /// RFCs this document obsoletes
    pub obsoletes: Vec<u32>,

    /// RFCs this document updates
    pub updates: Vec<u32>,
}

/// A submission lifecycle event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmissionEvent {
    pub timestamp: DateTime<Utc>,
    pub event_type: SubmissionEventType,
    pub description: String,
}

/// Types of submission events.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SubmissionEventType {
    Created,
    Checked,
    Fixed,
    Submitted,
    Accepted,
    Rejected,
    Published,
    Expired,
    Withdrawn,
    StateChanged,
}

impl Document {
    /// Create a new empty document with the given name and stream.
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

    /// Parse draft name components: "draft-{source}-{name}-{version}"
    #[must_use]
    pub fn parse_draft_name(name: &str) -> Option<DraftNameParts> {
        if !name.starts_with("draft-") {
            return None;
        }
        let rest = &name[6..];
        let parts: Vec<&str> = rest.rsplitn(2, '-').collect();
        if parts.len() != 2 {
            return None;
        }
        let version_str = parts[0];
        let source_and_name = parts[1];

        let version = version_str.parse::<u32>().ok()?;

        Some(DraftNameParts {
            full_name: name.to_string(),
            source_and_name: source_and_name.to_string(),
            version,
        })
    }
}

/// Parsed components of a draft name.
#[derive(Debug, Clone)]
pub struct DraftNameParts {
    pub full_name: String,
    pub source_and_name: String,
    pub version: u32,
}
