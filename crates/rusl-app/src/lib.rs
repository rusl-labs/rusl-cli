#[path = "../../../src/cache/mod.rs"]
pub mod cache;
#[path = "../../../src/cli.rs"]
pub mod cli;
#[path = "../../../src/commands/mod.rs"]
pub mod commands;
#[path = "../../../src/config/mod.rs"]
pub mod config;
pub mod dependency_service;
#[path = "../../../src/generate/mod.rs"]
pub mod generate;
pub mod install_service;
pub mod list_service;
#[path = "../../../src/manifest/mod.rs"]
pub mod manifest;
pub mod outdated_service;
#[path = "../../../src/registry/mod.rs"]
pub mod registry;
#[path = "../../../src/resolver/mod.rs"]
pub mod resolver;
#[path = "../../../src/ui.rs"]
pub mod ui;
pub mod whoami_service;

pub use cli::Cli;

pub async fn run(command: cli::Commands) -> anyhow::Result<()> {
    match command {
        cli::Commands::Install(args) => commands::install::run(args).await?,
        cli::Commands::Add(args) => commands::add::run(args).await?,
        cli::Commands::Remove(args) => commands::remove::run(args).await?,
        cli::Commands::Login(args) => commands::login::run(args).await?,
        cli::Commands::Whoami(args) => commands::whoami::run(args).await?,
        cli::Commands::List(args) => commands::list::run(args).await?,
        cli::Commands::Outdated(args) => commands::outdated::run(args).await?,
        cli::Commands::Why(args) => commands::why::run(args).await?,
        cli::Commands::Generate(args) => commands::generate::run(args).await?,
    }

    Ok(())
}
