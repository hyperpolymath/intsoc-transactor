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
