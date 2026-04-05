mod cli;
mod commands;
mod ui;

use clap::Parser;
use cli::{Cli, Commands};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("rusl=info".parse()?))
        .init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Install(args) => commands::install::run(args).await?,
        Commands::Add(args) => commands::add::run(args).await?,
        Commands::Remove(args) => commands::remove::run(args).await?,
        Commands::Login(args) => commands::login::run(args).await?,
        Commands::Whoami(args) => commands::whoami::run(args).await?,
        Commands::List(args) => commands::list::run(args).await?,
        Commands::Outdated(args) => commands::outdated::run(args).await?,
        Commands::Why(args) => commands::why::run(args).await?,
        Commands::Generate(args) => commands::generate::run(args).await?,
    }

    Ok(())
}
