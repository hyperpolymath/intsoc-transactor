// SPDX-License-Identifier: PMPL-1.0-or-later

//! Internet Society API Interface Layer.
//!
//! This crate provides the network-facing components of the transactor, 
//! implementing asynchronous clients for the formal IETF and IANA services.
//!
//! SERVICES:
//! 1. IETF Datatracker: For document discovery, state tracking, and submission.
//! 2. IANA Registries: For protocol parameter verification and lookup.

pub mod datatracker;
pub mod iana;

use thiserror::Error;

/// Centralized error handling for all external API interactions.
#[derive(Debug, Error)]
pub enum ApiError {
    /// Network-level failure (DNS, TLS, Connection).
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    /// Application-level error returned by the remote service.
    #[error("API error: {status} - {message}")]
    Api { status: u16, message: String },

    /// Requested resource (Draft, RFC, Registry) was not found.
    #[error("not found: {0}")]
    NotFound(String),

    /// Response format did not match the expected schema.
    #[error("deserialization error: {0}")]
    Deserialize(#[from] serde_json::Error),
}
