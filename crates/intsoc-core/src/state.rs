// SPDX-License-Identifier: PMPL-1.0-or-later

//! State machines for document lifecycle tracking.
//!
//! This module defines the core state machine architecture used to track 
//! Internet Society (IETF, IANA, etc.) documents through their various 
//! administrative and technical review stages.
//!
//! INVARIANT: Only transitions explicitly defined in the `StreamState` 
//! implementation are allowed.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Core interface for any document submission stream state.
/// Ensures states can be displayed, serialized, and compared.
pub trait StreamState: fmt::Display + Clone + PartialEq + Serialize {
    /// Returns the whitelist of valid next states from the current state.
    fn valid_transitions(&self) -> Vec<Self>;

    /// Identifies if the state is a final destination (e.g., Published, Dead).
    fn is_terminal(&self) -> bool;

    /// Defines the entry point for the state machine.
    fn initial() -> Self;
}

/// IETF (Internet Engineering Task Force) document states.
/// Maps to the formal Datatracker state space.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IetfState {
    /// Initial draft state
    Draft,
    /// Automated verification stage
    IdnitsCheck,
    /// Submitted by an individual (not yet a WG document)
    IndividualSubmitted,
    /// Adopted by a Working Group
    WgAdopted,
    /// Official Working Group document
    WgDocument,
    /// Final call within the WG
    WgLastCall,
    /// Preparing for IESG review
    WaitingForWriteup,
    /// Review by Area Director
    AdEvaluation,
    /// Review by the Steering Group
    IesgEvaluation,
    /// Community-wide review period
    IesgLastCall,
    /// Formally approved for publication
    Approved,
    /// Handed off to the RFC Editor
    RfcEditorQueue,
    /// Final 48-hour author review
    Auth48,
    /// Officially published as an RFC
    Published,
    /// Draft has expired without publication
    Expired,
    /// Withdrawn by the authors or WG
    Withdrawn,
    /// Administrative death (no longer active)
    Dead,
    /// Superseded by a newer version/document
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
        // ENFORCEMENT: Defines the valid edges in the state graph.
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
                vec![] // Terminal states have no transitions.
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

// ... [IRTF, IAB, Independent, and IANA states follow same pattern]

/// A recorded transition in the state machine history.
/// Provides an audit trail for the document lifecycle.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transition<S: StreamState> {
    pub from: S,
    pub to: S,
    pub timestamp: DateTime<Utc>,
    pub reason: Option<String>,
}

/// Generic, high-assurance state machine engine.
/// Validates every transition and maintains a persistent history.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateMachine<S: StreamState> {
    current: S,
    history: Vec<Transition<S>>,
}

impl<S: StreamState> StateMachine<S> {
    /// Factory: Starts a new machine at the initial state.
    #[must_use]
    pub fn new() -> Self {
        Self {
            current: S::initial(),
            history: Vec::new(),
        }
    }

    /// Accessor: Returns the current state.
    #[must_use]
    pub fn current(&self) -> &S {
        &self.current
    }

    /// Accessor: Returns the full audit trail.
    #[must_use]
    pub fn history(&self) -> &[Transition<S>] {
        &self.history
    }

    /// EXECUTION: Attempt to transition to a new state.
    ///
    /// Returns `Ok(())` if the transition is allowed by the spec, or `Err` if not.
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

    /// Returns true if the document has reached a terminal (final) state.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.current.is_terminal()
    }

    /// Returns the list of possible next states.
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

/// Error type for illegal state transitions.
#[derive(Debug, thiserror::Error)]
pub enum StateMachineError<S: StreamState> {
    #[error("invalid transition from {from} to {to} (valid: {valid:?})")]
    InvalidTransition {
        from: S,
        to: S,
        valid: Vec<S>,
    },
}
