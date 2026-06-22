use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use colored::*;
use rusl_app::{account_service, config, whoami_service::load_current_user};

#[derive(Parser, Debug)]
pub struct AccountArgs {
    #[command(subcommand)]
    pub command: AccountCommand,
}

#[derive(Subcommand, Debug)]
pub enum AccountCommand {
    /// List all accounts you are a collaborator on.
    List,
    /// Set the active acting account for this workspace.
    Set {
        /// The slug of the account to act as.
        slug: Option<String>,
    },
}

pub async fn run(args: AccountArgs) -> Result<()> {
    match args.command {
        AccountCommand::List => list_accounts().await,
        AccountCommand::Set { slug } => set_account(slug).await,
    }
}

async fn list_accounts() -> Result<()> {
    let profile = load_current_user().await.context("Failed to load user profile")?;
    let config = config::load().context("Failed to load configurations")?;
    
    let default_slug = profile.slug.clone();
    let acting_slug = config.acting_account.unwrap_or(default_slug.clone());

    println!("Your accounts:\n");

    let mut all_accounts = profile.accounts;
    // ensure the default account is at the top
    if let Some(pos) = all_accounts.iter().position(|a| a.slug == default_slug) {
        let default_acc = all_accounts.remove(pos);
        all_accounts.insert(0, default_acc);
    }

    for account in all_accounts {
        let is_active = account.slug == acting_slug;
        let prefix = if is_active { "=>" } else { "  " };
        
        let slug_display = if is_active {
            account.slug.green().bold()
        } else {
            account.slug.normal()
        };

        let default_tag = if account.slug == default_slug {
            " (default user account)".dimmed()
        } else {
            "".dimmed()
        };

        println!("{} {} [{}] {}", prefix, slug_display, account.role, default_tag);
    }

    Ok(())
}

async fn set_account(slug: Option<String>) -> Result<()> {
    account_service::set_acting_account(slug.as_deref())?;
    
    if let Some(s) = slug {
        println!("{} Acting account set to: {}", "✔".green(), s.bold());
    } else {
        println!("{} Acting account cleared. Using default user account.", "✔".green());
    }
    
    Ok(())
}
