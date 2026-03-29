mod cache;
mod cli;
mod commands;
mod config;
mod manifest;
mod registry;
mod resolver;

use clap::Parser;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("rusl=info".parse()?))
        .init();

    let cli = cli::Cli::parse();

    match cli.command {
        cli::Commands::Install(args) => commands::install::run(args).await?,
        cli::Commands::Add(args) => commands::add::run(args).await?,
        cli::Commands::Remove(args) => commands::remove::run(args).await?,
        cli::Commands::Login(args) => commands::login::run(args).await?,
    }

    Ok(())
}
