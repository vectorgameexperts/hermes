use anyhow::Result;
use clap::Args;
use tracing::info;

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
    let file = include_str!("../../Cargo.toml");

    #[derive(serde::Deserialize)]
    struct Package {
        version: String,
    }
    #[derive(serde::Deserialize)]
    struct MyToml {
        package: Package,
    }
    let toml_file: MyToml = toml::from_str(file).unwrap();
    info!(
        "The version of your Cargo.toml package is: {:?}",
        args.version.clone().unwrap_or(toml_file.package.version)
    );

    // Exit cleanly
    Ok(())
}
