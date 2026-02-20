// SPDX-License-Identifier: PMPL-1.0-or-later

//! Date fix generator.

use chrono::Utc;
use intsoc_core::document::{Document, DocumentFormat};
use intsoc_core::fix::{Fix, FixChange};
use intsoc_core::validation::{CheckCategory, CheckResult, Fixability};

use super::FixGenerator;

/// Generates fixes for incorrect or missing dates.
pub struct DateFixGenerator;

impl FixGenerator for DateFixGenerator {
    fn category(&self) -> CheckCategory {
        CheckCategory::Date
    }

    fn generate(&self, document: &Document, results: &[CheckResult]) -> Vec<Fix> {
        results
            .iter()
            .filter(|r| r.category == CheckCategory::Date)
            .filter_map(|r| generate_date_fix(document, r))
            .collect()
    }
}

fn generate_date_fix(document: &Document, result: &CheckResult) -> Option<Fix> {
    let now = Utc::now().date_naive();
    let year = now.format("%Y").to_string();
    let month = now.format("%B").to_string();
    let day = now.format("%-d").to_string();

    match document.format {
        DocumentFormat::XmlV3 | DocumentFormat::XmlV2 => Some(Fix {
            id: format!("fix-{}", result.check_id),
            check_id: result.check_id.clone(),
            description: format!("Update date to {month} {day}, {year}"),
            fixability: Fixability::AutoSafe,
            category: CheckCategory::Date,
            change: FixChange::XmlReplace {
                path: "/rfc/front/date".to_string(),
                old_value: String::new(),
                new_value: format!(
                    r#"<date year="{year}" month="{month}" day="{day}"/>"#
                ),
            },
        }),
        DocumentFormat::PlainText => {
            // Find and replace the date line
            Some(Fix {
                id: format!("fix-{}", result.check_id),
                check_id: result.check_id.clone(),
                description: format!("Update date to {month} {year}"),
                fixability: Fixability::AutoSafe,
                category: CheckCategory::Date,
                change: FixChange::Replace {
                    start_line: 0,
                    end_line: 0,
                    old_text: String::new(),
                    new_text: format!("{month} {year}"),
                },
            })
        }
    }
}
