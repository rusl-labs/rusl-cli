use crate::cli::{DepType, InstallArgs, RemoveArgs};
use crate::commands::install;
use anyhow::{Context, Result};
use colored::Colorize;
use std::fs;
use toml_edit::DocumentMut;

pub async fn run(args: RemoveArgs) -> Result<()> {
    let pb = crate::ui::spinner("Reading rusl.bundle.toml...");

    let cwd = std::env::current_dir()?;
    let manifest_path = cwd.join("rusl.bundle.toml");

    if !manifest_path.exists() {
        pb.finish_and_clear();
        anyhow::bail!("No rusl.bundle.toml found in current directory! Please create one.");
    }

    let contents = fs::read_to_string(&manifest_path)?;
    let mut doc = contents
        .parse::<DocumentMut>()
        .context("Failed to parse perfectly typed TOML DOM")?;

    let table_key = match args.kind {
        DepType::Schema => "schemas",
        DepType::Bundle => "bundles",
    };

    let removed = if let Some(table_ref) = doc.get_mut(table_key) {
        if let Some(table) = table_ref.as_table_mut() {
            table.remove(&args.slug)
        } else {
            None
        }
    } else {
        None
    };

    if removed.is_some() {
        pb.set_message(format!("Removing {} from [{}]...", args.slug, table_key));
        fs::write(&manifest_path, doc.to_string())
            .context("Failed to persist mathematically modified TOML natively")?;

        pb.finish_with_message(format!(
            "{} Removed {} from rusl.bundle.toml",
            "Success:".green().bold(),
            args.slug
        ));
        install::run(InstallArgs {}).await?;
    } else {
        pb.finish_with_message(format!(
            "{} {} is not in [{}]. Nothing to remove.",
            "Status:".cyan().bold(),
            args.slug,
            table_key
        ));
    }

    Ok(())
}
