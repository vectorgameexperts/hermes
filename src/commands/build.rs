use anyhow::Result;
use clap::Args;

use crate::build::parse_toml::get_toml;

/// Build your application
#[derive(Debug, Args)]
#[clap(name = "build")]
pub struct BuildArgs {
    /// Override the version of the application built.
    #[clap(short, long)]
    pub version: Option<String>,
}

impl BuildArgs {
    pub fn run(&self) -> Result<()> {
        build(self)
    }
}

pub fn build(args: &BuildArgs) -> Result<()> {
    // Exit cleanly
    get_toml()
}
