use crate::cli::ListArgs;
use anyhow::Result;
use colored::Colorize;
use rusl_app::list_service::{self, ListFlatView, ListOutput, ListTreeNode, ListTreeView};

pub async fn run(args: ListArgs) -> Result<()> {
    match list_service::load_dependencies(args.tree)? {
        ListOutput::MissingLockfile => {
            println!(
                "{}",
                "No schemas installed. `rusl.lock` not found.".yellow()
            );
        }
        ListOutput::EmptyLockfile => {
            println!("{}", "No dependencies found in rusl.lock.".yellow());
        }
        ListOutput::Flat(flat) => print_flat_view(&flat),
        ListOutput::Tree(tree) => print_tree_view(&tree),
    }

    Ok(())
}

fn print_flat_view(flat: &ListFlatView) {
    println!("{}", "rusl.lock".bold());

    for (index, item) in flat.items.iter().enumerate() {
        let prefix = if index + 1 == flat.items.len() {
            "└── "
        } else {
            "├── "
        };

        let source = item
            .source
            .as_ref()
            .map(|source| format!("  ({})", source.dimmed()))
            .unwrap_or_default();
        println!(
            "{}{}{}{}{}",
            prefix.dimmed(),
            item.display_name.bold(),
            format!("@v{}", item.version).cyan(),
            dev_marker(item.dev),
            source
        );
    }
}

fn dev_marker(dev: bool) -> String {
    if dev {
        format!(" {}", "(dev)".yellow())
    } else {
        String::new()
    }
}

fn print_tree_view(tree: &ListTreeView) {
    println!(
        "{}",
        format!("{}@v{}", tree.root_name, tree.root_version).bold()
    );

    for (index, node) in tree.dependencies.iter().enumerate() {
        print_tree_node(node, "", index + 1 == tree.dependencies.len());
    }
}

fn print_tree_node(node: &ListTreeNode, prefix: &str, is_last: bool) {
    let connector = if is_last { "└── " } else { "├── " };
    let version = node
        .version
        .as_ref()
        .map(|version| format!("@v{version}").cyan().to_string())
        .unwrap_or_default();

    if node.repeated {
        println!(
            "{}{}{}{}{} {}",
            prefix,
            connector.dimmed(),
            node.display_name.bold(),
            version,
            dev_marker(node.dev),
            "(*)".dimmed()
        );
        return;
    }

    println!(
        "{}{}{}{}{}",
        prefix,
        connector.dimmed(),
        node.display_name.bold(),
        version,
        dev_marker(node.dev)
    );

    let child_prefix = format!("{}{}", prefix, if is_last { "    " } else { "│   " });
    for (index, child) in node.children.iter().enumerate() {
        print_tree_node(child, &child_prefix, index + 1 == node.children.len());
    }
}
