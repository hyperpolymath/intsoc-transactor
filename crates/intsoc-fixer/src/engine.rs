// SPDX-License-Identifier: PMPL-1.0-or-later

//! Fix Engine Core Logic.
//!
//! This module orchestrates the automatic remediation of validation findings.
//! It transforms `CheckResult` findings into concrete `Fix` operations and 
//! applies them to the document source code.

use intsoc_core::document::Document;
use intsoc_core::fix::{Fix, FixChange};
use intsoc_core::validation::{CheckResult, Fixability};

use crate::generators::generate_all_fixes;
use crate::{FixError, FixPlan};

/// The primary remediation coordinator.
pub struct FixEngine {
    /// POLICY: If true, AutoSafe fixes are applied without user confirmation.
    pub auto_apply_safe: bool,
}

impl FixEngine {
    /// Factory: Creates a new fix engine instance.
    #[must_use]
    pub fn new() -> Self {
        Self {
            auto_apply_safe: false,
        }
    }

    /// PLANNING: Generates a comprehensive `FixPlan` based on validation results.
    /// Uses specialized generators to find the best remediation for each finding.
    #[must_use]
    pub fn plan(&self, document: &Document, results: &[CheckResult]) -> FixPlan {
        let fixes = generate_all_fixes(document, results);
        let mut plan = FixPlan::new(document);
        for fix in fixes {
            plan.add(fix);
        }
        plan
    }

    /// EXECUTION: Applies all `AutoSafe` (high-confidence) fixes from a plan.
    pub fn apply_auto_safe(
        &self,
        source: &str,
        plan: &FixPlan,
    ) -> Result<String, FixError> {
        let safe_fixes: Vec<&Fix> = plan
            .fixes
            .iter()
            .filter(|f| f.fixability == Fixability::AutoSafe)
            .collect();

        self.apply_fixes(source, &safe_fixes)
    }

    /// EXECUTION: Sequentially applies a list of specific fixes to the document.
    pub fn apply_fixes(
        &self,
        source: &str,
        fixes: &[&Fix],
    ) -> Result<String, FixError> {
        let mut result = source.to_string();

        for fix in fixes {
            result = apply_single_fix(&result, fix)?;
        }

        Ok(result)
    }

    /// VISUALIZATION: Generates a standard unified diff between original and modified source.
    /// Uses the `similar` crate for high-quality line-based diffing.
    #[must_use]
    pub fn preview(&self, original: &str, modified: &str) -> String {
        use similar::TextDiff;

        let diff = TextDiff::from_lines(original, modified);
        let mut output = String::new();

        for change in diff.iter_all_changes() {
            let sign = match change.tag() {
                similar::ChangeTag::Delete => "-",
                similar::ChangeTag::Insert => "+",
                similar::ChangeTag::Equal => " ",
            };
            output.push_str(sign);
            output.push_str(change.value());
        }

        output
    }
}

impl Default for FixEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// CORE MUTATION LOGIC: Implements the physical changes to the source string.
/// Supports multiple change types:
/// - `Replace`: Line-based or text-fragment replacement.
/// - `Insert`: Inserting text at a specific line.
/// - `Delete`: Removing a range of lines.
/// - `XmlReplace/Insert`: (Future) Path-aware XML mutations.
fn apply_single_fix(source: &str, fix: &Fix) -> Result<String, FixError> {
    match &fix.change {
        FixChange::Replace {
            start_line,
            end_line,
            old_text,
            new_text,
        } => {
            if !old_text.is_empty() {
                // Fragment-based replacement
                Ok(source.replacen(old_text, new_text, 1))
            } else {
                // Physical line-based replacement
                let lines: Vec<&str> = source.lines().collect();
                let start = *start_line as usize;
                let end = *end_line as usize;

                if start >= lines.len() {
                    return Err(FixError::TargetNotFound(format!(
                        "line {start} out of range"
                    )));
                }

                let mut result = Vec::new();
                for (i, line) in lines.iter().enumerate() {
                    if i == start {
                        result.push(new_text.as_str());
                    } else if i > start && i <= end {
                        continue; // Skip lines being replaced
                    } else {
                        result.push(line);
                    }
                }
                Ok(result.join("\n"))
            }
        }
        FixChange::Insert { line, text } => {
            let lines: Vec<&str> = source.lines().collect();
            let pos = *line as usize;
            let mut result = Vec::new();

            for (i, l) in lines.iter().enumerate() {
                if i == pos {
                    result.push(text.as_str());
                }
                result.push(l);
            }

            if pos >= lines.len() {
                result.push(text.as_str());
            }

            Ok(result.join("\n"))
        }
        FixChange::Delete {
            start_line,
            end_line,
        } => {
            let lines: Vec<&str> = source.lines().collect();
            let start = *start_line as usize;
            let end = *end_line as usize;

            let result: Vec<&str> = lines
                .iter()
                .enumerate()
                .filter(|(i, _)| *i < start || *i > end)
                .map(|(_, l)| *l)
                .collect();

            Ok(result.join("\n"))
        }
        // ... [future XML-aware mutation logic]
        _ => {
            tracing::warn!("Fix type not yet fully implemented for direct source manipulation");
            Ok(source.to_string())
        }
    }
}
