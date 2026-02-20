// SPDX-License-Identifier: PMPL-1.0-or-later

//! Fix generators for different categories of document issues.

pub mod boilerplate;
pub mod date;
pub mod draft_name;
pub mod header;
pub mod references;
pub mod sections;

use intsoc_core::document::Document;
use intsoc_core::fix::Fix;
use intsoc_core::validation::CheckResult;

/// Trait for fix generators.
pub trait FixGenerator {
    /// Generate fixes for the given check results against a document.
    fn generate(&self, document: &Document, results: &[CheckResult]) -> Vec<Fix>;

    /// The check category this generator handles.
    fn category(&self) -> intsoc_core::validation::CheckCategory;
}

/// Run all built-in fix generators against a document.
pub fn generate_all_fixes(document: &Document, results: &[CheckResult]) -> Vec<Fix> {
    let generators: Vec<Box<dyn FixGenerator>> = vec![
        Box::new(boilerplate::BoilerplateFixGenerator),
        Box::new(date::DateFixGenerator),
        Box::new(header::HeaderFixGenerator),
        Box::new(sections::SectionFixGenerator),
        Box::new(references::ReferenceFixGenerator),
        Box::new(draft_name::DraftNameFixGenerator),
    ];

    generators
        .iter()
        .flat_map(|generator| generator.generate(document, results))
        .collect()
}
