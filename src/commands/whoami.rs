use crate::cli::WhoamiArgs;
use crate::config;
use crate::registry::client::RegistryClient;
use anyhow::{Context, Result};
use colored::Colorize;

pub async fn run(_args: WhoamiArgs) -> Result<()> {
    let config = config::load().context("Failed to load network configurations")?;
    let client = RegistryClient::new(config);

    let session_data = client
        .fetch_me()
        .await
        .context("Failed to authenticate with registry! Are you sure you are logged in?")?;

    let email = session_data["user"]["email"]
        .as_str()
        .unwrap_or("Unknown Email");
    let slug = session_data["user"]["slug"]
        .as_str()
        .unwrap_or("Unknown Slug");
    let user_id = session_data["user"]["id"].as_str().unwrap_or("Unknown");

    println!("\n    {}", "rusl".truecolor(255, 100, 0).bold());
    println!("  {}", "======================".bright_black());

    println!("  {}:    {}", "Profile".green().bold(), email.white());
    println!("  {}:       {}", "Slug".cyan().bold(), slug.white());
    println!(
        "  {}:         {}",
        "ID".magenta().bold(),
        user_id.bright_black()
    );

    if let Some(accounts_map) = session_data["accounts"].as_object() {
        println!("\n  {}", "Organizations:".yellow().bold());
        for (acc_slug, details) in accounts_map {
            // Only list actual organizations or secondary accounts to avoid redundantly showing their personal user type if preferred,
            // but showing all mapped roles is great for debugging!
            let acc_type = details["type"].as_str().unwrap_or("unknown");
            let role = details["roles"][0].as_str().unwrap_or("MEMBER");

            let bullet = if acc_type == "organization" {
                "⊛"
            } else {
                "◦"
            };
            println!(
                "    {} {} ({})",
                bullet.bright_black(),
                acc_slug.white().bold(),
                role.blue()
            );
        }
        println!(); // Spacing
    } else {
        println!();
    }

    Ok(())
}
