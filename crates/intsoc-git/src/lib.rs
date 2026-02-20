// SPDX-License-Identifier: PMPL-1.0-or-later

//! Git integration for Internet Society document versioning.
//!
//! Uses `gix` (pure Rust) for all git operations, avoiding
//! the C dependency of `git2-rs`.

use std::path::{Path, PathBuf};
use thiserror::Error;

/// Git integration errors.
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

/// Git repository wrapper for document versioning.
pub struct DocumentRepo {
    path: PathBuf,
}

impl DocumentRepo {
    /// Open an existing repository.
    pub fn open(path: impl Into<PathBuf>) -> Result<Self, GitError> {
        let path = path.into();
        if !path.join(".git").exists() && !path.join("HEAD").exists() {
            return Err(GitError::NotRepo(path));
        }
        Ok(Self { path })
    }

    /// Initialize a new repository for document tracking.
    pub fn init(path: impl Into<PathBuf>) -> Result<Self, GitError> {
        let path = path.into();
        let repo = gix::init(&path).map_err(|e| GitError::Gix(e.to_string()))?;
        drop(repo);
        Ok(Self { path })
    }

    /// Get the repository path.
    #[must_use]
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Get the current HEAD commit hash.
    pub fn head_hash(&self) -> Result<String, GitError> {
        let repo = gix::open(&self.path).map_err(|e| GitError::Gix(e.to_string()))?;
        let head = repo.head_id().map_err(|e| GitError::Gix(e.to_string()))?;
        Ok(head.to_string())
    }

    /// List files tracked in the repository.
    pub fn tracked_files(&self) -> Result<Vec<PathBuf>, GitError> {
        let repo = gix::open(&self.path).map_err(|e| GitError::Gix(e.to_string()))?;
        let index = repo.index().map_err(|e| GitError::Gix(e.to_string()))?;

        let files: Vec<PathBuf> = index
            .entries()
            .iter()
            .map(|entry| PathBuf::from(entry.path(&index).to_string()))
            .collect();

        Ok(files)
    }

    /// Check if a file has been modified since last commit.
    pub fn is_modified(&self, file: &Path) -> Result<bool, GitError> {
        let repo = gix::open(&self.path).map_err(|e| GitError::Gix(e.to_string()))?;
        let index = repo.index().map_err(|e| GitError::Gix(e.to_string()))?;

        let file_str = file.to_string_lossy();
        let entry = index
            .entries()
            .iter()
            .find(|e| e.path(&index).to_string() == file_str.as_ref());

        match entry {
            Some(_) => {
                // Check if working tree version differs from index
                let full_path = self.path.join(file);
                if !full_path.exists() {
                    return Ok(true); // Deleted
                }
                let metadata = std::fs::metadata(&full_path)?;
                // Simple mtime check
                Ok(metadata.len() > 0) // Placeholder - real impl would compare hashes
            }
            None => Ok(true), // Not tracked = new file
        }
    }
}
