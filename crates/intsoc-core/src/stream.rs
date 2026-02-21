// SPDX-License-Identifier: PMPL-1.0-or-later

//! Submission streams for Internet Society documents.
//!
//! This module defines the various "Streams" available for document submission.
//! Each stream (IETF, IRTF, Independent, etc.) has its own governance,
//! boilerplate requirements, and expected name prefixes.

use crate::organization::Organization;
use serde::{Deserialize, Serialize};
use std::fmt;

/// All formal submission streams across Internet Society organizations.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(tag = "type", content = "detail")]
pub enum Stream {
    // -- IETF STREAMS (RFC 2026) --
    /// Standard individual submission
    IetfIndividual,
    /// Working group sponsored document
    IetfWorkingGroup {
        /// The working group abbreviation (e.g., "httpbis")
        wg: String,
    },
    /// Standards Track (Proposed Standard, Internet Standard)
    IetfStandardsTrack {
        wg: Option<String>,
    },
    /// Informational document (RFC 2026 Section 4.2.2)
    IetfInformational {
        wg: Option<String>,
    },
    /// Experimental document (RFC 2026 Section 4.2.1)
    IetfExperimental {
        wg: Option<String>,
    },
    /// Best Current Practice (RFC 2026 Section 5)
    IetfBcp {
        wg: Option<String>,
    },
    /// BIS (replacement for existing RFC)
    IetfBis {
        obsoletes: Vec<u32>,
    },

    // -- IRTF STREAMS (RFC 5743) --
    /// Official Research Group document
    IrtfResearchGroup {
        rg: String,
    },
    /// Individual submission to the IRTF stream
    IrtfIndividual,

    // -- IAB STREAMS (RFC 4845) --
    /// Formal IAB document
    IabDocument,
    /// IAB informational statement or opinion
    IabStatement,

    // -- INDEPENDENT STREAM (RFC 4846) --
    /// Independent submission (non-IETF sponsored)
    IndependentSubmission,

    // -- IANA (Non-RFC stream) --
    /// Request for a new IANA registry
    IanaRegistryRequest {
        registry: String,
    },
    /// Protocol parameter assignment in an existing registry
    IanaParameterAssignment {
        registry: String,
    },

    // -- RFC EDITOR STREAMS (RFC 8729) --
    /// Official errata for an existing RFC
    RfcEditorErrata {
        rfc: u32,
    },
    /// Document originating from the RFC Editor's editorial stream
    RfcEditorEditorial,
}

impl Stream {
    /// Returns the organization responsible for managing this stream.
    #[must_use]
    pub fn organization(&self) -> Organization {
        match self {
            Self::IetfIndividual
            | Self::IetfWorkingGroup { .. }
            | Self::IetfStandardsTrack { .. }
            | Self::IetfInformational { .. }
            | Self::IetfExperimental { .. }
            | Self::IetfBcp { .. }
            | Self::IetfBis { .. } => Organization::Ietf,

            Self::IrtfResearchGroup { .. } | Self::IrtfIndividual => Organization::Irtf,

            Self::IabDocument | Self::IabStatement => Organization::Iab,

            Self::IndependentSubmission => Organization::Independent,

            Self::IanaRegistryRequest { .. } | Self::IanaParameterAssignment { .. } => {
                Organization::Iana
            }

            Self::RfcEditorErrata { .. } | Self::RfcEditorEditorial => Organization::RfcEditor,
        }
    }

    /// Returns the standardized draft name prefix for this stream.
    /// Example: `draft-ietf-httpbis-` for an IETF WG document.
    #[must_use]
    pub fn draft_prefix(&self, author_last_name: &str) -> Option<String> {
        match self {
            Self::IetfIndividual => Some(format!("draft-{author_last_name}-")),
            Self::IetfWorkingGroup { wg } => Some(format!("draft-ietf-{wg}-")),
            Self::IetfStandardsTrack { wg: Some(wg) } => Some(format!("draft-ietf-{wg}-")),
            Self::IetfInformational { wg: Some(wg) } => Some(format!("draft-ietf-{wg}-")),
            Self::IetfExperimental { wg: Some(wg) } => Some(format!("draft-ietf-{wg}-")),
            Self::IetfBcp { wg: Some(wg) } => Some(format!("draft-ietf-{wg}-")),
            Self::IetfBis { .. } => Some(format!("draft-{author_last_name}-")),
            Self::IrtfResearchGroup { rg } => Some(format!("draft-irtf-{rg}-")),
            Self::IrtfIndividual => Some(format!("draft-{author_last_name}-")),
            _ => None,
        }
    }

    /// Returns the IETF Trust boilerplate ID required for the document header.
    #[must_use]
    pub fn boilerplate_id(&self) -> &'static str {
        match self.organization() {
            Organization::Ietf => "trust200902",
            Organization::Irtf => "trust200902",
            Organization::Iab => "trust200902",
            Organization::Independent => "trust200902",
            Organization::Iana => "iana-submission",
            Organization::RfcEditor => "rfc-editor",
        }
    }
}

impl fmt::Display for Stream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IetfIndividual => write!(f, "IETF Individual"),
            Self::IetfWorkingGroup { wg } => write!(f, "IETF WG ({wg})"),
            Self::IetfStandardsTrack { wg } => {
                write!(f, "IETF Standards Track")?;
                if let Some(wg) = wg {
                    write!(f, " ({wg})")?;
                }
                Ok(())
            }
            Self::IetfInformational { wg } => {
                write!(f, "IETF Informational")?;
                if let Some(wg) = wg {
                    write!(f, " ({wg})")?;
                }
                Ok(())
            }
            Self::IetfExperimental { wg } => {
                write!(f, "IETF Experimental")?;
                if let Some(wg) = wg {
                    write!(f, " ({wg})")?;
                }
                Ok(())
            }
            Self::IetfBcp { wg } => {
                write!(f, "IETF BCP")?;
                if let Some(wg) = wg {
                    write!(f, " ({wg})")?;
                }
                Ok(())
            }
            Self::IetfBis { obsoletes } => {
                let rfcs: Vec<String> = obsoletes.iter().map(|r| format!("RFC {r}")).collect();
                write!(f, "IETF BIS (obsoletes {})", rfcs.join(", "))
            }
            Self::IrtfResearchGroup { rg } => write!(f, "IRTF RG ({rg})"),
            Self::IrtfIndividual => write!(f, "IRTF Individual"),
            Self::IabDocument => write!(f, "IAB Document"),
            Self::IabStatement => write!(f, "IAB Statement"),
            Self::IndependentSubmission => write!(f, "Independent Submission"),
            Self::IanaRegistryRequest { registry } => {
                write!(f, "IANA Registry Request ({registry})")
            }
            Self::IanaParameterAssignment { registry } => {
                write!(f, "IANA Parameter Assignment ({registry})")
            }
            Self::RfcEditorErrata { rfc } => write!(f, "RFC Editor Errata (RFC {rfc})"),
            Self::RfcEditorEditorial => write!(f, "RFC Editor Editorial"),
        }
    }
}
