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
    /// Install schemas defined in rusl.bundle.toml as portable project files
    Install(InstallArgs),
    /// Add a dependency into the local `rusl.bundle.toml` and instantly install it
    Add(AddArgs),
    /// Remove a dependency from the local `rusl.bundle.toml`
    Remove(RemoveArgs),
    /// Authenticate securely against the backend registry using OAuth2/PKCE callbacks natively
    Login(LoginArgs),
    /// Clear the stored authenticated session from this machine
    Logout(LogoutArgs),
    /// Check the active authenticated session
    Whoami(WhoamiArgs),
    /// List all locally installed dependencies based on the lockfile
    List(ListArgs),
    /// Check for updates to dependencies in the registry
    Outdated(OutdatedArgs),
    /// Search visible registry resources and print JSON
    Search(Box<SearchArgs>),
    /// Explain why a package is installed by showing all dependency paths to it
    Why(WhyArgs),
    /// Manage local and global Rusl cache data
    Cache(CacheArgs),
    /// Start the Rusl MCP server over stdio
    Mcp(McpArgs),
    /// Manage the current acting account for the local directory
    Account(crate::commands::account::AccountArgs),
    /// Install agent skills and MCP for Cursor, Claude Code, and other harnesses
    Setup(SetupArgs),
}

#[derive(Parser, Debug)]
pub struct SetupArgs {
    #[command(subcommand)]
    pub command: Option<SetupCommand>,

    /// Harness targets (cursor, claude, codex, opencode, grok). Default: interactive select (or detect when non-interactive).
    pub targets: Vec<String>,

    /// Install for all known harnesses
    #[arg(long)]
    pub all: bool,

    /// Skip interactive prompts; use detected harnesses / defaults
    #[arg(long, short = 'y')]
    pub yes: bool,

    /// Use a local pack directory (…/rusl-agent-kit/pack) instead of the cache/clone
    #[arg(long = "pack-dir", value_name = "DIR", env = "RUSL_SKILLS_PACK")]
    pub pack_dir: Option<std::path::PathBuf>,

    /// Pin a skills pack git tag (e.g. skills-v0.1.0) on a real clone
    #[arg(long)]
    pub tag: Option<String>,

    /// Install into the current project (e.g. .cursor/skills). Default.
    #[arg(long, group = "setup_scope")]
    pub project: bool,

    /// Install into user-global harness dirs (e.g. ~/.cursor/skills)
    #[arg(long, group = "setup_scope")]
    pub global: bool,

    /// Install skills only (skip MCP merge)
    #[arg(long)]
    pub skills_only: bool,

    /// Merge MCP only (skip skill install)
    #[arg(long)]
    pub mcp_only: bool,

    /// Replace existing rusl skill directories
    #[arg(long)]
    pub force: bool,
}

#[derive(Subcommand, Debug)]
pub enum SetupCommand {
    /// Re-fetch the pack and reinstall (uses --force)
    Update {
        /// Optional skills pack tag to checkout
        #[arg(long)]
        tag: Option<String>,
    },
    /// Show installed pack pin and harnesses
    Status,
    /// Non-mutating health check for skills + MCP
    Doctor,
}

#[derive(Parser, Debug)]
pub struct InstallArgs {}

#[derive(Parser, Debug)]
pub struct LoginArgs {}

#[derive(Parser, Debug)]
pub struct LogoutArgs {}

#[derive(Parser, Debug)]
pub struct WhoamiArgs {}

#[derive(Parser, Debug)]
pub struct AddArgs {
    /// The canonical resource identifier (e.g. acme/schemas/payment or acme/bundles/billing)
    pub identifier: String,
    /// An optional specific version requirement (e.g. >= 1.0.0). Defaults to latest if omitted.
    #[arg(long, short)]
    pub version: Option<String>,
}

#[derive(Parser, Debug)]
pub struct RemoveArgs {
    /// The canonical resource identifier (e.g. acme/schemas/payment or acme/bundles/billing)
    pub identifier: String,
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
pub struct SearchArgs {
    /// Search text. Omit to return all visible results.
    pub query: Option<String>,
    /// Restrict results to resource types. Repeat or comma-separate values.
    #[arg(long = "type", value_enum, value_delimiter = ',')]
    pub types: Vec<SearchType>,
    /// Restrict results to exact canonical resource identifiers. Repeat or comma-separate values.
    #[arg(long = "identifier", value_delimiter = ',')]
    pub identifiers: Vec<String>,
    /// Restrict results to account slugs. Repeat or comma-separate values.
    #[arg(long = "account", value_delimiter = ',')]
    pub account_slugs: Vec<String>,
    /// One-based result page.
    #[arg(long)]
    pub page: Option<i32>,
    /// Results per page. Must be between 1 and 100.
    #[arg(long)]
    pub per_page: Option<i32>,
    /// Response shape. Compact is the default.
    #[arg(long, value_enum)]
    pub view: Option<SearchViewArg>,
    /// Include popularity and discoverability metrics.
    #[arg(long)]
    pub include_metrics: bool,
    /// Restrict global search by discovery profile status.
    #[arg(long, value_enum)]
    pub discovery_profile_status: Option<SearchDiscoveryProfileStatus>,
    /// Restrict global search to resources that have any of these metric names.
    #[arg(long = "metric-name", value_delimiter = ',')]
    pub metric_names: Vec<String>,
    /// Type-specific identifier prefix for schema, bundle, or annotation_type search.
    #[arg(long)]
    pub identifier_prefix: Option<String>,
    /// Type-specific resource lifecycle status for schema, bundle, or annotation_type search.
    #[arg(long, value_enum)]
    pub status: Option<SearchResourceStatus>,
    /// Type-specific current version status for schema or bundle search.
    #[arg(long, value_enum)]
    pub current_version_status: Option<SearchVersionStatus>,
    /// Type-specific JSON root instance type for schema search. Repeat or comma-separate values.
    #[arg(
        long = "current-version-root-instance-type",
        value_enum,
        value_delimiter = ','
    )]
    pub current_version_root_instance_types: Vec<SearchJsonRootInstanceType>,
    /// Type-specific schema format for schema search.
    #[arg(long, value_enum)]
    pub schema_format: Option<SearchSchemaFormat>,
    /// Type-specific sort for bundle search.
    #[arg(long, value_enum)]
    pub bundle_sort: Option<SearchBundleSort>,
    /// Type-specific cardinality for annotation_type search.
    #[arg(long, value_enum)]
    pub annotation_type_cardinality: Option<SearchAnnotationTypeCardinality>,
    /// Type-specific lifecycle status for annotation search.
    #[arg(long, value_enum)]
    pub annotation_status: Option<SearchAnnotationStatus>,
    /// Type-specific sort for annotation search.
    #[arg(long, value_enum)]
    pub annotation_sort: Option<SearchAnnotationSort>,
    /// Restrict annotation search to registered annotation type GUIDs.
    #[arg(long = "annotation-type-guid", value_delimiter = ',')]
    pub annotation_type_guids: Vec<String>,
    /// Restrict annotation search to annotations set by these user GUIDs.
    #[arg(long = "set-by-user-guid", value_delimiter = ',')]
    pub set_by_user_guids: Vec<String>,
    /// Restrict annotation search by annotated subject account slugs.
    #[arg(long = "subject-account", value_delimiter = ',')]
    pub subject_account_slugs: Vec<String>,
    /// Restrict annotation search to annotated subject GUIDs.
    #[arg(long = "subject-guid", value_delimiter = ',')]
    pub subject_guids: Vec<String>,
    /// Restrict annotation search by annotated subject identifier prefix.
    #[arg(long)]
    pub subject_identifier_prefix: Option<String>,
    /// Restrict annotation search by annotated subject types. Repeat or comma-separate values.
    #[arg(long = "subject-type", value_enum, value_delimiter = ',')]
    pub subject_types: Vec<SearchAnnotationSubjectType>,
    /// Restrict annotation search to registered annotation type identifiers.
    #[arg(long = "type-identifier", value_delimiter = ',')]
    pub type_identifiers: Vec<String>,
}

#[derive(clap::ValueEnum, Clone, Copy, Debug)]
pub enum SearchType {
    Schema,
    Bundle,
    #[value(name = "annotation_type", alias = "annotation-type")]
    AnnotationType,
    Annotation,
}

#[derive(clap::ValueEnum, Clone, Copy, Debug)]
pub enum SearchViewArg {
    Compact,
    Full,
}

#[derive(clap::ValueEnum, Clone, Copy, Debug)]
pub enum SearchDiscoveryProfileStatus {
    Pending,
    Ready,
    Failed,
}

#[derive(clap::ValueEnum, Clone, Copy, Debug)]
pub enum SearchResourceStatus {
    Active,
    Archived,
}

#[derive(clap::ValueEnum, Clone, Copy, Debug)]
pub enum SearchVersionStatus {
    Draft,
    Active,
    Deprecated,
    Yanked,
}

#[derive(clap::ValueEnum, Clone, Copy, Debug)]
pub enum SearchJsonRootInstanceType {
    Array,
    Boolean,
    Integer,
    Null,
    Number,
    Object,
    String,
}

#[derive(clap::ValueEnum, Clone, Copy, Debug)]
pub enum SearchSchemaFormat {
    #[value(name = "json_schema", alias = "json-schema")]
    JsonSchema,
}

#[derive(clap::ValueEnum, Clone, Copy, Debug)]
pub enum SearchBundleSort {
    Relevance,
    Dependencies,
}

#[derive(clap::ValueEnum, Clone, Copy, Debug)]
pub enum SearchAnnotationTypeCardinality {
    #[value(
        name = "one_per_subject_per_account",
        alias = "one-per-subject-per-account"
    )]
    OnePerSubjectPerAccount,
    #[value(
        name = "many_per_subject_per_account",
        alias = "many-per-subject-per-account"
    )]
    ManyPerSubjectPerAccount,
}

#[derive(clap::ValueEnum, Clone, Copy, Debug)]
pub enum SearchAnnotationStatus {
    Active,
    Deprecated,
    Revoked,
}

#[derive(clap::ValueEnum, Clone, Copy, Debug)]
pub enum SearchAnnotationSort {
    Relevance,
    Endorsements,
}

#[derive(clap::ValueEnum, Clone, Copy, Debug)]
pub enum SearchAnnotationSubjectType {
    Annotations,
    #[value(name = "bundle_versions", alias = "bundle-versions")]
    BundleVersions,
    Bundles,
    #[value(name = "schema_proposals", alias = "schema-proposals")]
    SchemaProposals,
    #[value(name = "schema_versions", alias = "schema-versions")]
    SchemaVersions,
    Schemas,
}

#[derive(Parser, Debug)]
pub struct WhyArgs {
    /// The package to search for (e.g., rusl/schemas/common or hassox/bundles/test-bundle)
    pub package: String,
}

#[derive(Parser, Debug)]
pub struct CacheArgs {
    /// Remove the global content cache and local installed schema files
    #[arg(long)]
    pub clear: bool,
}

#[derive(Parser, Debug)]
pub struct McpArgs {}
