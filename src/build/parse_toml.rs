use super::toml_schema::*;
use anyhow::Result;
use std::env;
use std::fs;
use std::path::Path;
use tracing::info;

pub fn get_toml() -> Result<()> {
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
    Ok(())
}
