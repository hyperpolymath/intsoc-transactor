// SPDX-License-Identifier: PMPL-1.0-or-later

//! IETF Datatracker and IANA API clients.
//!
//! Provides async clients for:
//! - IETF Datatracker API (document lookup, submission, status)
//! - IANA registry lookups

pub mod datatracker;
pub mod iana;

use thiserror::Error;

/// API client errors.
#[derive(Debug, Error)]
pub enum ApiError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("API error: {status} - {message}")]
    Api { status: u16, message: String },

    #[error("not found: {0}")]
    NotFound(String),

    #[error("deserialization error: {0}")]
    Deserialize(#[from] serde_json::Error),
}
