// SPDX-License-Identifier: PMPL-1.0-or-later

//! Fix types shared between core and fixer crate.

use crate::validation::{CheckCategory, Fixability};
use serde::{Deserialize, Serialize};

/// A proposed fix for a document issue.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fix {
    /// Unique fix identifier
    pub id: String,

    /// The check result this fix addresses
    pub check_id: String,

    /// Human-readable description
    pub description: String,

    /// Fix classification
    pub fixability: Fixability,

    /// Category of the fix
    pub category: CheckCategory,

    /// The actual change to apply
    pub change: FixChange,
}

/// The concrete change a fix makes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FixChange {
    /// Replace text at a range
    Replace {
        start_line: u32,
        end_line: u32,
        old_text: String,
        new_text: String,
    },
    /// Insert text at a position
    Insert { line: u32, text: String },
    /// Delete a range
    Delete { start_line: u32, end_line: u32 },
    /// Replace XML element content
    XmlReplace {
        path: String,
        old_value: String,
        new_value: String,
    },
    /// Insert XML element
    XmlInsert {
        parent_path: String,
        position: XmlInsertPosition,
        element: String,
    },
}

/// Where to insert an XML element.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum XmlInsertPosition {
    First,
    Last,
    Before(String),
    After(String),
}
