use anyhow::{Context, Result};
use clap::Parser;
use tracing::error;
use tracing_subscriber::prelude::*;

mod args;
mod subcommands;

#[tokio::main]
async fn main() -> Result<()> {
    let args = args::Args::parse();

    // Colorful terminal on windows
    #[cfg(windows)]
    if let Err(err) = ansi_term::enable_ansi_support() {
        eprintln!("error enabling ANSI support: {:?}", err);
    }

    // Filter logs based on the RUST_LOG env var or -v flag.
    let verbose = args.v;
    let log_filter = std::env::var("RUST_LOG").unwrap_or(format!(
        "hermes={}",
        if verbose { "trace" } else { "info" }
    ));
    let log_filter = tracing_subscriber::EnvFilter::new(log_filter);

    // Enable tracing/logging
    tracing_subscriber::registry()
        // Filter spans based on the RUST_LOG env var or -v flag.
        .with(log_filter)
        // Format tracing
        .with(
            tracing_subscriber::fmt::layer()
                .with_target(false)
                .with_level(true)
                .compact(),
        )
        // Install this registry as the global tracing registry.
        .try_init()
        .context("error initializing logging")?;

    if let Err(err) = args.run().await {
        if verbose {
            error!("{err:?}");
        } else {
            error!(
                "{err}\n\nEnable verbose logging (-v) for the full stack trace."
            );
        }
        std::process::exit(1);
    }
    Ok(())
}
