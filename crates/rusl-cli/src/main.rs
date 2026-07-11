mod cli;
mod commands;
mod ui;

use clap::Parser;
use cli::{Cli, Commands};
use std::io::IsTerminal;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("rusl=info".parse()?))
        .with_writer(std::io::stderr)
        .init();

    let cli = Cli::parse();

    maybe_print_update_hint(&cli.command).await;

    match cli.command {
        Commands::Install(args) => commands::install::run(args).await?,
        Commands::Add(args) => commands::add::run(args).await?,
        Commands::Remove(args) => commands::remove::run(args).await?,
        Commands::Login(args) => commands::login::run(args).await?,
        Commands::Logout(args) => commands::logout::run(args).await?,
        Commands::Whoami(args) => commands::whoami::run(args).await?,
        Commands::List(args) => commands::list::run(args).await?,
        Commands::Outdated(args) => commands::outdated::run(args).await?,
        Commands::Search(args) => commands::search::run(*args).await?,
        Commands::Why(args) => commands::why::run(args).await?,
        Commands::Cache(args) => commands::cache::run(args).await?,
        Commands::Mcp(args) => commands::mcp::run(args).await?,
        Commands::Account(args) => commands::account::run(args).await?,
        Commands::Setup(args) => commands::setup::run(args).await?,
    }

    Ok(())
}

async fn maybe_print_update_hint(command: &Commands) {
    if !should_check_for_update(command) || !std::io::stderr().is_terminal() {
        return;
    }

    if let Some(hint) =
        rusl_app::update_check_service::check_for_update(env!("CARGO_PKG_VERSION")).await
    {
        eprintln!("{}", hint.message);
    }
}

fn should_check_for_update(command: &Commands) -> bool {
    !matches!(command, Commands::Mcp(_))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::{LogoutArgs, McpArgs};

    #[test]
    fn skips_update_check_for_mcp_command() {
        assert!(!should_check_for_update(&Commands::Mcp(McpArgs {})));
    }

    #[test]
    fn checks_for_update_for_logout_command() {
        assert!(should_check_for_update(&Commands::Logout(LogoutArgs {})));
    }
}
