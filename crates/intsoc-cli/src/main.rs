// SPDX-License-Identifier: PMPL-1.0-or-later

//! Internet Society Transactor CLI
//!
//! Usage:
//!   intsoc check <file>       Check a document for issues
//!   intsoc fix <file>         Generate and apply fixes
//!   intsoc submit <file>      Submit to the appropriate stream
//!   intsoc status <name>      Check submission status
//!   intsoc init <name>        Initialize a new draft from template

#![forbid(unsafe_code)]
use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod commands;

#[derive(Parser)]
#[command(
    name = "intsoc",
    version,
    about = "Internet Society document transactor — check, fix, and submit documents across IETF, IRTF, IAB, Independent, IANA, and RFC Editor streams"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Enable verbose output
    #[arg(short, long, global = true)]
    verbose: bool,

    /// Output format (text, json)
    #[arg(short, long, global = true, default_value = "text")]
    format: String,
}

#[derive(Subcommand)]
enum Commands {
    /// Check a document for issues
    Check {
        /// Path to the document file (XML or plain text)
        file: PathBuf,

        /// Only show errors (skip warnings and info)
        #[arg(long)]
        errors_only: bool,
    },

    /// Generate and apply fixes to a document
    Fix {
        /// Path to the document file
        file: PathBuf,

        /// Only apply AutoSafe fixes (no review needed)
        #[arg(long)]
        auto_only: bool,

        /// Preview changes without applying
        #[arg(long)]
        dry_run: bool,

        /// Output fixed document to a different file
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// Submit a document to the appropriate stream
    Submit {
        /// Path to the document file
        file: PathBuf,

        /// Skip pre-submission checks
        #[arg(long)]
        skip_checks: bool,
    },

    /// Check submission status of a draft
    Status {
        /// Draft name (e.g., "draft-jewell-http-430-consent-required-00")
        name: String,
    },

    /// Initialize a new draft from a Nickel template
    Init {
        /// Draft name
        name: String,

        /// Stream type (individual, wg, irtf, iab, independent)
        #[arg(short, long, default_value = "individual")]
        stream: String,

        /// Working group or research group abbreviation
        #[arg(short, long)]
        group: Option<String>,

        /// Output directory
        #[arg(short, long, default_value = ".")]
        dir: PathBuf,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    // Set up tracing
    let filter = if cli.verbose {
        "debug"
    } else {
        "info"
    };
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .init();

    let result = match cli.command {
        Commands::Check { file, errors_only } => {
            commands::check::run(&file, errors_only, &cli.format).await
        }
        Commands::Fix {
            file,
            auto_only,
            dry_run,
            output,
        } => commands::fix::run(&file, auto_only, dry_run, output.as_deref(), &cli.format).await,
        Commands::Submit { file, skip_checks } => {
            commands::submit::run(&file, skip_checks).await
        }
        Commands::Status { name } => commands::status::run(&name, &cli.format).await,
        Commands::Init {
            name,
            stream,
            group,
            dir,
        } => commands::init::run(&name, &stream, group.as_deref(), &dir).await,
    };

    if let Err(e) = result {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
