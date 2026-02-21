// SPDX-License-Identifier: PMPL-1.0-or-later

//! IETF Datatracker REST API Client.
//!
//! Implements high-level operations against the authoritative IETF document 
//! management system. Supports both authenticated and public read-only access.
//!
//! API REF: https://datatracker.ietf.org/api/

use crate::ApiError;
use serde::{Deserialize, Serialize};

const BASE_URL: &str = "https://datatracker.ietf.org";

/// The primary interface for interacting with the Datatracker.
pub struct DataTrackerClient {
    client: reqwest::Client,
    base_url: String,
}

/// Aggregated metadata for an Internet-Draft or RFC.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DraftInfo {
    pub name: String,
    pub title: String,
    pub rev: String, // Revision number (e.g. "00")
    pub pages: Option<u32>,
    pub time: String,
    pub expires: Option<String>,
    pub group: Option<GroupInfo>,
    pub stream: Option<String>,
    pub intended_std_level: Option<String>,
}

/// Metadata for the Working Group or Research Group responsible for a document.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupInfo {
    pub acronym: String,
    pub name: String,
    #[serde(rename = "type")]
    pub group_type: String, // e.g. "wg", "rg", "area"
}

/// Represents the status of a specific document submission attempt.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmissionStatus {
    pub id: u64,
    pub name: String,
    pub rev: String,
    pub state: String,
    pub submission_date: String,
}

impl DataTrackerClient {
    /// FACTORY: Initializes a client with a standard user-agent.
    #[must_use]
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::builder()
                .user_agent("intsoc-transactor/0.1.0")
                .build()
                .expect("failed to build HTTP client"),
            base_url: BASE_URL.to_string(),
        }
    }

    /// LOOKUP: Retrieves full metadata for a document by its name (e.g., "draft-ietf-httpbis-brotli").
    pub async fn get_draft(&self, name: &str) -> Result<DraftInfo, ApiError> {
        let url = format!("{}/api/v1/doc/document/{name}/", self.base_url);
        let resp = self.client.get(&url).send().await?;

        if resp.status() == reqwest::StatusCode::NOT_FOUND {
            return Err(ApiError::NotFound(name.to_string()));
        }

        if !resp.status().is_success() {
            return Err(ApiError::Api {
                status: resp.status().as_u16(),
                message: resp.text().await.unwrap_or_default(),
            });
        }

        Ok(resp.json().await?)
    }

    /// STATUS: Lists all submission events for a given document name.
    pub async fn submission_status(&self, name: &str) -> Result<Vec<SubmissionStatus>, ApiError> {
        let url = format!(
            "{}/api/v1/submit/submission/?name={name}&format=json",
            self.base_url
        );
        let resp = self.client.get(&url).send().await?;

        if !resp.status().is_success() {
            return Err(ApiError::Api {
                status: resp.status().as_u16(),
                message: resp.text().await.unwrap_or_default(),
            });
        }

        #[derive(Deserialize)]
        struct ListResponse {
            objects: Vec<SubmissionStatus>,
        }

        let list: ListResponse = resp.json().await?;
        Ok(list.objects)
    }

    /// UTILITY: Checks if a document name is currently unused in the Datatracker.
    pub async fn is_name_available(&self, name: &str) -> Result<bool, ApiError> {
        match self.get_draft(name).await {
            Ok(_) => Ok(false),
            Err(ApiError::NotFound(_)) => Ok(true),
            Err(e) => Err(e),
        }
    }
}

impl Default for DataTrackerClient {
    fn default() -> Self {
        Self::new()
    }
}
