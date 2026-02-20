// SPDX-License-Identifier: PMPL-1.0-or-later

//! Policy validation against Nickel-defined submission rules.

use crate::{NickelError, NickelWorkspace};
use intsoc_core::document::Document;
use intsoc_core::stream::Stream;

/// Check a document against the stream-specific submission policy.
pub fn check_policy(
    workspace: &NickelWorkspace,
    document: &Document,
) -> Result<PolicyResult, NickelError> {
    let policy_file = workspace.policies_dir().join("stream-rules.ncl");
    if !policy_file.exists() {
        return Err(NickelError::PolicyFailed(
            "stream-rules.ncl not found".to_string(),
        ));
    }

    // Build policy check context
    let _context = build_policy_context(document);

    // For now, perform basic structural checks inline
    // Full Nickel evaluation deferred to when nickel CLI is available
    let mut violations = Vec::new();

    // Check stream-specific requirements
    match &document.stream {
        Stream::IetfWorkingGroup { wg } if wg.is_empty() => {
            violations.push("Working group abbreviation cannot be empty".to_string());
        }
        Stream::IrtfResearchGroup { rg } if rg.is_empty() => {
            violations.push("Research group abbreviation cannot be empty".to_string());
        }
        _ => {}
    }

    // Check required fields
    if document.title.is_empty() {
        violations.push("Document title is required".to_string());
    }
    if document.authors.is_empty() {
        violations.push("At least one author is required".to_string());
    }
    if document.abstract_text.is_none() {
        violations.push("Abstract is required".to_string());
    }

    Ok(PolicyResult {
        passed: violations.is_empty(),
        violations,
    })
}

/// Result of a policy check.
#[derive(Debug, Clone)]
pub struct PolicyResult {
    pub passed: bool,
    pub violations: Vec<String>,
}

fn build_policy_context(document: &Document) -> serde_json::Value {
    serde_json::json!({
        "name": document.name,
        "title": document.title,
        "stream": document.stream,
        "category": document.category,
        "authors_count": document.authors.len(),
        "has_abstract": document.abstract_text.is_some(),
        "has_boilerplate": document.has_boilerplate,
        "format": document.format,
    })
}
