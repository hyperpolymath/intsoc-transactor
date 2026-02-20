// SPDX-License-Identifier: PMPL-1.0-or-later

//! Transaction model for tracking submission lifecycle.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::document::Document;
use crate::stream::Stream;
use crate::validation::CheckResult;

/// A submission transaction tracking the full lifecycle of a document submission.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    /// Unique transaction ID
    pub id: String,

    /// Document being submitted
    pub document_name: String,

    /// Target stream
    pub stream: Stream,

    /// Current transaction phase
    pub phase: TransactionPhase,

    /// Check results from validation
    pub check_results: Vec<CheckResult>,

    /// Fixes applied during this transaction
    pub fixes_applied: Vec<FixRecord>,

    /// Submission attempts
    pub attempts: Vec<SubmissionAttempt>,

    /// When this transaction was created
    pub created_at: DateTime<Utc>,

    /// When this transaction was last updated
    pub updated_at: DateTime<Utc>,
}

/// Current phase of the submission transaction.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransactionPhase {
    /// Document loaded but not yet checked
    Loaded,
    /// Checking in progress
    Checking,
    /// Check complete, review results
    Checked,
    /// Fixes being applied
    Fixing,
    /// Ready for submission
    ReadyToSubmit,
    /// Submission in progress
    Submitting,
    /// Successfully submitted
    Submitted,
    /// Submission failed
    Failed,
}

/// Record of a fix that was applied.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FixRecord {
    pub fix_id: String,
    pub description: String,
    pub applied_at: DateTime<Utc>,
    pub diff: String,
}

/// Record of a submission attempt.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmissionAttempt {
    pub attempt_number: u32,
    pub timestamp: DateTime<Utc>,
    pub result: SubmissionResult,
}

/// Result of a submission attempt.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SubmissionResult {
    Success { submission_id: String },
    Rejected { reason: String },
    Error { message: String },
}

impl Transaction {
    /// Create a new transaction for a document.
    #[must_use]
    pub fn new(document: &Document) -> Self {
        let now = Utc::now();
        Self {
            id: format!("tx-{}", now.timestamp_millis()),
            document_name: document.name.clone(),
            stream: document.stream.clone(),
            phase: TransactionPhase::Loaded,
            check_results: Vec::new(),
            fixes_applied: Vec::new(),
            attempts: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }
}
