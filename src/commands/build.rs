use anyhow::Result;
use clap::Args;
use std::env;
use std::fs;
use std::path::Path;
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
    // Get the current working directory as a String
    let current_dir = env::current_dir()?;
    let current_dir_str = current_dir.to_string_lossy().to_string();
    let workspace_toml = format!("{}/Cargo.toml", current_dir_str);
    info!("Current working directory: {:?}", current_dir_str);

    // Check if the Cargo.toml file exists
    if !Path::new(&workspace_toml).exists() {
        return Err(anyhow::anyhow!(
            "No Cargo.toml found in the current directory"
        ));
    }

    let toml_content = fs::read_to_string(&workspace_toml)?;
    // Find the Cargo.toml file

    #[derive(serde::Deserialize)]
    struct Package {
        version: String,
        metadata: MetaData,
    }

    #[derive(serde::Deserialize)]
    struct MetaData {
        hermes: HermesMetaData,
    }
    #[derive(serde::Deserialize)]
    struct HermesMetaData {
        icon_android: String,
        icon_iphone: String,
        icon_web: String,
        require_webgpu: bool,
    }
    #[derive(serde::Deserialize)]
    struct WorkspaceToml {
        package: Package,
    }
    let toml_file: WorkspaceToml = toml::from_str(&toml_content).unwrap();
    info!(
        "The version in your Cargo.toml package is: {:?}",
        args.version.clone().unwrap_or(toml_file.package.version)
    );
    info!(
        "Found: {:?} as current favicon",
        toml_file.package.metadata.hermes.icon_web
    );
    info!(
        "Found: {:?} as current android icon",
        toml_file.package.metadata.hermes.icon_android
    );
    info!(
        "Found: {:?} as current iphone icon",
        toml_file.package.metadata.hermes.icon_iphone
    );
    info!(
        "Web GPU Enabled? {:?}",
        toml_file.package.metadata.hermes.require_webgpu
    );

    // Exit cleanly
    Ok(())
}
