// SPDX-License-Identifier: PMPL-1.0-or-later

//! Internet Society Document Fix Engine
//!
//! Generates, plans, and applies fixes to Internet-Drafts and other
//! Internet Society documents. Uses the AutoSafe/Recommended/ManualOnly
//! classification from the validation framework.

#![forbid(unsafe_code)]
pub mod diff;
pub mod engine;
pub mod generators;

use intsoc_core::document::Document;
use intsoc_core::fix::Fix;
use intsoc_core::validation::Fixability;
use thiserror::Error;

/// Fix engine errors.
#[derive(Debug, Error)]
pub enum FixError {
    #[error("fix conflicts with existing change: {0}")]
    Conflict(String),

    #[error("fix target not found: {0}")]
    TargetNotFound(String),

    #[error("cannot apply fix in current state: {0}")]
    InvalidState(String),
}

/// A plan of fixes to apply to a document.
#[derive(Debug, Clone)]
pub struct FixPlan {
    /// Fixes to apply, in order
    pub fixes: Vec<Fix>,

    /// Original document source (for undo)
    pub original_source: String,

    /// Whether the plan has been applied
    pub applied: bool,
}

impl FixPlan {
    /// Create a new fix plan for a document.
    #[must_use]
    pub fn new(document: &Document) -> Self {
        Self {
            fixes: Vec::new(),
            original_source: document.source.clone(),
            applied: false,
        }
    }

    /// Add a fix to the plan.
    pub fn add(&mut self, fix: Fix) {
        self.fixes.push(fix);
    }

    /// Get only AutoSafe fixes.
    #[must_use]
    pub fn auto_safe_fixes(&self) -> Vec<&Fix> {
        self.fixes
            .iter()
            .filter(|f| f.fixability == Fixability::AutoSafe)
            .collect()
    }

    /// Get only Recommended fixes.
    #[must_use]
    pub fn recommended_fixes(&self) -> Vec<&Fix> {
        self.fixes
            .iter()
            .filter(|f| f.fixability == Fixability::Recommended)
            .collect()
    }

    /// Get only ManualOnly fixes.
    #[must_use]
    pub fn manual_only_fixes(&self) -> Vec<&Fix> {
        self.fixes
            .iter()
            .filter(|f| f.fixability == Fixability::ManualOnly)
            .collect()
    }
}

/// Result of applying fixes to a document (consumed by the GUI backend).
#[derive(Debug, Clone)]
pub struct FixResult {
    /// Whether the fix operation completed without errors.
    pub success: bool,
    /// The full document source after fixes have been applied.
    pub fixed_source: String,
    /// Unified diff showing what was changed.
    pub diff_preview: String,
    /// Number of auto-safe fixes applied.
    pub auto_safe_applied: usize,
    /// Number of recommended fixes applied.
    pub recommended_applied: usize,
    /// Number of issues remaining that require manual intervention.
    pub manual_remaining: usize,
}

/// Run the fix engine on a parsed document.
///
/// # Arguments
///
/// * `document` - The parsed document to fix.
/// * `auto_only` - If true, only apply `AutoSafe` fixes.
/// * `_dry_run` - If true, generate a diff preview but do not apply changes.
///
/// # Errors
///
/// Returns an error description if the fixer encounters an unrecoverable issue.
pub fn fix_document(
    document: &Document,
    _auto_only: bool,
    _dry_run: bool,
) -> Result<FixResult, String> {
    // TODO: Implement full fix pipeline.
    // For now, return the source unchanged so the GUI can boot.
    Ok(FixResult {
        success: true,
        fixed_source: document.source.clone(),
        diff_preview: String::new(),
        auto_safe_applied: 0,
        recommended_applied: 0,
        manual_remaining: 0,
    })
}
