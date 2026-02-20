// SPDX-License-Identifier: PMPL-1.0-or-later

//! Submission streams for Internet Society documents.
//!
//! Each organization has one or more submission streams with distinct
//! workflows, boilerplate requirements, and state machines.

use crate::organization::Organization;
use serde::{Deserialize, Serialize};
use std::fmt;

/// All submission streams across Internet Society organizations.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(tag = "type", content = "detail")]
pub enum Stream {
    // -- IETF streams --
    /// Individual submission (no working group sponsorship)
    IetfIndividual,
    /// Working group document
    IetfWorkingGroup {
        /// The working group abbreviation (e.g., "httpbis")
        wg: String,
    },
    /// Standards Track document
    IetfStandardsTrack {
        wg: Option<String>,
    },
    /// Informational document
    IetfInformational {
        wg: Option<String>,
    },
    /// Experimental document
    IetfExperimental {
        wg: Option<String>,
    },
    /// Best Current Practice
    IetfBcp {
        wg: Option<String>,
    },
    /// BIS (replacement for existing RFC)
    IetfBis {
        obsoletes: Vec<u32>,
    },

    // -- IRTF streams --
    /// Research Group document
    IrtfResearchGroup {
        /// The research group abbreviation
        rg: String,
    },
    /// IRTF individual document
    IrtfIndividual,

    // -- IAB streams --
    /// IAB document
    IabDocument,
    /// IAB informational statement
    IabStatement,

    // -- Independent --
    /// Independent submission (RFC 4846 stream)
    IndependentSubmission,

    // -- IANA --
    /// IANA registry request
    IanaRegistryRequest {
        /// Target registry name
        registry: String,
    },
    /// IANA protocol parameter assignment
    IanaParameterAssignment {
        registry: String,
    },

    // -- RFC Editor --
    /// RFC Editor errata
    RfcEditorErrata {
        rfc: u32,
    },
    /// Editorial stream document
    RfcEditorEditorial,
}

impl Stream {
    /// Returns the organization responsible for this stream.
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

    /// Returns the draft name prefix for this stream (e.g., "draft-ietf-httpbis-").
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

    /// Returns the required boilerplate text identifier for this stream.
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
