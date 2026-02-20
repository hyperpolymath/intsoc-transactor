// SPDX-License-Identifier: PMPL-1.0-or-later

//! Tauri 2.0 entry point for the intsoc-transactor desktop GUI.
//!
//! Registers Tauri commands that bridge the ReScript frontend to the
//! Rust domain crates: intsoc-core, intsoc-parser, and intsoc-fixer.

use serde::{Deserialize, Serialize};

use intsoc_core::validation::{CheckCategory, CheckResult, CheckSummary, Fixability, Severity};
use intsoc_fixer::engine::FixEngine;

// ---------------------------------------------------------------------------
// Serializable response types (JSON-compatible for the frontend)
// ---------------------------------------------------------------------------

/// Check result serialized for the frontend.
#[derive(Debug, Clone, Serialize)]
struct FrontendCheckResult {
    check_id: String,
    severity: String,
    message: String,
    location: Option<String>,
    category: String,
    fixable: String,
    suggestion: Option<String>,
}

/// Check summary serialized for the frontend.
#[derive(Debug, Clone, Serialize)]
struct FrontendCheckSummary {
    results: Vec<FrontendCheckResult>,
    error_count: usize,
    warning_count: usize,
    info_count: usize,
    auto_fixable_count: usize,
    recommended_fixable_count: usize,
    manual_only_count: usize,
}

/// Fix result serialized for the frontend.
#[derive(Debug, Clone, Serialize)]
struct FrontendFixResult {
    success: bool,
    fixed_source: String,
    diff_preview: String,
    auto_safe_applied: usize,
    recommended_applied: usize,
    manual_remaining: usize,
}

/// Submission status serialized for the frontend.
#[derive(Debug, Clone, Serialize)]
struct FrontendSubmissionStatus {
    document_name: String,
    stream: String,
    state: String,
    submitted: bool,
    datatracker_url: Option<String>,
    message: String,
}

/// Parameters for the check_document command.
#[derive(Debug, Deserialize)]
struct CheckDocumentParams {
    source: String,
    stream_hint: Option<String>,
}

/// Parameters for the fix_document command.
#[derive(Debug, Deserialize)]
struct FixDocumentParams {
    source: String,
    auto_only: bool,
    dry_run: bool,
}

/// Parameters for the get_submission_status command.
#[derive(Debug, Deserialize)]
struct GetStatusParams {
    document_name: String,
}

// ---------------------------------------------------------------------------
// Conversion helpers
// ---------------------------------------------------------------------------

fn severity_to_string(s: &Severity) -> String {
    match s {
        Severity::Info => "Info".to_string(),
        Severity::Warning => "Warning".to_string(),
        Severity::Error => "Error".to_string(),
        Severity::Fatal => "Fatal".to_string(),
    }
}

fn category_to_string(c: &CheckCategory) -> String {
    match c {
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

fn fixability_to_string(f: &Fixability) -> String {
    match f {
        Fixability::AutoSafe => "AutoSafe".to_string(),
        Fixability::Recommended => "Recommended".to_string(),
        Fixability::ManualOnly => "ManualOnly".to_string(),
        Fixability::NotFixable => "NotFixable".to_string(),
    }
}

fn location_to_string(loc: &Option<intsoc_core::validation::Location>) -> Option<String> {
    loc.as_ref().map(|l| match l {
        intsoc_core::validation::Location::Line(n) => format!("line {n}"),
        intsoc_core::validation::Location::LineColumn { line, column } => {
            format!("line {line}, col {column}")
        }
        intsoc_core::validation::Location::XmlPath(path) => path.clone(),
        intsoc_core::validation::Location::Section(s) => format!("section {s}"),
    })
}

fn convert_check_result(r: &CheckResult) -> FrontendCheckResult {
    FrontendCheckResult {
        check_id: r.check_id.clone(),
        severity: severity_to_string(&r.severity),
        message: r.message.clone(),
        location: location_to_string(&r.location),
        category: category_to_string(&r.category),
        fixable: fixability_to_string(&r.fixable),
        suggestion: r.suggestion.clone(),
    }
}

fn convert_check_summary(summary: &CheckSummary) -> FrontendCheckSummary {
    FrontendCheckSummary {
        results: summary.results.iter().map(convert_check_result).collect(),
        error_count: summary.error_count,
        warning_count: summary.warning_count,
        info_count: summary.info_count,
        auto_fixable_count: summary.auto_fixable_count,
        recommended_fixable_count: summary.recommended_fixable_count,
        manual_only_count: summary.manual_only_count,
    }
}

// ---------------------------------------------------------------------------
// Tauri commands
// ---------------------------------------------------------------------------

/// Check a document for issues.
///
/// Parses the source, runs all validators, and returns a structured summary
/// of errors, warnings, and fix suggestions.
#[tauri::command]
fn check_document(params: CheckDocumentParams) -> Result<FrontendCheckSummary, String> {
    let _stream_hint = params.stream_hint;

    let document = intsoc_parser::parse(&params.source).map_err(|e| format!("Parse error: {e}"))?;

    // Run checks (replicates CLI check logic)
    let mut results = Vec::new();

    if !document.has_boilerplate {
        results.push(CheckResult {
            check_id: "boilerplate-missing".to_string(),
            severity: Severity::Error,
            message: "Required boilerplate text is missing".to_string(),
            location: None,
            category: CheckCategory::Boilerplate,
            fixable: Fixability::AutoSafe,
            suggestion: Some("Add IETF Trust Legal Provisions boilerplate".to_string()),
        });
    }

    if document.title.is_empty() {
        results.push(CheckResult {
            check_id: "header-no-title".to_string(),
            severity: Severity::Error,
            message: "Document has no title".to_string(),
            location: None,
            category: CheckCategory::Header,
            fixable: Fixability::ManualOnly,
            suggestion: Some("Add a <title> element to the <front> section".to_string()),
        });
    }

    if document.authors.is_empty() {
        results.push(CheckResult {
            check_id: "header-no-authors".to_string(),
            severity: Severity::Error,
            message: "Document has no authors".to_string(),
            location: None,
            category: CheckCategory::Header,
            fixable: Fixability::ManualOnly,
            suggestion: Some("Add at least one <author> element".to_string()),
        });
    }

    if document.abstract_text.is_none() {
        results.push(CheckResult {
            check_id: "sections-no-abstract".to_string(),
            severity: Severity::Warning,
            message: "Document has no abstract".to_string(),
            location: None,
            category: CheckCategory::Sections,
            fixable: Fixability::ManualOnly,
            suggestion: Some("Add an <abstract> element".to_string()),
        });
    }

    if document.date.is_none() {
        results.push(CheckResult {
            check_id: "date-missing".to_string(),
            severity: Severity::Warning,
            message: "Document has no date".to_string(),
            location: None,
            category: CheckCategory::Date,
            fixable: Fixability::AutoSafe,
            suggestion: Some("Add a <date> element with current date".to_string()),
        });
    }

    if document.name.is_empty() {
        results.push(CheckResult {
            check_id: "draft-name-missing".to_string(),
            severity: Severity::Error,
            message: "Document has no draft name (docName attribute)".to_string(),
            location: None,
            category: CheckCategory::DraftName,
            fixable: Fixability::ManualOnly,
            suggestion: Some("Add docName attribute to <rfc> element".to_string()),
        });
    } else if !document.name.starts_with("draft-") {
        results.push(CheckResult {
            check_id: "draft-name-format".to_string(),
            severity: Severity::Error,
            message: format!("Draft name '{}' does not start with 'draft-'", document.name),
            location: None,
            category: CheckCategory::DraftName,
            fixable: Fixability::Recommended,
            suggestion: Some("Draft names must begin with 'draft-'".to_string()),
        });
    }

    if document.ipr.is_none() {
        results.push(CheckResult {
            check_id: "ipr-missing".to_string(),
            severity: Severity::Error,
            message: "No IPR declaration found".to_string(),
            location: None,
            category: CheckCategory::Ipr,
            fixable: Fixability::AutoSafe,
            suggestion: Some("Add ipr=\"trust200902\" to <rfc> element".to_string()),
        });
    }

    let summary = CheckSummary::from_results(results);
    Ok(convert_check_summary(&summary))
}

/// Fix a document by generating and applying fixes.
///
/// Uses the intsoc-fixer engine to generate a fix plan, optionally apply
/// auto-safe or all fixes, and return the result with a diff preview.
#[tauri::command]
fn fix_document(params: FixDocumentParams) -> Result<FrontendFixResult, String> {
    let document =
        intsoc_parser::parse(&params.source).map_err(|e| format!("Parse error: {e}"))?;

    // Run checks first to identify issues
    let mut results = Vec::new();

    if !document.has_boilerplate {
        results.push(CheckResult {
            check_id: "boilerplate-missing".to_string(),
            severity: Severity::Error,
            message: "Required boilerplate text is missing".to_string(),
            location: None,
            category: CheckCategory::Boilerplate,
            fixable: Fixability::AutoSafe,
            suggestion: Some("Add IETF Trust Legal Provisions boilerplate".to_string()),
        });
    }

    if document.date.is_none() {
        results.push(CheckResult {
            check_id: "date-missing".to_string(),
            severity: Severity::Warning,
            message: "Document has no date".to_string(),
            location: None,
            category: CheckCategory::Date,
            fixable: Fixability::AutoSafe,
            suggestion: Some("Add a <date> element with current date".to_string()),
        });
    }

    if document.ipr.is_none() {
        results.push(CheckResult {
            check_id: "ipr-missing".to_string(),
            severity: Severity::Error,
            message: "No IPR declaration found".to_string(),
            location: None,
            category: CheckCategory::Ipr,
            fixable: Fixability::AutoSafe,
            suggestion: Some("Add ipr=\"trust200902\" to <rfc> element".to_string()),
        });
    }

    // Generate fix plan
    let engine = FixEngine::new();
    let plan = engine.plan(&document, &results);

    let auto_safe_count = plan.auto_safe_fixes().len();
    let recommended_count = plan.recommended_fixes().len();
    let manual_count = plan.manual_only_fixes().len();

    // Apply fixes based on options
    let fixed_source = if params.auto_only {
        engine
            .apply_auto_safe(&params.source, &plan)
            .map_err(|e| format!("Fix error: {e}"))?
    } else {
        let all_fixes: Vec<&intsoc_core::fix::Fix> = plan.fixes.iter().collect();
        engine
            .apply_fixes(&params.source, &all_fixes)
            .map_err(|e| format!("Fix error: {e}"))?
    };

    // Generate diff preview
    let diff_preview = if params.dry_run {
        engine.preview(&params.source, &fixed_source)
    } else {
        String::new()
    };

    Ok(FrontendFixResult {
        success: true,
        fixed_source: if params.dry_run {
            String::new()
        } else {
            fixed_source
        },
        diff_preview,
        auto_safe_applied: auto_safe_count,
        recommended_applied: if params.auto_only { 0 } else { recommended_count },
        manual_remaining: manual_count,
    })
}

/// Get the submission status for a document.
///
/// Queries the IETF Datatracker API (or provides guidance for other streams)
/// to report the current lifecycle state.
#[tauri::command]
fn get_submission_status(params: GetStatusParams) -> Result<FrontendSubmissionStatus, String> {
    let doc_name = &params.document_name;

    // Determine if this is a valid draft name
    if !doc_name.starts_with("draft-") {
        return Ok(FrontendSubmissionStatus {
            document_name: doc_name.clone(),
            stream: "Unknown".to_string(),
            state: "Not a draft".to_string(),
            submitted: false,
            datatracker_url: None,
            message: "Document name does not start with 'draft-'. Cannot query Datatracker."
                .to_string(),
        });
    }

    // For now, return a stub response pointing to the Datatracker.
    // Full API integration will query https://datatracker.ietf.org/api/v1/doc/document/{name}/
    let datatracker_url = format!("https://datatracker.ietf.org/doc/{doc_name}/");

    Ok(FrontendSubmissionStatus {
        document_name: doc_name.clone(),
        stream: "IETF".to_string(),
        state: "Query pending".to_string(),
        submitted: false,
        datatracker_url: Some(datatracker_url),
        message: format!(
            "Automated status query not yet implemented. \
             Check the Datatracker manually for '{doc_name}'."
        ),
    })
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

fn main() {
    tracing_subscriber::fmt::init();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            check_document,
            fix_document,
            get_submission_status,
        ])
        .run(tauri::generate_context!())
        .expect("error running intsoc-transactor GUI");
}
