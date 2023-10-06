use clap::Subcommand;
use hermes::commands::{build::BuildArgs, serve::ServeArgs};

#[derive(Subcommand, Debug)]
pub enum Subcommands {
    Build(BuildArgs),
    Clean,
    Serve(ServeArgs),
}
