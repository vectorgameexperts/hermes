use crate::subcommands::Subcommands;
use anyhow::Result;
use clap::Parser;

/// Application cross-platform bundler for bevy apps
#[derive(Parser, Debug)]
#[clap(about, author, version, name = "hermes")]
pub struct Args {
    #[clap(subcommand)]
    action: Subcommands,
    /// Enable verbose logging.
    #[clap(short)]
    pub v: bool,
}

impl Args {
    #[tracing::instrument(level = "trace", skip_all)]
    pub async fn run(self) -> Result<()> {
        match self.action {
            Subcommands::Build(build) => build.run(),
            Subcommands::Clean => todo!("clean"),
            Subcommands::Serve => todo!("serve"),
        }
    }
}
