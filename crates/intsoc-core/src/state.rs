// SPDX-License-Identifier: PMPL-1.0-or-later

//! State machines for document lifecycle tracking.
//!
//! Each Internet Society organization has its own document lifecycle
//! with distinct states and valid transitions.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Trait for stream-specific state enums.
pub trait StreamState: fmt::Display + Clone + PartialEq + Serialize {
    /// Returns all valid next states from the current state.
    fn valid_transitions(&self) -> Vec<Self>;

    /// Whether this is a terminal state.
    fn is_terminal(&self) -> bool;

    /// The initial state for this stream.
    fn initial() -> Self;
}

/// IETF document states (20+ states from Datatracker).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IetfState {
    Draft,
    IdnitsCheck,
    IndividualSubmitted,
    WgAdopted,
    WgDocument,
    WgLastCall,
    WaitingForWriteup,
    AdEvaluation,
    IesgEvaluation,
    IesgLastCall,
    Approved,
    RfcEditorQueue,
    Auth48,
    Published,
    Expired,
    Withdrawn,
    Dead,
    Replaced,
}

impl fmt::Display for IetfState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Draft => write!(f, "Draft"),
            Self::IdnitsCheck => write!(f, "Idnits Check"),
            Self::IndividualSubmitted => write!(f, "Individual Submitted"),
            Self::WgAdopted => write!(f, "WG Adopted"),
            Self::WgDocument => write!(f, "WG Document"),
            Self::WgLastCall => write!(f, "WG Last Call"),
            Self::WaitingForWriteup => write!(f, "Waiting for Writeup"),
            Self::AdEvaluation => write!(f, "AD Evaluation"),
            Self::IesgEvaluation => write!(f, "IESG Evaluation"),
            Self::IesgLastCall => write!(f, "IESG Last Call"),
            Self::Approved => write!(f, "Approved"),
            Self::RfcEditorQueue => write!(f, "RFC Editor Queue"),
            Self::Auth48 => write!(f, "AUTH48"),
            Self::Published => write!(f, "Published"),
            Self::Expired => write!(f, "Expired"),
            Self::Withdrawn => write!(f, "Withdrawn"),
            Self::Dead => write!(f, "Dead"),
            Self::Replaced => write!(f, "Replaced"),
        }
    }
}

impl StreamState for IetfState {
    fn valid_transitions(&self) -> Vec<Self> {
        match self {
            Self::Draft => vec![Self::IdnitsCheck, Self::Expired, Self::Withdrawn],
            Self::IdnitsCheck => vec![
                Self::IndividualSubmitted,
                Self::WgAdopted,
                Self::Draft,
                Self::Expired,
            ],
            Self::IndividualSubmitted => vec![
                Self::AdEvaluation,
                Self::Expired,
                Self::Withdrawn,
                Self::Dead,
            ],
            Self::WgAdopted => vec![Self::WgDocument, Self::Expired, Self::Dead],
            Self::WgDocument => vec![Self::WgLastCall, Self::Expired, Self::Dead, Self::Replaced],
            Self::WgLastCall => vec![
                Self::WaitingForWriteup,
                Self::WgDocument,
                Self::Expired,
                Self::Dead,
            ],
            Self::WaitingForWriteup => vec![Self::AdEvaluation, Self::Expired],
            Self::AdEvaluation => vec![Self::IesgEvaluation, Self::WgDocument, Self::Dead],
            Self::IesgEvaluation => vec![
                Self::IesgLastCall,
                Self::Approved,
                Self::WgDocument,
                Self::Dead,
            ],
            Self::IesgLastCall => vec![Self::Approved, Self::IesgEvaluation, Self::Dead],
            Self::Approved => vec![Self::RfcEditorQueue],
            Self::RfcEditorQueue => vec![Self::Auth48, Self::Approved],
            Self::Auth48 => vec![Self::Published, Self::RfcEditorQueue],
            Self::Published | Self::Expired | Self::Withdrawn | Self::Dead | Self::Replaced => {
                vec![]
            }
        }
    }

    fn is_terminal(&self) -> bool {
        matches!(
            self,
            Self::Published | Self::Expired | Self::Withdrawn | Self::Dead | Self::Replaced
        )
    }

    fn initial() -> Self {
        Self::Draft
    }
}

/// IRTF document states (8 states).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IrtfState {
    Draft,
    RgDocument,
    RgLastCall,
    IrsgReview,
    IesgConflictReview,
    Approved,
    Published,
    Expired,
}

impl fmt::Display for IrtfState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Draft => write!(f, "Draft"),
            Self::RgDocument => write!(f, "RG Document"),
            Self::RgLastCall => write!(f, "RG Last Call"),
            Self::IrsgReview => write!(f, "IRSG Review"),
            Self::IesgConflictReview => write!(f, "IESG Conflict Review"),
            Self::Approved => write!(f, "Approved"),
            Self::Published => write!(f, "Published"),
            Self::Expired => write!(f, "Expired"),
        }
    }
}

impl StreamState for IrtfState {
    fn valid_transitions(&self) -> Vec<Self> {
        match self {
            Self::Draft => vec![Self::RgDocument, Self::Expired],
            Self::RgDocument => vec![Self::RgLastCall, Self::Expired],
            Self::RgLastCall => vec![Self::IrsgReview, Self::RgDocument, Self::Expired],
            Self::IrsgReview => vec![Self::IesgConflictReview, Self::RgDocument],
            Self::IesgConflictReview => vec![Self::Approved, Self::RgDocument],
            Self::Approved => vec![Self::Published],
            Self::Published | Self::Expired => vec![],
        }
    }

    fn is_terminal(&self) -> bool {
        matches!(self, Self::Published | Self::Expired)
    }

    fn initial() -> Self {
        Self::Draft
    }
}

/// IAB document states (4 states).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IabState {
    Draft,
    IabReview,
    Approved,
    Published,
}

impl fmt::Display for IabState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Draft => write!(f, "Draft"),
            Self::IabReview => write!(f, "IAB Review"),
            Self::Approved => write!(f, "Approved"),
            Self::Published => write!(f, "Published"),
        }
    }
}

impl StreamState for IabState {
    fn valid_transitions(&self) -> Vec<Self> {
        match self {
            Self::Draft => vec![Self::IabReview],
            Self::IabReview => vec![Self::Approved, Self::Draft],
            Self::Approved => vec![Self::Published],
            Self::Published => vec![],
        }
    }

    fn is_terminal(&self) -> bool {
        matches!(self, Self::Published)
    }

    fn initial() -> Self {
        Self::Draft
    }
}

/// Independent submission states (5 states).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IndependentState {
    Submitted,
    IseReview,
    IesgConflictReview,
    Approved,
    Published,
}

impl fmt::Display for IndependentState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Submitted => write!(f, "Submitted"),
            Self::IseReview => write!(f, "ISE Review"),
            Self::IesgConflictReview => write!(f, "IESG Conflict Review"),
            Self::Approved => write!(f, "Approved"),
            Self::Published => write!(f, "Published"),
        }
    }
}

impl StreamState for IndependentState {
    fn valid_transitions(&self) -> Vec<Self> {
        match self {
            Self::Submitted => vec![Self::IseReview],
            Self::IseReview => vec![Self::IesgConflictReview, Self::Submitted],
            Self::IesgConflictReview => vec![Self::Approved, Self::IseReview],
            Self::Approved => vec![Self::Published],
            Self::Published => vec![],
        }
    }

    fn is_terminal(&self) -> bool {
        matches!(self, Self::Published)
    }

    fn initial() -> Self {
        Self::Submitted
    }
}

/// IANA request states (6 states).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IanaState {
    Drafted,
    Submitted,
    ExpertReview,
    IanaReview,
    Completed,
    Rejected,
}

impl fmt::Display for IanaState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Drafted => write!(f, "Drafted"),
            Self::Submitted => write!(f, "Submitted"),
            Self::ExpertReview => write!(f, "Expert Review"),
            Self::IanaReview => write!(f, "IANA Review"),
            Self::Completed => write!(f, "Completed"),
            Self::Rejected => write!(f, "Rejected"),
        }
    }
}

impl StreamState for IanaState {
    fn valid_transitions(&self) -> Vec<Self> {
        match self {
            Self::Drafted => vec![Self::Submitted],
            Self::Submitted => vec![Self::ExpertReview, Self::IanaReview],
            Self::ExpertReview => vec![Self::IanaReview, Self::Rejected],
            Self::IanaReview => vec![Self::Completed, Self::Rejected],
            Self::Completed | Self::Rejected => vec![],
        }
    }

    fn is_terminal(&self) -> bool {
        matches!(self, Self::Completed | Self::Rejected)
    }

    fn initial() -> Self {
        Self::Drafted
    }
}

/// A transition record in the state machine history.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transition<S: StreamState> {
    pub from: S,
    pub to: S,
    pub timestamp: DateTime<Utc>,
    pub reason: Option<String>,
}

/// Generic state machine runner with transition validation and history.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateMachine<S: StreamState> {
    current: S,
    history: Vec<Transition<S>>,
}

impl<S: StreamState> StateMachine<S> {
    /// Create a new state machine at the initial state.
    #[must_use]
    pub fn new() -> Self {
        Self {
            current: S::initial(),
            history: Vec::new(),
        }
    }

    /// Get the current state.
    #[must_use]
    pub fn current(&self) -> &S {
        &self.current
    }

    /// Get the transition history.
    #[must_use]
    pub fn history(&self) -> &[Transition<S>] {
        &self.history
    }

    /// Attempt to transition to a new state.
    ///
    /// Returns `Ok(())` if the transition is valid, or an error if not.
    pub fn transition(
        &mut self,
        to: S,
        reason: Option<String>,
    ) -> Result<(), StateMachineError<S>> {
        let valid = self.current.valid_transitions();
        if !valid.contains(&to) {
            return Err(StateMachineError::InvalidTransition {
                from: self.current.clone(),
                to,
                valid,
            });
        }

        let transition = Transition {
            from: self.current.clone(),
            to: to.clone(),
            timestamp: Utc::now(),
            reason,
        };

        self.history.push(transition);
        self.current = to;
        Ok(())
    }

    /// Check if the state machine is in a terminal state.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.current.is_terminal()
    }

    /// Get all valid next states from the current state.
    #[must_use]
    pub fn available_transitions(&self) -> Vec<S> {
        self.current.valid_transitions()
    }
}

impl<S: StreamState> Default for StateMachine<S> {
    fn default() -> Self {
        Self::new()
    }
}

/// State machine errors.
#[derive(Debug, thiserror::Error)]
pub enum StateMachineError<S: StreamState> {
    #[error("invalid transition from {from} to {to} (valid: {valid:?})")]
    InvalidTransition {
        from: S,
        to: S,
        valid: Vec<S>,
    },
}
