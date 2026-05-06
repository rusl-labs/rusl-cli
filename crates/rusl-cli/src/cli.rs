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
    /// Generate code from resolved schemas using configured plugins
    #[command(long_about = "\
Invokes a configured code generator plugin against the project's resolved \
schemas. Plugins receive schema data on stdin and return generated file \
contents on stdout. rusl controls all file writes.\n\n\
If <name> is provided, the named generator from rusl.config.toml is used. \
If omitted, the generator marked default=true is invoked.\n\n\
CONFIGURATION\n\
    Generators are configured in rusl.config.toml:\n\n\
        [generators.typescript]\n\
        type = \"stdio\"\n\
        command = \"bunx rusl-gen-typescript\"\n\
        output_dir = \"./generated/types\"\n\
        default = true\n\n\
        [generators.typescript.args]\n\
        style = \"interface\"\n\n\
EXAMPLES\n\
    Generate using the default generator:\n\
        $ rusl generate\n\n\
    Generate using a named generator:\n\
        $ rusl generate typescript\n\n\
    List available generators:\n\
        $ rusl generate --list\n\n\
    Debug a plugin by capturing the request:\n\
        $ rusl generate typescript --print-request > request.json\n\
        $ ./my-plugin < request.json")]
    Generate(GenerateArgs),
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
    /// The package to search for (e.g., rusl/common or bundles/hassox/test-bundle)
    pub package: String,
}

#[derive(Parser, Debug)]
pub struct GenerateArgs {
    /// Generator name from rusl.config.toml. If omitted, the default generator is used.
    pub name: Option<String>,

    /// List all available generators across config tiers
    #[arg(long)]
    pub list: bool,

    /// Build the GenerationRequest JSON and print to stdout without spawning the plugin
    #[arg(long)]
    pub print_request: bool,
}

#[derive(Parser, Debug)]
pub struct McpArgs {}
