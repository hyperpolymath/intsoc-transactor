// SPDX-License-Identifier: PMPL-1.0-or-later

//! Validation framework for document checking.

use serde::{Deserialize, Serialize};

/// Severity of a check result.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Severity {
    /// Informational note, not a problem
    Info,
    /// Warning that should be addressed
    Warning,
    /// Error that must be fixed before submission
    Error,
    /// Fatal error that prevents further processing
    Fatal,
}

/// A single check result from document validation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckResult {
    /// Unique check identifier (e.g., "boilerplate-missing")
    pub check_id: String,

    /// Severity of the finding
    pub severity: Severity,

    /// Human-readable description
    pub message: String,

    /// Location in the document (line number, XML path, etc.)
    pub location: Option<Location>,

    /// The category of check that produced this result
    pub category: CheckCategory,

    /// Whether an automatic fix is available
    pub fixable: Fixability,

    /// Suggested fix description (if any)
    pub suggestion: Option<String>,
}

/// Location within a document.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Location {
    /// Line number in the source
    Line(u32),
    /// Line and column
    LineColumn { line: u32, column: u32 },
    /// XML path (e.g., "/rfc/front/title")
    XmlPath(String),
    /// Section number (e.g., "3.2.1")
    Section(String),
}

/// Categories of checks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CheckCategory {
    /// Boilerplate text validation
    Boilerplate,
    /// Date and expiry checks
    Date,
    /// Header/metadata validation
    Header,
    /// Reference checks (normative/informative)
    References,
    /// Required section validation
    Sections,
    /// Text formatting issues
    TextFormat,
    /// XML structure/syntax
    Xml,
    /// IANA considerations
    IanaSections,
    /// Draft naming conventions
    DraftName,
    /// IPR declaration
    Ipr,
}

/// Whether and how a check result can be fixed automatically.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Fixability {
    /// Can be fixed automatically with no risk
    AutoSafe,
    /// Can be fixed automatically but should be reviewed
    Recommended,
    /// Requires manual intervention
    ManualOnly,
    /// Not fixable (informational only)
    NotFixable,
}

/// Summary of all check results for a document.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckSummary {
    pub results: Vec<CheckResult>,
    pub error_count: usize,
    pub warning_count: usize,
    pub info_count: usize,
    pub auto_fixable_count: usize,
    pub recommended_fixable_count: usize,
    pub manual_only_count: usize,
}

impl CheckSummary {
    /// Create a summary from a list of check results.
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

    /// Whether the document passes all checks (no errors or fatals).
    #[must_use]
    pub fn passes(&self) -> bool {
        self.error_count == 0
    }
}
