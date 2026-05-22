use crate::cli::WhoamiArgs;
use anyhow::Result;
use colored::Colorize;
use rusl_app::whoami_service;

pub async fn run(_args: WhoamiArgs) -> Result<()> {
    let profile = whoami_service::load_current_user().await?;

    println!("\n    {}", "rusl".truecolor(255, 100, 0).bold());
    println!("  {}", "======================".bright_black());

    println!(
        "  {}:    {}",
        "Profile".green().bold(),
        profile.email.white()
    );
    println!("  {}:       {}", "Slug".cyan().bold(), profile.slug.white());
    println!(
        "  {}:         {}",
        "ID".magenta().bold(),
        profile.user_id.bright_black()
    );

    if !profile.accounts.is_empty() {
        println!("\n  {}", "Organizations:".yellow().bold());
        for account in profile.accounts {
            let bullet = if account.is_organization() {
                "⊛"
            } else {
                "◦"
            };
            println!(
                "    {} {} ({})",
                bullet.bright_black(),
                account.slug.white().bold(),
                account.role.blue()
            );
        }
        println!();
    } else {
        println!();
    }

    Ok(())
}
