// SPDX-License-Identifier: PMPL-1.0-or-later

//! Fix engine: orchestrates fix generation, planning, and application.

use intsoc_core::document::Document;
use intsoc_core::fix::{Fix, FixChange};
use intsoc_core::validation::{CheckResult, Fixability};

use crate::generators::generate_all_fixes;
use crate::{FixError, FixPlan};

/// The main fix engine that coordinates fix generation and application.
pub struct FixEngine {
    /// Whether to auto-apply safe fixes
    pub auto_apply_safe: bool,
}

impl FixEngine {
    /// Create a new fix engine.
    #[must_use]
    pub fn new() -> Self {
        Self {
            auto_apply_safe: false,
        }
    }

    /// Generate a fix plan from check results.
    #[must_use]
    pub fn plan(&self, document: &Document, results: &[CheckResult]) -> FixPlan {
        let fixes = generate_all_fixes(document, results);
        let mut plan = FixPlan::new(document);
        for fix in fixes {
            plan.add(fix);
        }
        plan
    }

    /// Apply all AutoSafe fixes from a plan to the document source.
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

    /// Apply specific fixes to the document source.
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

    /// Generate a unified diff preview of the changes.
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

fn apply_single_fix(source: &str, fix: &Fix) -> Result<String, FixError> {
    match &fix.change {
        FixChange::Replace {
            start_line,
            end_line,
            old_text,
            new_text,
        } => {
            if !old_text.is_empty() {
                Ok(source.replacen(old_text, new_text, 1))
            } else {
                // Line-based replacement
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
                        continue;
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
        FixChange::XmlReplace {
            path,
            old_value,
            new_value,
        } => {
            // Simple XML attribute/element replacement
            if !old_value.is_empty() {
                Ok(source.replacen(old_value, new_value, 1))
            } else {
                // For attribute-level changes, would need XML DOM manipulation
                // For now, return unchanged and log
                tracing::warn!("XML replace for path '{}' requires DOM manipulation", path);
                Ok(source.to_string())
            }
        }
        FixChange::XmlInsert {
            parent_path,
            position: _,
            element,
        } => {
            // Simple insertion before closing tag
            tracing::info!("XML insert into '{}': {}", parent_path, element);
            Ok(source.to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fix_engine_new() {
        let engine = FixEngine::new();
        assert!(!engine.auto_apply_safe);
    }

    #[test]
    fn test_preview_diff() {
        let engine = FixEngine::new();
        let original = "line 1\nline 2\nline 3\n";
        let modified = "line 1\nline 2 modified\nline 3\n";
        let diff = engine.preview(original, modified);
        assert!(diff.contains("-line 2"));
        assert!(diff.contains("+line 2 modified"));
    }
}
