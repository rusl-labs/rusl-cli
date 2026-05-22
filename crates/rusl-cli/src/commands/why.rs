use crate::cli::WhyArgs;
use anyhow::Result;
use colored::Colorize;
use rusl_app::why_service::{self, WhyOutput, WhyTreeNode, WhyTreeView};

pub async fn run(args: WhyArgs) -> Result<()> {
    match why_service::load_dependency_paths(&args.package)? {
        WhyOutput::MissingLockfile => {
            println!(
                "{}",
                "No schemas installed. `rusl.lock` not found.".yellow()
            );
        }
        WhyOutput::TargetNotFound { package } => {
            println!(
                "{} Package {} not found in rusl.lock.",
                "Error:".red().bold(),
                package.bold()
            );
        }
        WhyOutput::Unreachable { display_name } => {
            println!(
                "{} {} is in the lockfile but not reachable from root dependencies.",
                "Warning:".yellow().bold(),
                display_name.bold()
            );
        }
        WhyOutput::Tree(tree) => print_tree(&tree),
    }

    Ok(())
}

fn print_tree(tree: &WhyTreeView) {
    let version = tree
        .target_version
        .as_ref()
        .map(|value| format!("@v{value}").cyan().to_string())
        .unwrap_or_default();

    println!(
        "{} {}{}",
        "why:".green().bold(),
        tree.target_display_name.bold(),
        version
    );
    println!();
    println!(
        "{}",
        format!("{}@v{}", tree.root_name, tree.root_version).bold()
    );

    for (index, node) in tree.paths.iter().enumerate() {
        print_search_node(node, "", index + 1 == tree.paths.len());
    }
}

fn print_search_node(node: &WhyTreeNode, prefix: &str, is_last: bool) {
    let connector = if is_last { "└── " } else { "├── " };
    let version = node
        .version
        .as_ref()
        .map(|value| format!("@v{value}").cyan().to_string())
        .unwrap_or_default();

    if node.is_target {
        println!(
            "{}{}{}{}",
            prefix,
            connector.dimmed(),
            node.display_name.bold().underline(),
            version
        );
        return;
    }

    println!(
        "{}{}{}{}",
        prefix,
        connector.dimmed(),
        node.display_name.bold(),
        version
    );

    let child_prefix = format!("{}{}", prefix, if is_last { "    " } else { "│   " });
    for (index, child) in node.children.iter().enumerate() {
        print_search_node(child, &child_prefix, index + 1 == node.children.len());
    }
}
