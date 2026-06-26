use crate::cli::AddArgs;
use anyhow::Result;
use colored::Colorize;
use rusl_app::dependency_service::{self, AddDependencyRequest};
use rusl_app::resolver::graph::ProgressReporter;

pub async fn run(args: AddArgs) -> Result<()> {
    let progress = CliDependencyProgress::new();
    let request = AddDependencyRequest {
        identifier: args.identifier,
        version_requirement: args.version,
    };

    let result = dependency_service::add_dependency(request, &progress).await?;
    progress.finish_with_message(format!(
        "{} Added {} to [{}] and installed {} schemas.",
        "Success:".green().bold(),
        result.slug,
        result.table_key,
        result.install.schema_count
    ));
    Ok(())
}

struct CliDependencyProgress {
    spinner: indicatif::ProgressBar,
}

impl CliDependencyProgress {
    fn new() -> Self {
        Self {
            spinner: crate::ui::spinner("Reading rusl.bundle.toml..."),
        }
    }

    fn finish_with_message(&self, message: String) {
        self.spinner.finish_with_message(message);
    }
}

impl ProgressReporter for CliDependencyProgress {
    fn set_message(&self, message: String) {
        self.spinner.set_message(message);
    }

    fn println(&self, message: String) {
        self.spinner.println(message);
    }
}
