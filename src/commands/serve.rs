use anyhow::Result;
use clap::Args;
use std::env;

use crate::serve::dev_server::start_server;

/// Build your application
#[derive(Debug, Args)]
#[clap(name = "serve")]
pub struct ServeArgs {
    /// Override the version of the application built.
    #[clap(short, long)]
    pub version: Option<String>,
}

impl ServeArgs {
    pub async fn run(&self) -> Result<()> {
        serve(self).await
    }
}

pub async fn serve(args: &ServeArgs) -> Result<()> {
    let current_dir = env::current_dir()?;
    let current_dir_str = current_dir.to_string_lossy().to_string();
    start_server(&current_dir_str).await;
    Ok(())
}
