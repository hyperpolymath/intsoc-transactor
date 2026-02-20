// SPDX-License-Identifier: PMPL-1.0-or-later

//! Reference fix generator.

use intsoc_core::document::Document;
use intsoc_core::fix::Fix;
use intsoc_core::validation::{CheckCategory, CheckResult, Fixability};

use super::FixGenerator;

/// Generates fixes for reference issues.
pub struct ReferenceFixGenerator;

impl FixGenerator for ReferenceFixGenerator {
    fn category(&self) -> CheckCategory {
        CheckCategory::References
    }

    fn generate(&self, _document: &Document, results: &[CheckResult]) -> Vec<Fix> {
        results
            .iter()
            .filter(|r| r.category == CheckCategory::References)
            .filter_map(|r| {
                // Most reference fixes require manual review
                Some(Fix {
                    id: format!("fix-{}", r.check_id),
                    check_id: r.check_id.clone(),
                    description: format!("Review reference issue: {}", r.message),
                    fixability: Fixability::ManualOnly,
                    category: CheckCategory::References,
                    change: intsoc_core::fix::FixChange::Replace {
                        start_line: 0,
                        end_line: 0,
                        old_text: String::new(),
                        new_text: String::new(),
                    },
                })
            })
            .collect()
    }
}
