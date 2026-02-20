// SPDX-License-Identifier: PMPL-1.0-or-later

//! Draft name fix generator.

use intsoc_core::document::Document;
use intsoc_core::fix::{Fix, FixChange};
use intsoc_core::validation::{CheckCategory, CheckResult, Fixability};

use super::FixGenerator;

/// Generates fixes for draft naming convention issues.
pub struct DraftNameFixGenerator;

impl FixGenerator for DraftNameFixGenerator {
    fn category(&self) -> CheckCategory {
        CheckCategory::DraftName
    }

    fn generate(&self, document: &Document, results: &[CheckResult]) -> Vec<Fix> {
        results
            .iter()
            .filter(|r| r.category == CheckCategory::DraftName)
            .filter_map(|r| generate_draft_name_fix(document, r))
            .collect()
    }
}

fn generate_draft_name_fix(document: &Document, result: &CheckResult) -> Option<Fix> {
    let msg = result.message.to_lowercase();

    if msg.contains("version") || msg.contains("revision") {
        // Fix version number in draft name
        let expected_name = if let Some(parts) = Document::parse_draft_name(&document.name) {
            let base = &parts.source_and_name;
            format!("draft-{base}-{:02}", document.version)
        } else {
            return None;
        };

        Some(Fix {
            id: format!("fix-{}", result.check_id),
            check_id: result.check_id.clone(),
            description: format!("Update draft name to '{expected_name}'"),
            fixability: Fixability::AutoSafe,
            category: CheckCategory::DraftName,
            change: FixChange::XmlReplace {
                path: "/rfc/@docName".to_string(),
                old_value: document.name.clone(),
                new_value: expected_name,
            },
        })
    } else {
        None
    }
}
