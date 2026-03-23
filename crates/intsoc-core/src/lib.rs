// SPDX-License-Identifier: PMPL-1.0-or-later

//! Internet Society Transactor - Core Domain Model
//!
//! Provides the domain types, state machines, and validation framework
//! for processing documents across all Internet Society organizations:
//! IETF, IRTF, IAB, Independent Stream, IANA, and RFC Editor.

#![forbid(unsafe_code)]
pub mod document;
pub mod fix;
pub mod organization;
pub mod state;
pub mod stream;
pub mod submission;
pub mod transaction;
pub mod validation;
