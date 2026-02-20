// SPDX-License-Identifier: PMPL-1.0-or-later

//! IANA registry API client.

use crate::ApiError;
use serde::{Deserialize, Serialize};

const IANA_BASE: &str = "https://www.iana.org";

/// IANA registry API client.
pub struct IanaClient {
    client: reqwest::Client,
}

/// IANA registry metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryInfo {
    pub id: String,
    pub title: String,
    pub category: String,
    pub updated: Option<String>,
}

impl IanaClient {
    /// Create a new IANA client.
    #[must_use]
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::builder()
                .user_agent("intsoc-transactor/0.1.0")
                .build()
                .expect("failed to build HTTP client"),
        }
    }

    /// Look up a registry by name.
    pub async fn get_registry(&self, registry_id: &str) -> Result<RegistryInfo, ApiError> {
        let url = format!("{IANA_BASE}/assignments/{registry_id}/{registry_id}.xml");
        let resp = self.client.get(&url).send().await?;

        if resp.status() == reqwest::StatusCode::NOT_FOUND {
            return Err(ApiError::NotFound(registry_id.to_string()));
        }

        if !resp.status().is_success() {
            return Err(ApiError::Api {
                status: resp.status().as_u16(),
                message: resp.text().await.unwrap_or_default(),
            });
        }

        // IANA returns XML, parse minimally
        let body = resp.text().await?;

        Ok(RegistryInfo {
            id: registry_id.to_string(),
            title: extract_xml_text(&body, "title").unwrap_or_default(),
            category: extract_xml_text(&body, "category").unwrap_or_default(),
            updated: extract_xml_text(&body, "updated"),
        })
    }
}

impl Default for IanaClient {
    fn default() -> Self {
        Self::new()
    }
}

fn extract_xml_text(xml: &str, tag: &str) -> Option<String> {
    let open = format!("<{tag}>");
    let close = format!("</{tag}>");
    let start = xml.find(&open)?;
    let end = xml.find(&close)?;
    let content = &xml[start + open.len()..end];
    Some(content.trim().to_string())
}
