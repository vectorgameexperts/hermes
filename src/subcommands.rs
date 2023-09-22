use clap::Subcommand;
use hermes::commands::build::BuildArgs;

#[derive(Subcommand, Debug)]
pub enum Subcommands {
    Build(BuildArgs),
    Clean,
    Serve,
}
