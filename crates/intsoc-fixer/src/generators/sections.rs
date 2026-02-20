// SPDX-License-Identifier: PMPL-1.0-or-later

//! Required section fix generator.

use intsoc_core::document::Document;
use intsoc_core::fix::{Fix, FixChange, XmlInsertPosition};
use intsoc_core::validation::{CheckCategory, CheckResult, Fixability};

use super::FixGenerator;

/// Generates fixes for missing required sections.
pub struct SectionFixGenerator;

impl FixGenerator for SectionFixGenerator {
    fn category(&self) -> CheckCategory {
        CheckCategory::Sections
    }

    fn generate(&self, document: &Document, results: &[CheckResult]) -> Vec<Fix> {
        results
            .iter()
            .filter(|r| r.category == CheckCategory::Sections || r.category == CheckCategory::IanaSections)
            .filter_map(|r| generate_section_fix(document, r))
            .collect()
    }
}

fn generate_section_fix(_document: &Document, result: &CheckResult) -> Option<Fix> {
    let msg = result.message.to_lowercase();

    if msg.contains("security considerations") {
        Some(Fix {
            id: format!("fix-{}", result.check_id),
            check_id: result.check_id.clone(),
            description: "Add Security Considerations section".to_string(),
            fixability: Fixability::ManualOnly,
            category: CheckCategory::Sections,
            change: FixChange::XmlInsert {
                parent_path: "/rfc/middle".to_string(),
                position: XmlInsertPosition::Last,
                element: concat!(
                    "<section title=\"Security Considerations\">\n",
                    "  <t>TODO: Describe security considerations for this document.</t>\n",
                    "</section>"
                )
                .to_string(),
            },
        })
    } else if msg.contains("iana considerations") || msg.contains("iana") {
        Some(Fix {
            id: format!("fix-{}", result.check_id),
            check_id: result.check_id.clone(),
            description: "Add IANA Considerations section".to_string(),
            fixability: Fixability::ManualOnly,
            category: CheckCategory::IanaSections,
            change: FixChange::XmlInsert {
                parent_path: "/rfc/middle".to_string(),
                position: XmlInsertPosition::Last,
                element: concat!(
                    "<section title=\"IANA Considerations\">\n",
                    "  <t>This document has no IANA actions.</t>\n",
                    "</section>"
                )
                .to_string(),
            },
        })
    } else {
        None
    }
}
