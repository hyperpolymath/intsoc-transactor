// SPDX-License-Identifier: PMPL-1.0-or-later

//! The `status` command: check submission status of a draft.

use intsoc_api::datatracker::DataTrackerClient;

/// Run the status command.
pub async fn run(name: &str, format: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = DataTrackerClient::new();

    tracing::info!("Looking up: {name}");

    match client.get_draft(name).await {
        Ok(info) => {
            match format {
                "json" => {
                    println!("{}", serde_json::to_string_pretty(&info)?);
                }
                _ => {
                    println!("Draft: {}", info.name);
                    println!("Title: {}", info.title);
                    println!("Revision: {}", info.rev);
                    if let Some(ref group) = info.group {
                        println!("Group: {} ({})", group.name, group.acronym);
                    }
                    if let Some(ref stream) = info.stream {
                        println!("Stream: {stream}");
                    }
                    if let Some(ref level) = info.intended_std_level {
                        println!("Intended Status: {level}");
                    }
                    if let Some(ref expires) = info.expires {
                        println!("Expires: {expires}");
                    }
                }
            }
        }
        Err(intsoc_api::ApiError::NotFound(_)) => {
            println!("Draft '{name}' not found on Datatracker.");
            println!("It may not have been submitted yet, or the name may be incorrect.");
        }
        Err(e) => return Err(e.into()),
    }

    Ok(())
}
