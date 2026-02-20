// SPDX-License-Identifier: PMPL-1.0-or-later

//! Boilerplate text fix generator.

use intsoc_core::document::{Document, DocumentFormat, IprDeclaration};
use intsoc_core::fix::{Fix, FixChange};
use intsoc_core::validation::{CheckCategory, CheckResult, Fixability};

use super::FixGenerator;

/// Generates fixes for missing or incorrect boilerplate text.
pub struct BoilerplateFixGenerator;

impl FixGenerator for BoilerplateFixGenerator {
    fn category(&self) -> CheckCategory {
        CheckCategory::Boilerplate
    }

    fn generate(&self, document: &Document, results: &[CheckResult]) -> Vec<Fix> {
        results
            .iter()
            .filter(|r| r.category == CheckCategory::Boilerplate)
            .filter_map(|r| generate_boilerplate_fix(document, r))
            .collect()
    }
}

fn generate_boilerplate_fix(document: &Document, result: &CheckResult) -> Option<Fix> {
    let ipr = document.ipr.unwrap_or(IprDeclaration::Trust200902);
    let boilerplate = trust200902_boilerplate();

    match document.format {
        DocumentFormat::XmlV3 | DocumentFormat::XmlV2 => {
            // For XML, the boilerplate is auto-generated from the ipr attribute.
            // Fix by ensuring the ipr attribute is correct.
            Some(Fix {
                id: format!("fix-{}", result.check_id),
                check_id: result.check_id.clone(),
                description: format!(
                    "Set IPR declaration to '{}'",
                    ipr.xml_value()
                ),
                fixability: Fixability::AutoSafe,
                category: CheckCategory::Boilerplate,
                change: FixChange::XmlReplace {
                    path: "/rfc/@ipr".to_string(),
                    old_value: String::new(),
                    new_value: ipr.xml_value().to_string(),
                },
            })
        }
        DocumentFormat::PlainText => {
            // For plain text, insert boilerplate if missing
            Some(Fix {
                id: format!("fix-{}", result.check_id),
                check_id: result.check_id.clone(),
                description: "Insert IETF Trust Legal Provisions boilerplate".to_string(),
                fixability: Fixability::Recommended,
                category: CheckCategory::Boilerplate,
                change: FixChange::Insert {
                    line: 1,
                    text: boilerplate,
                },
            })
        }
    }
}

fn trust200902_boilerplate() -> String {
    concat!(
        "   This document is subject to BCP 78 and the IETF Trust's Legal\n",
        "   Provisions Relating to IETF Documents\n",
        "   (https://trustee.ietf.org/license-info) in effect on the date of\n",
        "   publication of this document.  Please review these documents\n",
        "   carefully, as they describe your rights and restrictions with respect\n",
        "   to this document.\n",
    )
    .to_string()
}
