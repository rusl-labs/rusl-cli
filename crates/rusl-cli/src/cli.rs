use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "rusl")]
#[command(version)]
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
    /// Check the active authenticated session
    Whoami(WhoamiArgs),
    /// List all locally installed dependencies based on the lockfile
    List(ListArgs),
    /// Check for updates to dependencies in the registry
    Outdated(OutdatedArgs),
    /// Explain why a package is installed by showing all dependency paths to it
    Why(WhyArgs),
    /// Manage local and global Rusl cache data
    Cache(CacheArgs),
    /// Start the Rusl MCP server over stdio
    Mcp(McpArgs),
}

#[derive(Parser, Debug)]
pub struct InstallArgs {}

#[derive(Parser, Debug)]
pub struct LoginArgs {}

#[derive(Parser, Debug)]
pub struct WhoamiArgs {}

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

#[derive(Parser, Debug)]
pub struct ListArgs {
    /// Display dependencies as a tree showing the full dependency graph
    #[arg(long)]
    pub tree: bool,
}

#[derive(Parser, Debug)]
pub struct OutdatedArgs {}

#[derive(Parser, Debug)]
pub struct WhyArgs {
    /// The package to search for (e.g., rusl/common or hassox/bundles/test-bundle)
    pub package: String,
}

#[derive(Parser, Debug)]
pub struct CacheArgs {
    /// Remove the global content cache and local linked schema cache
    #[arg(long)]
    pub clear: bool,
}

#[derive(Parser, Debug)]
pub struct McpArgs {}
