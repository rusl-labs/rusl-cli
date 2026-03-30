use crate::cli::{AddArgs, DepType, InstallArgs};
use crate::commands::install;
use crate::config;
use crate::registry::client::RegistryClient;
use anyhow::{Context, Result};
use colored::Colorize;
use std::fs;
use toml_edit::{DocumentMut, table, value};

pub async fn run(args: AddArgs) -> Result<()> {
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

    let config = config::load().context("Failed to load hierarchical configuration")?;
    let client = RegistryClient::new(config);

    // Calculate the mathematical semantic injection target
    let target_version = match args.version {
        Some(v) => v,
        None => {
            pb.set_message("Finding the latest version...");
            let parts: Vec<&str> = args.slug.split('/').collect();
            if parts.len() != 2 {
                anyhow::bail!("Slug must be in format account/schema-name");
            }

            let meta = match args.kind {
                DepType::Schema => client.fetch_schema_meta(parts[0], parts[1]).await?,
                DepType::Bundle => client.fetch_bundle_meta(parts[0], parts[1]).await?,
            };

            let latest = meta
                .versions
                .first()
                .context("The registry metadata array mathematically contains exactly zero mapped histories for this slug.")?;

            // By standard pubgrub mechanics, assume exact minimum bound logic dynamically
            format!(">={}", latest.version)
        }
    };

    let table_key = match args.kind {
        DepType::Schema => "schemas",
        DepType::Bundle => "bundles",
    };

    // Instantiate table natively if absent
    if doc.get(table_key).is_none() {
        doc[table_key] = table();
    }

    if let Some(table_ref) = doc.get_mut(table_key) {
        if let Some(table) = table_ref.as_table_mut() {
            pb.set_message(format!(
                "Adding {}@{} to [{}]...",
                args.slug, target_version, table_key
            ));
            table.insert(&args.slug, value(target_version));
        } else {
            anyhow::bail!(
                "The [{}] key mathematically exists but is fatally not mapped as a Table element!",
                table_key
            );
        }
    }

    fs::write(&manifest_path, doc.to_string())
        .context("Failed to persist mathematically modified TOML natively")?;

    pb.finish_with_message(format!(
        "{} Added {} to rusl.bundle.toml",
        "Success:".green().bold(),
        args.slug
    ));

    install::run(InstallArgs {}).await?;

    Ok(())
}
