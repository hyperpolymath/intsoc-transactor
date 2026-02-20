// SPDX-License-Identifier: PMPL-1.0-or-later

//! Header/metadata fix generator.

use intsoc_core::document::Document;
use intsoc_core::fix::{Fix, FixChange};
use intsoc_core::validation::{CheckCategory, CheckResult, Fixability};

use super::FixGenerator;

/// Generates fixes for header and metadata issues.
pub struct HeaderFixGenerator;

impl FixGenerator for HeaderFixGenerator {
    fn category(&self) -> CheckCategory {
        CheckCategory::Header
    }

    fn generate(&self, document: &Document, results: &[CheckResult]) -> Vec<Fix> {
        results
            .iter()
            .filter(|r| r.category == CheckCategory::Header)
            .filter_map(|r| generate_header_fix(document, r))
            .collect()
    }
}

fn generate_header_fix(document: &Document, result: &CheckResult) -> Option<Fix> {
    let msg = result.message.to_lowercase();

    if msg.contains("category") || msg.contains("intended status") {
        let category = document
            .category
            .map_or("info", |c| c.xml_value());

        Some(Fix {
            id: format!("fix-{}", result.check_id),
            check_id: result.check_id.clone(),
            description: format!("Set document category to '{category}'"),
            fixability: Fixability::Recommended,
            category: CheckCategory::Header,
            change: FixChange::XmlReplace {
                path: "/rfc/@category".to_string(),
                old_value: String::new(),
                new_value: category.to_string(),
            },
        })
    } else if msg.contains("workgroup") || msg.contains("working group") {
        Some(Fix {
            id: format!("fix-{}", result.check_id),
            check_id: result.check_id.clone(),
            description: "Add workgroup element to document front matter".to_string(),
            fixability: Fixability::ManualOnly,
            category: CheckCategory::Header,
            change: FixChange::XmlInsert {
                parent_path: "/rfc/front".to_string(),
                position: intsoc_core::fix::XmlInsertPosition::After("title".to_string()),
                element: "<workgroup>TODO</workgroup>".to_string(),
            },
        })
    } else {
        None
    }
}
