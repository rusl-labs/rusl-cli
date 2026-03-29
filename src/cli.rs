use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "rusl")]
#[command(about = "The schema package manager", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Install and link schemas defined in the rusl.bundle.toml
    Install(InstallArgs),
}

#[derive(Parser, Debug)]
pub struct InstallArgs {
    // Common install args like --force or --offline can go here later
}
