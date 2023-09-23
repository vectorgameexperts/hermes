use std::env;

use anyhow::Result;
use clap::Args;

use crate::build::pack_wasm::pack;
use crate::build::parse_toml::get_toml_data;

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
    let current_dir = env::current_dir()?;
    let current_dir_str = current_dir.to_string_lossy().to_string();
    // get toml meta data
    get_toml_data(&current_dir_str).unwrap();

    Ok(pack())
}
