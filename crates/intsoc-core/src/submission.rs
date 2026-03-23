// SPDX-License-Identifier: PMPL-1.0-or-later

//! Submission Status — Datatracker Query Interface.
//!
//! This module provides a stub for querying the IETF Datatracker for
//! the submission state of a named document. The actual API integration
//! lives in the `intsoc-api` crate; this module exposes the domain types
//! and a convenience facade used by the Gossamer GUI backend.

use serde::{Deserialize, Serialize};

/// The submission status of a document on the IETF Datatracker.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmissionStatus {
    /// The canonical document name (e.g. "draft-ietf-example-protocol-07").
    pub document_name: String,
    /// The IETF stream (e.g. "IETF", "IAB", "IRTF", "ISE").
    pub stream: String,
    /// Current submission state (e.g. "posted", "auth", "ietf").
    pub state: String,
    /// Whether the document has been submitted to the Datatracker.
    pub submitted: bool,
    /// URL to the Datatracker page, if available.
    pub datatracker_url: Option<String>,
    /// Human-readable status message.
    pub message: String,
}

/// Query the IETF Datatracker for the submission status of a document.
///
/// # Errors
///
/// Returns an error description if the query fails (network, auth, etc.).
pub fn get_status(document_name: &str) -> Result<SubmissionStatus, String> {
    // TODO: Implement actual Datatracker API query via intsoc-api.
    // For now, return a "not submitted" stub so the GUI can boot.
    Ok(SubmissionStatus {
        document_name: document_name.to_string(),
        stream: String::new(),
        state: "unknown".to_string(),
        submitted: false,
        datatracker_url: None,
        message: "Submission status not yet implemented".to_string(),
    })
}
