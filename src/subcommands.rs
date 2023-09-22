use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum Subcommands {
    Build,
    Clean,
    Serve,
}
