// SPDX-License-Identifier: PMPL-1.0-or-later

//! Git integration for Internet Society document versioning.
//!
//! This module provides a high-assurance interface for managing the
//! submission history of documents.
//!
//! ARCHITECTURAL CHOICE: Uses `gix` (the pure-Rust implementation of Git)
//! instead of `git2-rs` (libgit2 bindings).
//! BENEFITS:
//! 1. Memory Safety: Avoids C-based pointer arithmetic and buffer management.
//! 2. Build Simplicity: No external C library dependencies (libssh2, openssl, etc.).
//! 3. Static Binary Support: Enables easy cross-compilation for verified environments.

#![forbid(unsafe_code)]
use std::path::{Path, PathBuf};
use thiserror::Error;

/// Error space for high-level repository operations.
#[derive(Debug, Error)]
pub enum GitError {
    #[error("repository not found at {0}")]
    NotFound(PathBuf),

    #[error("gix error: {0}")]
    Gix(String),

    #[error("not a git repository: {0}")]
    NotRepo(PathBuf),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

/// A wrapper around a physical Git repository used to track a submission series.
pub struct DocumentRepo {
    /// Physical path to the repository root.
    path: PathBuf,
}
