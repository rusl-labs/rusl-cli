use crate::cli::CacheArgs;
use anyhow::{Result, bail};
use colored::Colorize;
use rusl_app::cache_service;

pub async fn run(args: CacheArgs) -> Result<()> {
    if !args.clear {
        bail!("No cache action selected. Use `rusl cache --clear`.");
    }

    let progress = crate::ui::spinner("Clearing cache...");
    let result = cache_service::clear_cache().await?;

    let details = match (
        result.global_store_cleared,
        result.local_schema_cache_cleared,
    ) {
        (true, true) => "global store and local schemas",
        (true, false) => "global store",
        (false, true) => "local schemas",
        (false, false) => "nothing to remove",
    };

    progress.finish_with_message(format!(
        "{} Cleared cache ({details}).",
        "Success:".green().bold()
    ));

    Ok(())
}
