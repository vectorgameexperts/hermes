use std::io::Write;
use std::{env, fs, io};

use anyhow::{Context, Ok, Result};
use clap::Args;

use crate::build::pack_wasm::pack;
use crate::build::parse_toml::get_toml_data;
use crate::pipelines::html::generate_index_html;

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

    pack(&current_dir_str);
    let index_path = format!("{}/www/index.html", current_dir_str);
    if !fs::metadata(&index_path).is_ok() {
        // Index.html doesn't exist, prompt the user
        print!(
            "index.html does not exist. Do you want to generate it? (Y/n): "
        );
        io::stdout().flush().context("Failed to flush stdout")?;

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .context("Failed to read line")?;

        let input = input.trim().to_lowercase();
        if input == "y" || input == "yes" {
            //Todo get name from project or metadata
            generate_index_html(&current_dir_str, "client");
        } else {
            println!("Skipping generation of index.html.");
        }
    } else {
        println!("index.html already exists. Skipping generation.");
    }
    Ok(())
}
