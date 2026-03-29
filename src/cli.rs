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
    /// Add a dependency into the local `rusl.bundle.toml` and instantly install it
    Add(AddArgs),
    /// Remove a dependency from the local `rusl.bundle.toml`
    Remove(RemoveArgs),
    /// Authenticate securely against the backend registry using OAuth2/PKCE callbacks natively
    Login(LoginArgs),
}

#[derive(Parser, Debug)]
pub struct InstallArgs {}

#[derive(Parser, Debug)]
pub struct LoginArgs {}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum DepType {
    Schema,
    Bundle,
}

#[derive(Parser, Debug)]
pub struct AddArgs {
    /// The mathematical category to index the dependency inside
    pub kind: DepType,
    /// The unique package identifier (e.g., rusl/common)
    pub slug: String,
    /// An optional specific version requirement (e.g. >= 1.0.0). Defaults to latest if omitted.
    #[arg(long, short)]
    pub version: Option<String>,
}

#[derive(Parser, Debug)]
pub struct RemoveArgs {
    /// The mathematical category to drop the dependency from
    pub kind: DepType,
    /// The unique package identifier (e.g., rusl/common)
    pub slug: String,
}
