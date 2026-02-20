// SPDX-License-Identifier: PMPL-1.0-or-later

//! Template rendering via Nickel CLI.
//!
//! Shells out to the `nickel` CLI for template evaluation,
//! avoiding embedding the full Nickel evaluator as a Rust dependency.

use crate::NickelError;
use std::path::Path;
use std::process::Command;

/// Render a Nickel template to JSON.
pub fn render_template(template_path: &Path) -> Result<String, NickelError> {
    if !template_path.exists() {
        return Err(NickelError::TemplateNotFound(template_path.to_path_buf()));
    }

    let output = Command::new("nickel")
        .arg("export")
        .arg("--format")
        .arg("json")
        .arg(template_path)
        .output()
        .map_err(|e| NickelError::Evaluation(format!("failed to run nickel: {e}")))?;

    if output.status.success() {
        String::from_utf8(output.stdout)
            .map_err(|e| NickelError::Evaluation(format!("invalid UTF-8 output: {e}")))
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(NickelError::Evaluation(stderr.to_string()))
    }
}

/// Validate a Nickel file against its contracts.
pub fn validate_contracts(file_path: &Path) -> Result<(), NickelError> {
    if !file_path.exists() {
        return Err(NickelError::TemplateNotFound(file_path.to_path_buf()));
    }

    let output = Command::new("nickel")
        .arg("typecheck")
        .arg(file_path)
        .output()
        .map_err(|e| NickelError::Evaluation(format!("failed to run nickel: {e}")))?;

    if output.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(NickelError::ContractViolation(stderr.to_string()))
    }
}
