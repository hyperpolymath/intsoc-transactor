// SPDX-License-Identifier: PMPL-1.0-or-later

//! Internet Society organization types.

use serde::{Deserialize, Serialize};
use std::fmt;

/// All Internet Society organizations that produce or process documents.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Organization {
    /// Internet Engineering Task Force
    Ietf,
    /// Internet Research Task Force
    Irtf,
    /// Internet Architecture Board
    Iab,
    /// Independent Submission Stream
    Independent,
    /// Internet Assigned Numbers Authority
    Iana,
    /// RFC Editor
    RfcEditor,
}

impl fmt::Display for Organization {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ietf => write!(f, "IETF"),
            Self::Irtf => write!(f, "IRTF"),
            Self::Iab => write!(f, "IAB"),
            Self::Independent => write!(f, "Independent"),
            Self::Iana => write!(f, "IANA"),
            Self::RfcEditor => write!(f, "RFC Editor"),
        }
    }
}

impl Organization {
    /// Returns the Datatracker URL prefix for this organization.
    #[must_use]
    pub fn datatracker_base(&self) -> &'static str {
        match self {
            Self::Ietf | Self::Irtf | Self::Iab | Self::Independent => {
                "https://datatracker.ietf.org"
            }
            Self::Iana => "https://www.iana.org",
            Self::RfcEditor => "https://www.rfc-editor.org",
        }
    }

    /// Whether this organization uses the IETF Datatracker for submissions.
    #[must_use]
    pub fn uses_datatracker(&self) -> bool {
        matches!(
            self,
            Self::Ietf | Self::Irtf | Self::Iab | Self::Independent
        )
    }
}
