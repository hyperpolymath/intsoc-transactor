// SPDX-License-Identifier: PMPL-1.0-or-later

//! Submission Transaction — Lifecycle State Tracking.
//!
//! This module implements the `Transaction` entity, which tracks the 
//! end-to-end journey of a document from initial ingestion to successful 
//! submission to the Datatracker.
//!
//! AUDIT TRAIL:
//! Every transaction records the specific validation findings (`check_results`), 
//! the remediation actions taken (`fixes_applied`), and the outcome 
//! of each network submission attempt.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::document::Document;
use crate::stream::Stream;
use crate::validation::CheckResult;

/// TRANSACTION: The stateful container for a submission session.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: String,
    pub document_name: String,
    pub stream: Stream,
    pub phase: TransactionPhase,
    pub check_results: Vec<CheckResult>,
    pub fixes_applied: Vec<FixRecord>,
    pub attempts: Vec<SubmissionAttempt>,
}

/// PHASE: The current administrative milestone of the transaction.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransactionPhase {
    Loaded,        // Document parsed.
    Checking,      // Validation in progress.
    Checked,       // Results ready for review.
    Fixing,        // Auto-remediation active.
    ReadyToSubmit, // All mandatory checks passed.
    Submitting,    // Network IO active.
    Submitted,     // ACK received from Datatracker.
    Failed,        // Fatal error or rejection.
}

/// FIX RECORD: A historical marker for an applied code change.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FixRecord {
    pub fix_id: String,
    pub description: String,
    pub applied_at: DateTime<Utc>,
    pub diff: String, // Unified diff of the change.
}
