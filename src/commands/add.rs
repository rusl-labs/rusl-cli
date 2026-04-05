use crate::cli::{AddArgs, DepType};
use crate::dependency_service::{self, AddDependencyRequest, DependencyKind};
use crate::resolver::graph::ProgressReporter;
use anyhow::Result;
use colored::Colorize;

pub async fn run(args: AddArgs) -> Result<()> {
    let progress = CliDependencyProgress::new();
    let request = AddDependencyRequest {
        kind: map_dependency_kind(args.kind),
        slug: args.slug,
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

fn map_dependency_kind(kind: DepType) -> DependencyKind {
    match kind {
        DepType::Schema => DependencyKind::Schema,
        DepType::Bundle => DependencyKind::Bundle,
    }
}
