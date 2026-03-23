// SPDX-License-Identifier: PMPL-1.0-or-later

//! The `fix` command: generate and apply fixes.

use intsoc_fixer::diff;
use intsoc_fixer::engine::FixEngine;
use std::path::Path;

/// Run the fix command.
pub async fn run(
    file: &Path,
    auto_only: bool,
    dry_run: bool,
    output: Option<&Path>,
    _format: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let source = std::fs::read_to_string(file)?;
    let document = intsoc_parser::parse(&source)?;

    tracing::info!("Fixing: {} ({})", document.name, document.stream);

    // Run checks to find issues
    let results = super::check::run_checks_internal(&document);

    // Generate fix plan
    let engine = FixEngine::new();
    let plan = engine.plan(&document, &results);

    let auto_count = plan.auto_safe_fixes().len();
    let rec_count = plan.recommended_fixes().len();
    let manual_count = plan.manual_only_fixes().len();

    println!(
        "Fix plan: {auto_count} auto-safe, {rec_count} recommended, {manual_count} manual-only"
    );

    // Apply fixes
    let fixed_source = if auto_only {
        engine.apply_auto_safe(&source, &plan)?
    } else {
        let all_fixes: Vec<&intsoc_core::fix::Fix> = plan.fixes.iter().collect();
        engine.apply_fixes(&source, &all_fixes)?
    };

    if dry_run {
        // Show diff preview
        let diff_text = diff::unified_diff(&source, &fixed_source, &file.display().to_string());
        if diff_text.is_empty() {
            println!("No changes to apply.");
        } else {
            println!("{diff_text}");
        }
    } else {
        // Write output
        let target = output.unwrap_or(file);
        std::fs::write(target, &fixed_source)?;
        println!("Fixed document written to: {}", target.display());

        let stats = diff::change_count(&source, &fixed_source);
        println!(
            "Changes: +{} -{} (total: {} lines)",
            stats.insertions,
            stats.deletions,
            stats.total()
        );
    }

    Ok(())
}
