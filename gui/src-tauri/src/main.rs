// SPDX-License-Identifier: PMPL-1.0-or-later

//! Tauri Main Entry — Desktop GUI Orchestrator.
//!
//! This module implements the Tauri 2.0 backend for the transactor GUI. 
//! It defines the RPC bridge that allows the ReScript/React frontend 
//! to invoke high-assurance logic implemented in the Rust crates.
//!
//! DESIGN PILLARS:
//! 1. **Serialization**: Implements `Frontend*` adapter types to ensure 
//!    JSON-compatible data exchange with the WebView.
//! 2. **Safety**: All commands are executed within the standard 
//!    Tauri security sandbox.
//! 3. **Conversion**: Provides loss-less mapping between internal 
//!    Rust enums and frontend-friendly string identifiers.

#![forbid(unsafe_code)]
use serde::{Deserialize, Serialize};
use intsoc_core::validation::{CheckCategory, CheckResult, CheckSummary, Fixability, Severity};
// ... [other imports]

/// FRONTEND ADAPTER: A JSON-serializable version of the domain check result.
#[derive(Debug, Clone, Serialize)]
struct FrontendCheckResult {
    check_id: String,
    severity: String,
    message: String,
    category: String,
    fixable: String,
}

/// TAURI COMMAND: `check_document`.
///
/// Triggers a full audit of the provided `source` text. Returns a 
/// complete summary of findings mapped to the frontend data model.
#[tauri::command]
fn check_document(params: CheckDocumentParams) -> Result<FrontendCheckSummary, String> {
    // 1. PARSE: Transform raw source into domain model.
    // 2. AUDIT: Execute the internal check kernel.
    // 3. MAP: Convert results to FrontendCheckResult.
    // ...
    Ok(summary)
}

// MAIN: Registers commands and boots the Tauri application.
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![check_document, fix_document])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
