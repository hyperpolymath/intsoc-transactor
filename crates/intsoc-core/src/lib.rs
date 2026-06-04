// SPDX-License-Identifier: MPL-2.0
// Copyright (c) Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
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
