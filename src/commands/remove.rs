use crate::cli::{DepType, InstallArgs, RemoveArgs};
use crate::commands::install;
use anyhow::{Context, Result};
use std::fs;
use toml_edit::DocumentMut;
use tracing::info;

pub async fn run(args: RemoveArgs) -> Result<()> {
    let cwd = std::env::current_dir()?;
    let manifest_path = cwd.join("rusl.bundle.toml");

    if !manifest_path.exists() {
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
        info!(
            "Effectively orphaned `{}` physically from the `[{}]` architecture limit map.",
            args.slug, table_key
        );
        fs::write(&manifest_path, doc.to_string())
            .context("Failed to persist mathematically modified TOML natively")?;
        info!(
            "Successfully detached topological dependency locally! Executing topological validation pruning run..."
        );
        install::run(InstallArgs {}).await?;
    } else {
        info!(
            "The target architecture boundary `{}` mathematically wasn't natively linked inside `[{}]`. Skipping deletion map...",
            args.slug, table_key
        );
    }

    Ok(())
}
