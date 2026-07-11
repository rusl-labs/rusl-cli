use crate::cli::{SetupArgs, SetupCommand};
use anyhow::{Result, bail};
use colored::Colorize;
use dialoguer::{MultiSelect, Select, theme::ColorfulTheme};
use rusl_app::setup::{
    HarnessId, InstallAction, McpMergeAction, PackSourceKind, SetupRequest, SetupScope,
    detect_harnesses, doctor, setup, status,
};
use std::io::IsTerminal;

pub async fn run(args: SetupArgs) -> Result<()> {
    match args.command {
        Some(SetupCommand::Doctor) => run_doctor(args.pack_dir).await,
        Some(SetupCommand::Status) => run_status().await,
        Some(SetupCommand::Update { tag }) => {
            run_setup(SetupArgs {
                command: None,
                targets: vec![],
                all: true,
                yes: true,
                pack_dir: args.pack_dir,
                tag: tag.or(args.tag),
                project: args.project,
                global: args.global,
                skills_only: args.skills_only,
                mcp_only: args.mcp_only,
                force: true,
            })
            .await
        }
        None => run_setup(args).await,
    }
}

struct SetupChoices {
    targets: Vec<String>,
    all: bool,
    scope: SetupScope,
}

async fn run_setup(args: SetupArgs) -> Result<()> {
    let choices = resolve_setup_choices(&args)?;

    let progress = crate::ui::spinner("Resolving agent skills pack...");

    let request = SetupRequest {
        targets: choices.targets,
        all: choices.all,
        pack_dir: args.pack_dir,
        tag: args.tag,
        scope: choices.scope,
        skills_only: args.skills_only,
        mcp_only: args.mcp_only,
        force: args.force,
        allow_clone: true,
    };

    let result = setup(request)?;
    progress.finish_and_clear();

    let scope_label = match choices.scope {
        SetupScope::Project => "project",
        SetupScope::Global => "global",
    };

    println!(
        "{} agent skills (v{}, {scope_label})…",
        "Installing".green().bold(),
        result.pack.manifest.version
    );
    println!("  pack → {}", format_pack_line(&result.pack));

    for report in &result.harnesses {
        let name = report.harness.as_str();
        if let Some(skills) = &report.skills {
            let names: Vec<_> = skills
                .skills
                .iter()
                .map(|s| s.install_name.as_str())
                .collect();
            let action_note = skills
                .skills
                .iter()
                .find(|s| s.action == InstallAction::SkippedExisting)
                .map(|_| " (some already present; use --force to replace)")
                .unwrap_or("");
            println!(
                "  {}  {} skills → {}{}",
                name,
                "✓".green(),
                report.skills_root.display(),
                action_note
            );
            if !names.is_empty() {
                println!("           {}", names.join(" "));
            }
        }
        if let Some(mcp) = &report.mcp {
            let verb = match mcp.action {
                McpMergeAction::Created => "wrote",
                McpMergeAction::Updated => "merged",
                McpMergeAction::Unchanged => "ok",
            };
            println!(
                "  {}  {} mcp    → {} ({verb})",
                name,
                "✓".green(),
                mcp.path.display()
            );
        } else if let Some(note) = &report.mcp_note {
            println!("  {}  {} mcp    — {}", name, "·".dimmed(), note);
        }
        if let Some(sandbox) = &report.sandbox {
            let verb = match sandbox.action {
                McpMergeAction::Created => "wrote",
                McpMergeAction::Updated => "merged",
                McpMergeAction::Unchanged => "ok",
            };
            println!(
                "  {}  {} sandbox → {} ({verb})",
                name,
                "✓".green(),
                sandbox.path.display()
            );
            if !sandbox.paths_added.is_empty() {
                println!("           + paths: {}", sandbox.paths_added.join(", "));
            }
            if !sandbox.network_added.is_empty() {
                println!("           + network: {}", sandbox.network_added.join(", "));
            }
        }
        if let Some(hint) = &report.plugin_hint {
            println!("  {}  {} for hooks: {}", name, "→".cyan(), hint);
        }
    }

    println!();
    println!(
        "Next: open your project and run skill {} (or ask the agent to adopt Rusl).",
        "rusl-init".bold()
    );
    println!("     {}   # verify", "rusl setup doctor".dimmed());
    let _ = result.lock_path;
    Ok(())
}

/// Interactive selector when on a TTY with no explicit targets/flags.
///
/// Space toggles, enter confirms. Non-interactive / `-y` / explicit targets skip the prompt.
fn resolve_setup_choices(args: &SetupArgs) -> Result<SetupChoices> {
    let interactive = should_prompt(args);

    let (targets, all) = if args.all {
        (vec![], true)
    } else if !args.targets.is_empty() {
        (args.targets.clone(), false)
    } else if interactive {
        (prompt_harnesses()?, false)
    } else {
        // Non-interactive: let the service detect / default.
        (vec![], false)
    };

    let scope = if args.global {
        SetupScope::Global
    } else if args.project {
        SetupScope::Project
    } else if interactive {
        prompt_scope()?
    } else {
        SetupScope::Project
    };

    Ok(SetupChoices {
        targets,
        all,
        scope,
    })
}

fn should_prompt(args: &SetupArgs) -> bool {
    if args.yes || args.all || !args.targets.is_empty() {
        return false;
    }
    // Need a real terminal for both stdin and stdout (dialoguer).
    std::io::stdin().is_terminal() && std::io::stdout().is_terminal()
}

fn prompt_harnesses() -> Result<Vec<String>> {
    let all = HarnessId::all();
    let detected = detect_harnesses();
    let default_set = HarnessId::default_install_set();

    let labels: Vec<String> = all
        .iter()
        .map(|id| {
            let detected_mark = if detected.contains(id) {
                "  (detected)"
            } else {
                ""
            };
            format!("{}{detected_mark}", id.as_str())
        })
        .collect();

    // Pre-check detected agents; if none detected, pre-check the default install set.
    let defaults: Vec<bool> = all
        .iter()
        .map(|id| {
            if detected.is_empty() {
                default_set.contains(id)
            } else {
                detected.contains(id)
            }
        })
        .collect();

    let theme = ColorfulTheme::default();
    println!(
        "{}",
        "Select agents to set up  (space to toggle, enter to confirm)".dimmed()
    );
    let chosen = MultiSelect::with_theme(&theme)
        .with_prompt("Agents")
        .items(&labels)
        .defaults(&defaults)
        .interact()?;

    if chosen.is_empty() {
        bail!(
            "No agents selected. Re-run `rusl setup` and pick at least one, or pass targets (e.g. `rusl setup cursor`)."
        );
    }

    Ok(chosen
        .into_iter()
        .map(|i| all[i].as_str().to_string())
        .collect())
}

fn prompt_scope() -> Result<SetupScope> {
    let theme = ColorfulTheme::default();
    let items = [
        "Project — this directory (.cursor/, .claude/, …)",
        "Global  — user home (~/.cursor/, ~/.claude/, …)",
    ];
    let idx = Select::with_theme(&theme)
        .with_prompt("Install scope")
        .items(items)
        .default(0)
        .interact()?;

    Ok(match idx {
        1 => SetupScope::Global,
        _ => SetupScope::Project,
    })
}

async fn run_doctor(pack_dir: Option<std::path::PathBuf>) -> Result<()> {
    let report = doctor(pack_dir)?;

    match &report.pack {
        Some(pack) => {
            println!(
                "skills pack     {} v{} ({})",
                "✓".green(),
                pack.manifest.version,
                format_pack_line(pack)
            );
        }
        None => {
            println!(
                "skills pack     {} {}",
                "✗".red(),
                report.pack_error.as_deref().unwrap_or("not resolved")
            );
        }
    }

    if let Some(lock) = &report.lock {
        println!(
            "lockfile        {} v{} source={} harnesses={}",
            "✓".green(),
            lock.version,
            lock.source,
            lock.harnesses.join(",")
        );
    } else {
        println!(
            "lockfile        {} not installed yet (run {})",
            "⚠".yellow(),
            "rusl setup".bold()
        );
    }

    for check in &report.harness_checks {
        let name = check.harness.as_str();
        let scope = match check.scope {
            SetupScope::Project => "project",
            SetupScope::Global => "global",
        };
        let label = format!("{name}/{scope}");
        let root = check.skills_root.display();
        if check.skills_missing.is_empty() {
            println!("{label:20} {} skills → {root}", "✓".green());
            println!("                     {}", check.skills_present.join(" "));
        } else if check.skills_present.is_empty() {
            println!("{label:20} {} skills missing under {root}", "✗".red());
        } else {
            println!("{label:20} {} skills partial under {root}", "⚠".yellow());
            println!(
                "                     present: {}  missing: {}",
                check.skills_present.join(" "),
                check.skills_missing.join(" ")
            );
        }
        if let Some(ok) = check.mcp_ok {
            if ok {
                println!(
                    "{label:20} {} mcp → {}",
                    "✓".green(),
                    check
                        .mcp_path
                        .as_ref()
                        .map(|p| p.display().to_string())
                        .unwrap_or_default()
                );
            } else {
                println!(
                    "{label:20} {} mcp missing ({})",
                    "✗".red(),
                    check
                        .mcp_path
                        .as_ref()
                        .map(|p| p.display().to_string())
                        .unwrap_or_default()
                );
            }
        } else if let Some(note) = &check.mcp_note {
            println!("{label:20} {} mcp — {note}", "·".dimmed());
        }
    }

    Ok(())
}

async fn run_status() -> Result<()> {
    match status()? {
        Some(lock) => {
            println!("version:  {}", lock.version);
            println!("source:   {}", lock.source);
            println!("pack:     {}", lock.pack_root);
            if let Some(target) = &lock.symlink_target {
                println!("symlink:  {target}");
            }
            println!("scope:    {}", lock.scope);
            println!("harnesses: {}", lock.harnesses.join(", "));
            println!("installed: {}", lock.installed_at);
        }
        None => {
            println!(
                "No agent skills installed yet. Run {}.",
                "rusl setup".bold()
            );
        }
    }
    Ok(())
}

fn format_pack_line(pack: &rusl_app::setup::ResolvedPack) -> String {
    match pack.source {
        PackSourceKind::Explicit => format!("{} (local --pack-dir)", pack.pack_root.display()),
        PackSourceKind::Cache => {
            if let Some(target) = &pack.symlink_target {
                format!(
                    "{} → {} (local)",
                    pack.kit_root
                        .as_ref()
                        .map(|p| p.display().to_string())
                        .unwrap_or_else(|| pack.pack_root.display().to_string()),
                    target.display()
                )
            } else {
                format!(
                    "{} (cache)",
                    pack.kit_root
                        .as_ref()
                        .map(|p| p.display().to_string())
                        .unwrap_or_else(|| pack.pack_root.display().to_string())
                )
            }
        }
        PackSourceKind::Cloned => format!(
            "{} (cloned {})",
            pack.kit_root
                .as_ref()
                .map(|p| p.display().to_string())
                .unwrap_or_else(|| pack.pack_root.display().to_string()),
            rusl_app::setup::DEFAULT_GITHUB_REPO
        ),
    }
}
