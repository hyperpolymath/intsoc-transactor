// SPDX-License-Identifier: PMPL-1.0-or-later

//! Nickel template rendering and policy validation.
//!
//! Integrates with the `nickel/` directory structure:
//! - `contracts/` - Type contracts for metadata validation
//! - `templates/` - Per-stream document templates
//! - `policies/`  - Submission constraints per organization

pub mod policy;
pub mod render;

use std::path::PathBuf;
use thiserror::Error;

/// Nickel integration errors.
#[derive(Debug, Error)]
pub enum NickelError {
    #[error("template not found: {0}")]
    TemplateNotFound(PathBuf),

    #[error("contract violation: {0}")]
    ContractViolation(String),

    #[error("policy check failed: {0}")]
    PolicyFailed(String),

    #[error("nickel evaluation error: {0}")]
    Evaluation(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

/// Nickel workspace configuration.
#[derive(Debug, Clone)]
pub struct NickelWorkspace {
    /// Root path to the nickel/ directory
    pub root: PathBuf,
}

impl NickelWorkspace {
    /// Create a new Nickel workspace from a root path.
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    /// Path to contracts directory.
    #[must_use]
    pub fn contracts_dir(&self) -> PathBuf {
        self.root.join("contracts")
    }

    /// Path to templates directory.
    #[must_use]
    pub fn templates_dir(&self) -> PathBuf {
        self.root.join("templates")
    }

    /// Path to policies directory.
    #[must_use]
    pub fn policies_dir(&self) -> PathBuf {
        self.root.join("policies")
    }

    /// Get the template path for a specific stream.
    #[must_use]
    pub fn template_for_stream(&self, org: &str, stream_type: &str) -> PathBuf {
        self.templates_dir()
            .join(org.to_lowercase())
            .join(format!("{stream_type}.ncl"))
    }

    /// Check if the workspace structure is valid.
    pub fn validate(&self) -> Result<(), NickelError> {
        for dir in [self.contracts_dir(), self.templates_dir(), self.policies_dir()] {
            if !dir.exists() {
                return Err(NickelError::TemplateNotFound(dir));
            }
        }
        Ok(())
    }
}
