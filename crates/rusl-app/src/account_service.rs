use anyhow::{Context, Result};

pub fn set_acting_account(slug: Option<&str>) -> Result<()> {
    let current_dir = std::env::current_dir().context("Failed to get current directory")?;
    let config_path = current_dir.join("rusl.config.toml");

    let contents = if config_path.exists() {
        std::fs::read_to_string(&config_path)
            .with_context(|| format!("Failed to read {}", config_path.display()))?
    } else {
        String::new()
    };

    let mut doc = contents
        .parse::<toml_edit::DocumentMut>()
        .with_context(|| format!("Failed to parse TOML in {}", config_path.display()))?;

    match slug {
        Some(s) => {
            doc["acting_account"] = toml_edit::value(s);
        }
        None => {
            doc.remove("acting_account");
        }
    }

    std::fs::write(&config_path, doc.to_string())
        .with_context(|| format!("Failed to write {}", config_path.display()))?;

    Ok(())
}
