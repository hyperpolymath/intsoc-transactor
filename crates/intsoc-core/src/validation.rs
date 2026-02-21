// SPDX-License-Identifier: PMPL-1.0-or-later

//! Validation framework for Internet Society document checking.
//!
//! This module provides the data structures for reporting validation findings
//! (errors, warnings, fixes) and categorizing them according to the type of 
//! check (Boilerplate, XML, IANA, etc.).

use serde::{Deserialize, Serialize};

/// Severity levels for validation findings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Severity {
    /// Non-critical note or reminder.
    Info,
    /// Potential issue that should be addressed but does not block submission.
    Warning,
    /// Compliance failure that MUST be fixed before submission.
    Error,
    /// Critical failure that prevents further validation or processing.
    Fatal,
}

/// A granular validation result for a specific check.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckResult {
    /// Unique identifier for the check (e.g., "iana-section-missing").
    pub check_id: String,

    /// Severity of this specific finding.
    pub severity: Severity,

    /// Human-readable explanation of the finding.
    pub message: String,

    /// Logical or physical location within the source document.
    pub location: Option<Location>,

    /// Categorization for grouping results in the UI.
    pub category: CheckCategory,

    /// Indicates if the transactor engine can automatically resolve this issue.
    pub fixable: Fixability,

    /// Optional instructions or text for a manual fix.
    pub suggestion: Option<String>,
}

/// Identifies a specific point within a document.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Location {
    /// Physical line number (1-based).
    Line(u32),
    /// Physical line and column.
    LineColumn { line: u32, column: u32 },
    /// Logical path in an XML document.
    XmlPath(String),
    /// logical section heading (e.g., "Security Considerations").
    Section(String),
}

/// Categories used to organize validation checks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CheckCategory {
    /// Mandatory legal and administrative text.
    Boilerplate,
    /// Expiration dates and timestamp validity.
    Date,
    /// Front-matter metadata (Authors, Title, Stream).
    Header,
    /// Cross-document reference validity and classification.
    References,
    /// Presence of required logical sections.
    Sections,
    /// Line length, indentation, and encoding.
    TextFormat,
    /// Formal XML schema validation (RFC 7991).
    Xml,
    /// Registry impact and parameter assignment text.
    IanaSections,
    /// Adherence to the draft- naming policy.
    DraftName,
    /// Intellectual Property Rights declaration compliance.
    Ipr,
}

/// Degree of automation possible for a specific fix.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Fixability {
    /// High-confidence automatic resolution (e.g., updating a date).
    AutoSafe,
    /// Automatic fix available but requires human review.
    Recommended,
    /// Issue is too complex for automation; requires manual edit.
    ManualOnly,
    /// Finding is purely informational; no fix required.
    NotFixable,
}

/// Aggregated report of all validation checks for a document.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckSummary {
    /// Individual check findings.
    pub results: Vec<CheckResult>,
    pub error_count: usize,
    pub warning_count: usize,
    pub info_count: usize,
    pub auto_fixable_count: usize,
    pub recommended_fixable_count: usize,
    pub manual_only_count: usize,
}

impl CheckSummary {
    /// Constructs a summary report from a list of raw results.
    #[must_use]
    pub fn from_results(results: Vec<CheckResult>) -> Self {
        let error_count = results.iter().filter(|r| r.severity >= Severity::Error).count();
        let warning_count = results
            .iter()
            .filter(|r| r.severity == Severity::Warning)
            .count();
        let info_count = results
            .iter()
            .filter(|r| r.severity == Severity::Info)
            .count();
        let auto_fixable_count = results
            .iter()
            .filter(|r| r.fixable == Fixability::AutoSafe)
            .count();
        let recommended_fixable_count = results
            .iter()
            .filter(|r| r.fixable == Fixability::Recommended)
            .count();
        let manual_only_count = results
            .iter()
            .filter(|r| r.fixable == Fixability::ManualOnly)
            .count();

        Self {
            results,
            error_count,
            warning_count,
            info_count,
            auto_fixable_count,
            recommended_fixable_count,
            manual_only_count,
        }
    }

    /// High-level pass/fail gate. Returns true if no Errors or Fatals exist.
    #[must_use]
    pub fn passes(&self) -> bool {
        self.error_count == 0
    }
}
