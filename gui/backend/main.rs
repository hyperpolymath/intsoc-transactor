// SPDX-License-Identifier: PMPL-1.0-or-later
// Copyright (c) 2026 Jonathan D.A. Jewell (hyperpolymath) <j.d.a.jewell@open.ac.uk>

//! Gossamer Main Entry — Desktop GUI Orchestrator.
//!
//! This module implements the Gossamer backend for the transactor GUI.
//! It defines the IPC command handlers that allow the ReScript/React frontend
//! to invoke high-assurance logic implemented in the Rust workspace crates.
//!
//! DESIGN PILLARS:
//! 1. **Serialization**: Implements `Frontend*` adapter types to ensure
//!    JSON-compatible data exchange with the WebView.
//! 2. **Safety**: All commands are executed within the Gossamer capability sandbox.
//! 3. **Conversion**: Provides loss-less mapping between internal
//!    Rust enums and frontend-friendly string identifiers.
//!
//! MIGRATION NOTE:
//! Converted from Tauri 2.0 to Gossamer. The `#[tauri::command]` functions
//! are now registered via `app.command()` with the standard Gossamer handler
//! signature: `Fn(serde_json::Value) -> Result<serde_json::Value, String>`.

#![forbid(unsafe_code)]

use gossamer_rs::App;
use intsoc_core::validation::{
    CheckCategory, CheckResult, CheckSummary, Fixability, Location, Severity,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

// =============================================================================
// Frontend adapter types — JSON-serializable versions of domain types
// =============================================================================

/// FRONTEND ADAPTER: A JSON-serializable version of the domain check result.
///
/// Maps internal enum variants (Severity, CheckCategory, Fixability) to
/// string identifiers that the ReScript frontend can pattern-match on.
#[derive(Debug, Clone, Serialize)]
struct FrontendCheckResult {
    /// Unique identifier for the check rule (e.g., "boilerplate-001").
    check_id: String,
    /// Severity level as a string: "Info", "Warning", "Error", or "Fatal".
    severity: String,
    /// Human-readable description of the finding.
    message: String,
    /// Optional source location (line:col or section reference).
    location: Option<String>,
    /// Check category as a string: "Boilerplate", "Date", "Header", etc.
    category: String,
    /// Fixability classification: "AutoSafe", "Recommended", "ManualOnly", "NotFixable".
    fixable: String,
    /// Optional suggestion for how to fix the issue.
    suggestion: Option<String>,
}

/// FRONTEND ADAPTER: Aggregated check summary with counts by severity/fixability.
#[derive(Debug, Clone, Serialize)]
struct FrontendCheckSummary {
    /// All individual check results.
    results: Vec<FrontendCheckResult>,
    /// Count of results with Error or Fatal severity.
    error_count: usize,
    /// Count of results with Warning severity.
    warning_count: usize,
    /// Count of results with Info severity.
    info_count: usize,
    /// Count of results that can be auto-fixed safely.
    auto_fixable_count: usize,
    /// Count of results with recommended (but not guaranteed safe) fixes.
    recommended_fixable_count: usize,
    /// Count of results requiring manual intervention.
    manual_only_count: usize,
}

/// FRONTEND ADAPTER: Result of applying fixes to a document.
#[derive(Debug, Clone, Serialize)]
struct FrontendFixResult {
    /// Whether the fix operation completed without errors.
    success: bool,
    /// The full document source after fixes have been applied.
    fixed_source: String,
    /// Unified diff showing what was changed.
    diff_preview: String,
    /// Number of auto-safe fixes applied.
    auto_safe_applied: usize,
    /// Number of recommended fixes applied.
    recommended_applied: usize,
    /// Number of issues remaining that require manual intervention.
    manual_remaining: usize,
}

/// FRONTEND ADAPTER: Datatracker submission status for a document.
#[derive(Debug, Clone, Serialize)]
struct FrontendSubmissionStatus {
    /// The canonical document name (e.g., "draft-ietf-example-protocol-07").
    document_name: String,
    /// The IETF stream (e.g., "IETF", "IAB", "IRTF", "ISE").
    stream: String,
    /// Current submission state.
    state: String,
    /// Whether the document has been submitted to the Datatracker.
    submitted: bool,
    /// URL to the Datatracker page, if available.
    datatracker_url: Option<String>,
    /// Human-readable status message.
    message: String,
}

// =============================================================================
// Conversion helpers — map domain enums to frontend strings
// =============================================================================

/// Convert a Severity enum variant to its frontend string representation.
fn severity_to_string(severity: &Severity) -> String {
    match severity {
        Severity::Info => "Info".to_string(),
        Severity::Warning => "Warning".to_string(),
        Severity::Error => "Error".to_string(),
        Severity::Fatal => "Fatal".to_string(),
    }
}

/// Convert a CheckCategory enum variant to its frontend string representation.
fn category_to_string(category: &CheckCategory) -> String {
    match category {
        CheckCategory::Boilerplate => "Boilerplate".to_string(),
        CheckCategory::Date => "Date".to_string(),
        CheckCategory::Header => "Header".to_string(),
        CheckCategory::References => "References".to_string(),
        CheckCategory::Sections => "Sections".to_string(),
        CheckCategory::TextFormat => "TextFormat".to_string(),
        CheckCategory::Xml => "Xml".to_string(),
        CheckCategory::IanaSections => "IanaSections".to_string(),
        CheckCategory::DraftName => "DraftName".to_string(),
        CheckCategory::Ipr => "Ipr".to_string(),
    }
}

/// Convert a Fixability enum variant to its frontend string representation.
fn fixability_to_string(fixability: &Fixability) -> String {
    match fixability {
        Fixability::AutoSafe => "AutoSafe".to_string(),
        Fixability::Recommended => "Recommended".to_string(),
        Fixability::ManualOnly => "ManualOnly".to_string(),
        Fixability::NotFixable => "NotFixable".to_string(),
    }
}

/// Convert a domain Location to a frontend-friendly string.
fn location_to_string(location: &Location) -> String {
    match location {
        Location::Line(line) => format!("line {line}"),
        Location::LineColumn { line, column } => format!("line {line}, col {column}"),
        Location::XmlPath(path) => path.clone(),
        Location::Section(section) => section.clone(),
    }
}

/// Convert a domain CheckResult to a FrontendCheckResult for JSON serialization.
fn to_frontend_result(result: &CheckResult) -> FrontendCheckResult {
    FrontendCheckResult {
        check_id: result.check_id.clone(),
        severity: severity_to_string(&result.severity),
        message: result.message.clone(),
        location: result.location.as_ref().map(location_to_string),
        category: category_to_string(&result.category),
        fixable: fixability_to_string(&result.fixable),
        suggestion: result.suggestion.clone(),
    }
}

// =============================================================================
// Command handlers — business logic invoked by the Gossamer IPC bridge
// =============================================================================

/// GOSSAMER COMMAND: `check_document`.
///
/// Triggers a full audit of the provided source text. The handler:
///   1. PARSE: Transforms raw source into the domain model via intsoc-parser.
///   2. AUDIT: Executes the internal check kernel from intsoc-core.
///   3. MAP:   Converts results to FrontendCheckResult for the WebView.
///
/// Payload fields:
///   - `source` (string): The raw document text to validate.
///   - `stream_hint` (string|null): Optional IETF stream hint.
///
/// Returns: A FrontendCheckSummary as JSON.
fn handle_check_document(payload: Value) -> Result<Value, String> {
    let source = payload["source"]
        .as_str()
        .ok_or_else(|| "missing 'source' field".to_string())?;

    let stream_hint = payload["stream_hint"].as_str().map(String::from);

    // Parse and audit the document using the workspace crates
    let parsed = intsoc_parser::parse_document(source).map_err(|e| format!("parse error: {e}"))?;

    let check_summary = intsoc_core::validation::check_document(&parsed, stream_hint.as_deref())
        .map_err(|e| format!("check error: {e}"))?;

    // Convert to frontend-friendly types
    let frontend_results: Vec<FrontendCheckResult> = check_summary
        .results
        .iter()
        .map(to_frontend_result)
        .collect();

    let summary = FrontendCheckSummary {
        results: frontend_results,
        error_count: check_summary.error_count,
        warning_count: check_summary.warning_count,
        info_count: check_summary.info_count,
        auto_fixable_count: check_summary.auto_fixable_count,
        recommended_fixable_count: check_summary.recommended_fixable_count,
        manual_only_count: check_summary.manual_only_count,
    };

    serde_json::to_value(summary).map_err(|e| format!("serialization error: {e}"))
}

/// GOSSAMER COMMAND: `fix_document`.
///
/// Applies automated fixes to the provided source text. The handler:
///   1. PARSE: Transforms raw source into the domain model.
///   2. FIX:   Runs the fixer engine from intsoc-fixer.
///   3. DIFF:  Generates a unified diff preview of changes.
///
/// Payload fields:
///   - `source` (string): The raw document text to fix.
///   - `auto_only` (bool): If true, only apply AutoSafe fixes.
///   - `dry_run` (bool): If true, generate diff but do not apply.
///
/// Returns: A FrontendFixResult as JSON.
fn handle_fix_document(payload: Value) -> Result<Value, String> {
    let source = payload["source"]
        .as_str()
        .ok_or_else(|| "missing 'source' field".to_string())?;

    let auto_only = payload["auto_only"].as_bool().unwrap_or(true);
    let dry_run = payload["dry_run"].as_bool().unwrap_or(false);

    // Parse and fix the document using the workspace crates
    let parsed = intsoc_parser::parse_document(source).map_err(|e| format!("parse error: {e}"))?;

    let fix_result = intsoc_fixer::fix_document(&parsed, auto_only, dry_run)
        .map_err(|e| format!("fix error: {e}"))?;

    let frontend_result = FrontendFixResult {
        success: fix_result.success,
        fixed_source: fix_result.fixed_source,
        diff_preview: fix_result.diff_preview,
        auto_safe_applied: fix_result.auto_safe_applied,
        recommended_applied: fix_result.recommended_applied,
        manual_remaining: fix_result.manual_remaining,
    };

    serde_json::to_value(frontend_result).map_err(|e| format!("serialization error: {e}"))
}

/// GOSSAMER COMMAND: `get_submission_status`.
///
/// Queries the IETF Datatracker (or a mock endpoint) for the submission
/// state of a named document.
///
/// Payload fields:
///   - `document_name` (string): The canonical document name to query.
///
/// Returns: A FrontendSubmissionStatus as JSON.
fn handle_get_submission_status(payload: Value) -> Result<Value, String> {
    let document_name = payload["document_name"]
        .as_str()
        .ok_or_else(|| "missing 'document_name' field".to_string())?;

    // Query submission status via intsoc-core (may use intsoc-api internally)
    let status = intsoc_core::submission::get_status(document_name)
        .map_err(|e| format!("status query error: {e}"))?;

    let frontend_status = FrontendSubmissionStatus {
        document_name: status.document_name,
        stream: status.stream,
        state: status.state,
        submitted: status.submitted,
        datatracker_url: status.datatracker_url,
        message: status.message,
    };

    serde_json::to_value(frontend_status).map_err(|e| format!("serialization error: {e}"))
}

// =============================================================================
// Main — boots the Gossamer application and registers all command handlers
// =============================================================================

fn main() -> Result<(), gossamer_rs::Error> {
    // Initialise structured logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    tracing::info!(
        "intsoc-transactor GUI starting (Gossamer {})",
        gossamer_rs::version()
    );

    // Create the Gossamer application window
    let mut app = App::with_config(gossamer_rs::WindowConfig {
        title: "intsoc-transactor - Internet Society Document Processing".to_string(),
        width: 1200,
        height: 800,
        resizable: true,
        decorations: true,
        fullscreen: false,
    })?;

    // -------------------------------------------------------------------------
    // Register IPC command handlers
    // -------------------------------------------------------------------------

    // check_document: validate an Internet-Draft against IETF/IANA standards
    app.command("check_document", handle_check_document);

    // fix_document: apply automated fixes to an Internet-Draft
    app.command("fix_document", handle_fix_document);

    // get_submission_status: query Datatracker submission state
    app.command("get_submission_status", handle_get_submission_status);

    // -------------------------------------------------------------------------
    // Load the frontend — serves the compiled ReScript/HTML from gui/
    // -------------------------------------------------------------------------

    // Navigate to the frontend dist (gossamer.conf.json controls the frontendDist path)
    app.navigate("gossamer://localhost/")?;

    tracing::info!("intsoc-transactor GUI ready — entering event loop");

    // Run the event loop (blocks until the window is closed)
    app.run();

    Ok(())
}
