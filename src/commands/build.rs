use anyhow::Result;
use clap::Args;
use std::env;
use std::fs;
use std::path::Path;
use tracing::info;

//package field
#[derive(serde::Deserialize)]
struct Package {
    version: String,
    #[serde(default)]
    metadata: Option<MetaData>,
}

//  package.metadata field
#[derive(serde::Deserialize)]
struct MetaData {
    #[serde(default)]
    hermes: Option<HermesMetaData>,
}

// package.metadata.hermes field
#[derive(serde::Deserialize)]
struct HermesMetaData {
    #[serde(default)]
    icon_android: Option<String>,
    #[serde(default)]
    icon_iphone: Option<String>,
    #[serde(default)]
    icon_web: Option<String>,
    #[serde(default)]
    require_webgpu: Option<bool>,
}

// struct for the toml and fields we need
#[derive(serde::Deserialize)]
struct WorkspaceToml {
    package: Package,
}

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

    let toml_file: WorkspaceToml = toml::from_str(&toml_content).unwrap();
    info!(
        "The version in your Cargo.toml package is: {:?}",
        args.version.clone().unwrap_or(toml_file.package.version)
    );
    if let Some(MetaData {
        hermes: Some(hermes),
    }) = &toml_file.package.metadata
    {
        info!(
            "Favicon: {:?}",
            hermes
                .icon_web
                .as_ref()
                .unwrap_or(&"No icon_web found".to_string())
        );
        info!(
            "Android icon: {:?}",
            hermes
                .icon_android
                .as_ref()
                .unwrap_or(&"No icon_android found".to_string())
        );
        info!(
            "Iphone icon: {:?}",
            hermes
                .icon_iphone
                .as_ref()
                .unwrap_or(&"No icon_iphone found".to_string())
        );
        info!(
            "Web GPU Enabled? {:?}",
            hermes.require_webgpu.unwrap_or(false)
        );
    } else {
        info!("No hermes configurations found");
    }

    // Exit cleanly
    Ok(())
}
