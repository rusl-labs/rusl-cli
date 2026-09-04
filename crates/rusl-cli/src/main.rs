mod cli;
mod commands;
mod ui;

use anyhow::Context;
use clap::Parser;
use cli::{Cli, Commands};
use std::io::IsTerminal;
use std::path::Path;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("rusl=info".parse()?))
        .with_writer(std::io::stderr)
        .init();

    let cli = Cli::parse();
    if let Some(dir) = &cli.cwd {
        apply_cwd(dir)?;
    }

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

/// Behave as if the process had been started in `dir`.
fn apply_cwd(dir: &Path) -> anyhow::Result<()> {
    let process_cwd = std::env::current_dir().context("Failed to get current working directory")?;
    let target = if dir.is_absolute() {
        dir.to_path_buf()
    } else {
        process_cwd.join(dir)
    };

    if !target.exists() {
        anyhow::bail!("Directory does not exist: {}", target.display());
    }
    if !target.is_dir() {
        anyhow::bail!("Not a directory: {}", target.display());
    }

    std::env::set_current_dir(&target)
        .with_context(|| format!("Failed to change working directory to {}", target.display()))?;
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
    use clap::CommandFactory;
    use serial_test::serial;
    use std::path::PathBuf;
    use tempfile::TempDir;

    #[test]
    fn skips_update_check_for_mcp_command() {
        assert!(!should_check_for_update(&Commands::Mcp(McpArgs {})));
    }

    #[test]
    fn checks_for_update_for_logout_command() {
        assert!(should_check_for_update(&Commands::Logout(LogoutArgs {})));
    }

    #[test]
    fn help_includes_global_cwd_flag() {
        let mut help = Vec::new();
        Cli::command().write_help(&mut help).expect("write help");
        let help = String::from_utf8(help).expect("utf8");
        assert!(help.contains("-C, --cwd <DIR>"));
    }

    #[test]
    fn mcp_help_mentions_cwd_flag() {
        let mut mcp = Cli::command()
            .find_subcommand("mcp")
            .expect("mcp subcommand")
            .clone();
        let mut help = Vec::new();
        mcp.write_long_help(&mut help).expect("write mcp help");
        let help = String::from_utf8(help).expect("utf8");
        assert!(
            help.contains("-C") || help.contains("--cwd"),
            "mcp long help should mention -C/--cwd: {help}"
        );
    }

    #[test]
    fn last_cwd_flag_wins() {
        let cli = Cli::try_parse_from(["rusl", "-C", "/tmp/first", "-C", "/tmp/second", "list"])
            .expect("parse");
        assert_eq!(cli.cwd, Some(PathBuf::from("/tmp/second")));
    }

    #[test]
    #[serial]
    fn apply_cwd_accepts_absolute_and_relative_paths() {
        let temp = TempDir::new().expect("temp");
        let nested = temp.path().join("nested");
        std::fs::create_dir_all(&nested).expect("mkdir");
        let previous = std::env::current_dir().expect("cwd");

        apply_cwd(temp.path()).expect("absolute");
        assert_eq!(
            std::env::current_dir()
                .expect("cwd")
                .canonicalize()
                .unwrap(),
            temp.path().canonicalize().unwrap()
        );

        apply_cwd(Path::new("nested")).expect("relative");
        assert_eq!(
            std::env::current_dir()
                .expect("cwd")
                .canonicalize()
                .unwrap(),
            nested.canonicalize().unwrap()
        );

        std::env::set_current_dir(&previous).expect("restore");
    }

    #[test]
    #[serial]
    fn apply_cwd_rejects_missing_and_non_directory_paths() {
        let temp = TempDir::new().expect("temp");
        let previous = std::env::current_dir().expect("cwd");
        let missing = temp.path().join("nope");
        let file = temp.path().join("file.txt");
        std::fs::write(&file, "x").expect("write");

        let err = apply_cwd(&missing).expect_err("missing");
        assert!(format!("{err:#}").contains("does not exist"));

        let err = apply_cwd(&file).expect_err("file");
        assert!(format!("{err:#}").contains("Not a directory"));

        std::env::set_current_dir(&previous).expect("restore");
    }

    #[test]
    #[serial]
    fn apply_cwd_combined_with_bundle_walk_up() {
        let temp = TempDir::new().expect("temp");
        let package = temp.path().join("packages").join("schemas");
        std::fs::create_dir_all(&package).expect("mkdir");
        std::fs::write(
            package.join("rusl.bundle.toml"),
            "[rusl.resources]\n\"acme/schemas/demo\" = \"*\"\n",
        )
        .expect("write package bundle");
        let previous = std::env::current_dir().expect("cwd");

        std::env::set_current_dir(temp.path()).expect("set temp");
        apply_cwd(Path::new("packages/schemas")).expect("apply -C");
        let project = rusl_app::project::discover_bundle_from_cwd().expect("discover");
        assert_eq!(
            project.root.canonicalize().unwrap(),
            package.canonicalize().unwrap()
        );

        std::fs::write(
            temp.path().join("rusl.bundle.toml"),
            "[rusl.resources]\n\"acme/schemas/root\" = \"*\"\n",
        )
        .expect("write root bundle");
        let nested = package.join("nested");
        std::fs::create_dir_all(&nested).expect("mkdir nested");
        std::env::set_current_dir(temp.path()).expect("set temp");
        apply_cwd(Path::new("packages/schemas/nested")).expect("apply -C nested");
        let project = rusl_app::project::discover_bundle_from_cwd().expect("discover");
        assert_eq!(
            project.root.canonicalize().unwrap(),
            package.canonicalize().unwrap()
        );

        std::env::set_current_dir(&previous).expect("restore");
    }
}
