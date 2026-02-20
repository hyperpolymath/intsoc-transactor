// SPDX-License-Identifier: PMPL-1.0-or-later

//! The `check` command: validate a document for issues.

use intsoc_core::validation::{CheckCategory, CheckSummary, Fixability, Severity};
use intsoc_parser;
use std::path::Path;

/// Run the check command.
pub async fn run(file: &Path, errors_only: bool, format: &str) -> Result<(), Box<dyn std::error::Error>> {
    let source = std::fs::read_to_string(file)?;
    let document = intsoc_parser::parse(&source)?;

    tracing::info!("Checking: {} ({})", document.name, document.stream);

    // Run all validators
    let results = run_checks_internal(&document);
    let summary = CheckSummary::from_results(results);

    match format {
        "json" => {
            println!("{}", serde_json::to_string_pretty(&summary)?);
        }
        _ => {
            print_summary(&summary, errors_only);
        }
    }

    if summary.passes() {
        tracing::info!("All checks passed");
        Ok(())
    } else {
        Err(format!("{} error(s) found", summary.error_count).into())
    }
}

pub(crate) fn run_checks_internal(document: &intsoc_core::document::Document) -> Vec<intsoc_core::validation::CheckResult> {
    let mut results = Vec::new();

    // Check boilerplate
    if !document.has_boilerplate {
        results.push(intsoc_core::validation::CheckResult {
            check_id: "boilerplate-missing".to_string(),
            severity: Severity::Error,
            message: "Required boilerplate text is missing".to_string(),
            location: None,
            category: CheckCategory::Boilerplate,
            fixable: Fixability::AutoSafe,
            suggestion: Some("Add IETF Trust Legal Provisions boilerplate".to_string()),
        });
    }

    // Check title
    if document.title.is_empty() {
        results.push(intsoc_core::validation::CheckResult {
            check_id: "header-no-title".to_string(),
            severity: Severity::Error,
            message: "Document has no title".to_string(),
            location: None,
            category: CheckCategory::Header,
            fixable: Fixability::ManualOnly,
            suggestion: Some("Add a <title> element to the <front> section".to_string()),
        });
    }

    // Check authors
    if document.authors.is_empty() {
        results.push(intsoc_core::validation::CheckResult {
            check_id: "header-no-authors".to_string(),
            severity: Severity::Error,
            message: "Document has no authors".to_string(),
            location: None,
            category: CheckCategory::Header,
            fixable: Fixability::ManualOnly,
            suggestion: Some("Add at least one <author> element".to_string()),
        });
    }

    // Check abstract
    if document.abstract_text.is_none() {
        results.push(intsoc_core::validation::CheckResult {
            check_id: "sections-no-abstract".to_string(),
            severity: Severity::Warning,
            message: "Document has no abstract".to_string(),
            location: None,
            category: CheckCategory::Sections,
            fixable: Fixability::ManualOnly,
            suggestion: Some("Add an <abstract> element".to_string()),
        });
    }

    // Check date
    if document.date.is_none() {
        results.push(intsoc_core::validation::CheckResult {
            check_id: "date-missing".to_string(),
            severity: Severity::Warning,
            message: "Document has no date".to_string(),
            location: None,
            category: CheckCategory::Date,
            fixable: Fixability::AutoSafe,
            suggestion: Some("Add a <date> element with current date".to_string()),
        });
    }

    // Check draft name
    if document.name.is_empty() {
        results.push(intsoc_core::validation::CheckResult {
            check_id: "draft-name-missing".to_string(),
            severity: Severity::Error,
            message: "Document has no draft name (docName attribute)".to_string(),
            location: None,
            category: CheckCategory::DraftName,
            fixable: Fixability::ManualOnly,
            suggestion: Some("Add docName attribute to <rfc> element".to_string()),
        });
    } else if !document.name.starts_with("draft-") {
        results.push(intsoc_core::validation::CheckResult {
            check_id: "draft-name-format".to_string(),
            severity: Severity::Error,
            message: format!("Draft name '{}' does not start with 'draft-'", document.name),
            location: None,
            category: CheckCategory::DraftName,
            fixable: Fixability::Recommended,
            suggestion: Some("Draft names must begin with 'draft-'".to_string()),
        });
    }

    // Check IPR
    if document.ipr.is_none() {
        results.push(intsoc_core::validation::CheckResult {
            check_id: "ipr-missing".to_string(),
            severity: Severity::Error,
            message: "No IPR declaration found".to_string(),
            location: None,
            category: CheckCategory::Ipr,
            fixable: Fixability::AutoSafe,
            suggestion: Some("Add ipr=\"trust200902\" to <rfc> element".to_string()),
        });
    }

    results
}

fn print_summary(summary: &CheckSummary, errors_only: bool) {
    for result in &summary.results {
        if errors_only && result.severity < Severity::Error {
            continue;
        }

        let severity_label = match result.severity {
            Severity::Fatal => "FATAL",
            Severity::Error => "ERROR",
            Severity::Warning => "WARN ",
            Severity::Info => "INFO ",
        };

        let fixable_label = match result.fixable {
            Fixability::AutoSafe => " [auto-fix]",
            Fixability::Recommended => " [recommended fix]",
            Fixability::ManualOnly => " [manual]",
            Fixability::NotFixable => "",
        };

        println!(
            "  {severity_label} [{check_id}] {msg}{fix}",
            check_id = result.check_id,
            msg = result.message,
            fix = fixable_label
        );

        if let Some(ref suggestion) = result.suggestion {
            println!("         -> {suggestion}");
        }
    }

    println!();
    println!(
        "Summary: {} error(s), {} warning(s), {} info",
        summary.error_count, summary.warning_count, summary.info_count
    );
    println!(
        "Fixable: {} auto-safe, {} recommended, {} manual-only",
        summary.auto_fixable_count, summary.recommended_fixable_count, summary.manual_only_count
    );
}
