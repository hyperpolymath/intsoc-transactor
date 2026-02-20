// SPDX-License-Identifier: PMPL-1.0-or-later

//! IETF Datatracker API client.
//!
//! API documentation: https://datatracker.ietf.org/api/

use crate::ApiError;
use serde::{Deserialize, Serialize};

const BASE_URL: &str = "https://datatracker.ietf.org";

/// IETF Datatracker API client.
pub struct DataTrackerClient {
    client: reqwest::Client,
    base_url: String,
}

/// Document metadata from the Datatracker.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DraftInfo {
    pub name: String,
    pub title: String,
    pub rev: String,
    pub pages: Option<u32>,
    pub time: String,
    pub expires: Option<String>,
    pub group: Option<GroupInfo>,
    pub stream: Option<String>,
    pub intended_std_level: Option<String>,
}

/// Working group information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupInfo {
    pub acronym: String,
    pub name: String,
    #[serde(rename = "type")]
    pub group_type: String,
}

/// Submission status from the Datatracker.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmissionStatus {
    pub id: u64,
    pub name: String,
    pub rev: String,
    pub state: String,
    pub submission_date: String,
}

impl DataTrackerClient {
    /// Create a new Datatracker client.
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

    /// Look up a draft by name.
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

    /// Get the submission status of a draft.
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

    /// Check if a draft name is available.
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
