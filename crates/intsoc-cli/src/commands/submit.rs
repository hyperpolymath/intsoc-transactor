// SPDX-License-Identifier: PMPL-1.0-or-later

//! The `submit` command: submit a document to the appropriate stream.

use std::path::Path;

/// Run the submit command.
pub async fn run(file: &Path, skip_checks: bool) -> Result<(), Box<dyn std::error::Error>> {
    let source = std::fs::read_to_string(file)?;
    let document = intsoc_parser::parse(&source)?;

    tracing::info!(
        "Preparing submission: {} -> {}",
        document.name,
        document.stream
    );

    if !skip_checks {
        // Run pre-submission checks
        let results = super::check::run_checks_internal(&document);
        let summary = intsoc_core::validation::CheckSummary::from_results(results);

        if !summary.passes() {
            return Err(format!(
                "Pre-submission checks failed with {} error(s). Run 'intsoc check' for details, or use --skip-checks to override.",
                summary.error_count
            ).into());
        }
        println!("Pre-submission checks passed.");
    }

    // Determine submission endpoint
    let org = document.stream.organization();
    if !org.uses_datatracker() {
        return Err(format!(
            "{org} submissions are not yet supported via API. Please submit manually at {}",
            org.datatracker_base()
        )
        .into());
    }

    println!("Submission target: IETF Datatracker");
    println!("Document: {}", document.name);
    println!("Stream: {}", document.stream);
    println!();
    println!("NOTE: Automated submission requires Datatracker API authentication.");
    println!("Please submit manually at: https://datatracker.ietf.org/submit/");
    println!("Automated submission will be available in a future release.");

    Ok(())
}
