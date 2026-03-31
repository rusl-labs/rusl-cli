use crate::cli::GenerateArgs;
use crate::config::{self, ConfigTier, GeneratorConfig};
use crate::generate::protocol::{
    GeneratedFile, GenerationRequest, GenerationResponse, SchemaEntry,
};
use crate::manifest::lock::LockManifest;
use anyhow::{bail, Context, Result};
use colored::Colorize;
use std::collections::{HashMap, HashSet, VecDeque};
use std::path::Path;
use std::{env, fs};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::process::Command;

pub async fn run(args: GenerateArgs) -> Result<()> {
    // --list: show all generators and exit
    if args.list {
        return list_generators();
    }

    let config = config::load().context("Failed to load configuration")?;
    let cwd = env::current_dir().context("Failed to get current working directory")?;

    // Step 1: Resolve generator
    let (gen_name, gen_config) = resolve_generator(&args, &config)?;

    // Validate config
    validate_config(&config)?;

    // Step 2: Verify command exists
    let argv = gen_config
        .command
        .as_ref()
        .context(format!(
            "Generator '{}' has no command configured.",
            gen_name
        ))?
        .to_argv();

    if argv.is_empty() {
        bail!("Generator '{}' has an empty command.", gen_name);
    }

    verify_command_exists(&argv[0])?;

    // Step 3: Verify installation
    let schemas_dir = cwd.join(config.schema_dir());
    let lock_path = cwd.join("rusl.lock");

    if !schemas_dir.exists() || !lock_path.exists() {
        bail!("No schemas installed. Run `rusl install` first.");
    }

    // Step 4: Load dependency graph from lockfile
    let lock_str = fs::read_to_string(&lock_path).context("Failed to read rusl.lock")?;
    let lock: LockManifest = toml::from_str(&lock_str).context("Failed to parse rusl.lock")?;

    let (adjacency, dep_meta) = build_graph_from_lock(&lock);

    // Step 5: Apply filter & compute closure
    let all_names: Vec<String> = adjacency.keys().cloned().collect();
    let (target_set, closure) = compute_closure(&all_names, &gen_config.filter, &adjacency)?;

    // Step 6: Topological sort (leaves first)
    let sorted = topological_sort(&closure, &adjacency)?;

    // Step 7: Build request
    let request = build_request(&sorted, &target_set, &dep_meta, &schemas_dir, &gen_config)?;

    // --print-request: dump JSON and exit
    if args.print_request {
        let json = serde_json::to_string_pretty(&request)
            .context("Failed to serialize GenerationRequest")?;
        println!("{}", json);
        return Ok(());
    }

    // Step 8-9: Spawn plugin and validate response
    let pb = crate::ui::spinner(&format!("Generating with {}...", gen_name));

    let response_bytes = spawn_plugin(&argv, &request).await?;

    // Validate response against the protocol schema if installed
    let response_schema_path = schemas_dir.join("rusl").join("cli-gen-response.json");
    let response = validate_and_parse_response(&response_bytes, &response_schema_path)?;

    // Validate file paths
    for file in &response.files {
        validate_file_path(&file.path)?;
    }

    // Step 10: Atomic writes
    let output_dir = cwd.join(
        gen_config
            .output_dir
            .as_ref()
            .context("Generator has no output_dir configured")?,
    );

    atomic_write(&output_dir, &response.files, gen_config.clean)?;

    // Step 11: Report
    pb.finish_with_message(format!(
        "{} Generated {} files → {}",
        "Success:".green().bold(),
        response.files.len(),
        gen_config.output_dir.as_deref().unwrap_or(".")
    ));

    Ok(())
}

// ---------------------------------------------------------------------------
// Step 1: Resolve generator
// ---------------------------------------------------------------------------

fn resolve_generator<'a>(
    args: &GenerateArgs,
    config: &'a config::Config,
) -> Result<(String, &'a GeneratorConfig)> {
    let enabled: HashMap<&String, &GeneratorConfig> = config
        .generators
        .iter()
        .filter(|(_, g)| g.enabled)
        .collect();

    if enabled.is_empty() {
        bail!(
            "No generators configured.\n\n\
             Add a generator to rusl.config.toml:\n\n\
             [generators.typescript]\n\
             command = \"bunx rusl-gen-typescript\"\n\
             output_dir = \"./generated/types\"\n\
             default = true"
        );
    }

    match &args.name {
        Some(name) => {
            let gcfg = enabled.get(&name).copied().ok_or_else(|| {
                let available = enabled
                    .keys()
                    .map(|k| format!("  {}", k))
                    .collect::<Vec<_>>()
                    .join("\n");
                anyhow::anyhow!(
                    "Generator '{}' not found.\n\nAvailable generators:\n{}",
                    name,
                    available
                )
            })?;
            Ok((name.clone(), gcfg))
        }
        None => {
            // Find the default generator
            let defaults: Vec<_> = enabled
                .iter()
                .filter(|(_, g)| g.default)
                .collect();

            match defaults.len() {
                0 => {
                    let list = enabled
                        .iter()
                        .map(|(name, gcfg)| {
                            format!(
                                "  {:<16} {}",
                                name,
                                gcfg.command
                                    .as_ref()
                                    .map(|c| c.display())
                                    .unwrap_or_default()
                            )
                        })
                        .collect::<Vec<_>>()
                        .join("\n");
                    bail!(
                        "No default generator configured.\n\n\
                         Available generators:\n{}\n\n\
                         Tip: Set `default = true` on a generator in rusl.config.toml, \
                         or specify one: `rusl generate <name>`",
                        list
                    );
                }
                1 => {
                    let (name, gcfg) = defaults[0];
                    Ok(((*name).clone(), *gcfg))
                }
                _ => bail!("Multiple generators are marked as default. At most one may have `default = true`."),
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Step 2: Verify command exists
// ---------------------------------------------------------------------------

fn verify_command_exists(executable: &str) -> Result<()> {
    // Check PATH via which crate, or fall back to checking if it's a relative/absolute path
    if executable.contains('/') || executable.contains('\\') {
        let path = Path::new(executable);
        if !path.exists() {
            bail!(
                "Generator command not found: {}. Is it installed?",
                executable
            );
        }
    } else if which::which(executable).is_err() {
        bail!(
            "Generator command not found: {}. Is it installed?",
            executable
        );
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Step 4: Build graph from lockfile
// ---------------------------------------------------------------------------

/// Metadata extracted from a lock dependency for building the request.
struct DepMeta {
    version: String,
    kind: String,
    /// Direct dependency names (stripped of prefix)
    dependencies: Vec<String>,
}

/// Parse lock key prefix into kind and name.
fn parse_lock_key(key: &str) -> (String, String) {
    if let Some(name) = key.strip_prefix("schema:") {
        ("schema".to_string(), name.to_string())
    } else if let Some(name) = key.strip_prefix("bundle:") {
        ("bundle".to_string(), name.to_string())
    } else if let Some(name) = key.strip_prefix("external:") {
        ("external".to_string(), name.to_string())
    } else {
        ("schema".to_string(), key.to_string())
    }
}

fn build_graph_from_lock(
    lock: &LockManifest,
) -> (
    HashMap<String, Vec<String>>,
    HashMap<String, DepMeta>,
) {
    let mut adjacency: HashMap<String, Vec<String>> = HashMap::new();
    let mut dep_meta: HashMap<String, DepMeta> = HashMap::new();

    for (key, dep) in &lock.dependencies {
        let (kind, name) = parse_lock_key(key);

        let dep_names: Vec<String> = dep
            .dependencies
            .iter()
            .map(|d| parse_lock_key(d).1)
            .collect();

        adjacency.insert(name.clone(), dep_names.clone());
        dep_meta.insert(
            name.clone(),
            DepMeta {
                version: dep.version.clone(),
                kind,
                dependencies: dep_names,
            },
        );
    }

    (adjacency, dep_meta)
}

// ---------------------------------------------------------------------------
// Step 5: Filter & dependency closure
// ---------------------------------------------------------------------------

fn compute_closure(
    all_names: &[String],
    filter: &[String],
    adjacency: &HashMap<String, Vec<String>>,
) -> Result<(HashSet<String>, HashSet<String>)> {
    let target_set: HashSet<String> = if filter.is_empty() {
        // No filter: all schemas are targets
        all_names.iter().cloned().collect()
    } else {
        let patterns: Vec<glob::Pattern> = filter
            .iter()
            .map(|p| {
                glob::Pattern::new(p)
                    .with_context(|| format!("Invalid filter glob pattern: {}", p))
            })
            .collect::<Result<Vec<_>>>()?;

        all_names
            .iter()
            .filter(|name| patterns.iter().any(|p| p.matches(name)))
            .cloned()
            .collect()
    };

    // Walk dependencies from targets to compute transitive closure
    let mut closure = target_set.clone();
    let mut queue: VecDeque<String> = target_set.iter().cloned().collect();

    while let Some(name) = queue.pop_front() {
        if let Some(deps) = adjacency.get(&name) {
            for dep in deps {
                if closure.insert(dep.clone()) {
                    queue.push_back(dep.clone());
                }
            }
        }
    }

    Ok((target_set, closure))
}

// ---------------------------------------------------------------------------
// Step 6: Topological sort (Kahn's algorithm, leaves first)
// ---------------------------------------------------------------------------

fn topological_sort(
    closure: &HashSet<String>,
    adjacency: &HashMap<String, Vec<String>>,
) -> Result<Vec<String>> {
    // Build in-degree map scoped to the closure
    let mut in_degree: HashMap<&String, usize> = HashMap::new();
    for name in closure {
        in_degree.entry(name).or_insert(0);
    }

    // Count incoming edges: for each A -> B (A depends on B), B gets +1 in-degree
    for name in closure {
        if let Some(deps) = adjacency.get(name) {
            for dep in deps {
                if closure.contains(dep) {
                    *in_degree.entry(dep).or_insert(0) += 1;
                }
            }
        }
    }

    // Kahn's: start with zero in-degree nodes (roots — things nothing depends on)
    let mut queue: VecDeque<&String> = in_degree
        .iter()
        .filter(|(_, deg)| **deg == 0)
        .map(|(name, _)| *name)
        .collect();

    let mut sorted: Vec<String> = Vec::new();

    while let Some(node) = queue.pop_front() {
        sorted.push(node.clone());
        if let Some(deps) = adjacency.get(node) {
            for dep in deps {
                if let Some(deg) = in_degree.get_mut(dep) {
                    *deg = deg.saturating_sub(1);
                    if *deg == 0 {
                        queue.push_back(dep);
                    }
                }
            }
        }
    }

    if sorted.len() != closure.len() {
        bail!("Cycle detected in dependency graph. This should not happen with a valid lockfile.");
    }

    // Kahn's gives us roots-first (things with no dependents first).
    // We need leaves-first (things with no dependencies first).
    sorted.reverse();
    Ok(sorted)
}

// ---------------------------------------------------------------------------
// Step 7: Build request
// ---------------------------------------------------------------------------

fn build_request(
    sorted: &[String],
    target_set: &HashSet<String>,
    dep_meta: &HashMap<String, DepMeta>,
    schemas_dir: &Path,
    gen_config: &GeneratorConfig,
) -> Result<GenerationRequest> {
    let mut schemas = Vec::new();

    for name in sorted {
        let meta = dep_meta
            .get(name)
            .with_context(|| format!("Missing metadata for schema: {}", name))?;

        // Build path: .rusl/schemas/<account>/<slug>.json
        let parts: Vec<&str> = name.split('/').collect();
        let schema_path = if parts.len() == 2 {
            schemas_dir.join(parts[0]).join(format!("{}.json", parts[1]))
        } else {
            schemas_dir.join(format!("{}.json", name))
        };

        let content = if schema_path.exists() {
            let raw = fs::read_to_string(&schema_path)
                .with_context(|| format!("Failed to read schema file: {:?}", schema_path))?;
            Some(
                serde_json::from_str(&raw)
                    .with_context(|| format!("Invalid JSON in schema file: {:?}", schema_path))?,
            )
        } else {
            None
        };

        let content_ref = if schema_path.exists() {
            // Relative path from project root
            Some(format!(".rusl/schemas/{}.json", name))
        } else {
            None
        };

        schemas.push(SchemaEntry {
            name: name.clone(),
            version: meta.version.clone(),
            kind: meta.kind.clone(),
            target: target_set.contains(name),
            content,
            content_ref,
            dependencies: meta.dependencies.clone(),
        });
    }

    Ok(GenerationRequest {
        version: "1".to_string(),
        options: gen_config.args.clone(),
        schemas,
    })
}

// ---------------------------------------------------------------------------
// Step 8-9: Spawn plugin & validate response
// ---------------------------------------------------------------------------

async fn spawn_plugin(argv: &[String], request: &GenerationRequest) -> Result<Vec<u8>> {
    let request_json =
        serde_json::to_vec(request).context("Failed to serialize GenerationRequest")?;

    let mut child = Command::new(&argv[0])
        .args(&argv[1..])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::inherit())
        .spawn()
        .with_context(|| format!("Failed to spawn generator: {}", argv[0]))?;

    let mut stdin = child.stdin.take().unwrap();
    let mut stdout = child.stdout.take().unwrap();

    // Concurrent read/write to prevent pipe deadlock
    let (write_result, read_result) = tokio::join!(
        async move {
            stdin.write_all(&request_json).await?;
            stdin.shutdown().await?;
            Ok::<_, std::io::Error>(())
        },
        async move {
            let mut buf = Vec::new();
            stdout.read_to_end(&mut buf).await?;
            Ok::<_, std::io::Error>(buf)
        }
    );

    write_result.context("Failed to write request to plugin stdin")?;
    let response_bytes = read_result.context("Failed to read plugin stdout")?;

    let status = child.wait().await.context("Failed to wait for plugin")?;

    match status.code() {
        Some(0) => {}
        Some(2) => {
            bail!(
                "Plugin does not support protocol version 1. Check for updates."
            );
        }
        Some(code) => {
            bail!("Generator exited with code {}.", code);
        }
        None => {
            bail!("Generator was terminated by a signal.");
        }
    }

    Ok(response_bytes)
}

// ---------------------------------------------------------------------------
// Step 9: Validate response against protocol schema and deserialize
// ---------------------------------------------------------------------------

fn validate_and_parse_response(
    response_bytes: &[u8],
    schema_path: &Path,
) -> Result<GenerationResponse> {
    // Parse raw bytes into a JSON value first
    let response_value: serde_json::Value =
        serde_json::from_slice(response_bytes).map_err(|e| {
            let preview =
                String::from_utf8_lossy(&response_bytes[..response_bytes.len().min(2048)]);
            anyhow::anyhow!(
                "Failed to parse generator response: {}\n\n\
                 First 2KB of raw output:\n{}\n\n\
                 Tip: Use `rusl generate <name> --print-request` to capture the request \
                 and test your plugin manually.",
                e,
                preview
            )
        })?;

    // Validate against the protocol schema if installed
    if schema_path.exists() {
        let schema_str = fs::read_to_string(schema_path)
            .with_context(|| format!("Failed to read response schema: {:?}", schema_path))?;
        let schema_value: serde_json::Value = serde_json::from_str(&schema_str)
            .with_context(|| format!("Invalid JSON in response schema: {:?}", schema_path))?;

        let validator = jsonschema::validator_for(&schema_value)
            .with_context(|| "Failed to compile response schema")?;

        let errors: Vec<String> = validator
            .iter_errors(&response_value)
            .map(|e| {
                let path = e.instance_path().to_string();
                let location = if path.is_empty() {
                    "(root)".to_string()
                } else {
                    path
                };
                format!(
                    "  {} {} {}\n       {}",
                    "✗".red(),
                    location.bold(),
                    "—".dimmed(),
                    e
                )
            })
            .collect();

        if !errors.is_empty() {
            bail!(
                "{} Generator response failed schema validation ({} {}):\n\n{}\n\n  {} Use {} to capture the request and test your plugin manually.",
                "Error:".red().bold(),
                errors.len(),
                if errors.len() == 1 { "error" } else { "errors" },
                errors.join("\n\n"),
                "Tip:".cyan().bold(),
                "`rusl generate <name> --print-request | <plugin>`".dimmed()
            );
        }
    }

    // Deserialize into the typed struct
    serde_json::from_value(response_value).context("Failed to deserialize generator response")
}

// ---------------------------------------------------------------------------
// Step 9 (cont): Path safety
// ---------------------------------------------------------------------------

fn validate_file_path(path: &str) -> Result<()> {
    if Path::new(path).is_absolute() {
        bail!(
            "Unsafe file path from generator: '{}'. Absolute paths are not allowed.",
            path
        );
    }

    for component in Path::new(path).components() {
        if let std::path::Component::ParentDir = component {
            bail!(
                "Unsafe file path from generator: '{}'. Path traversal (..) is not allowed.",
                path
            );
        }
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// Step 10: Atomic writes
// ---------------------------------------------------------------------------

fn atomic_write(output_dir: &Path, files: &[GeneratedFile], clean: bool) -> Result<()> {
    let parent = output_dir
        .parent()
        .unwrap_or_else(|| Path::new("."));

    // Ensure parent exists
    fs::create_dir_all(parent)
        .with_context(|| format!("Failed to create parent directory: {:?}", parent))?;

    // Create temp dir as sibling for same-filesystem atomic rename
    let temp_dir = tempfile::tempdir_in(parent)
        .context("Failed to create temporary directory for atomic write")?;

    for file in files {
        let dest = temp_dir.path().join(&file.path);
        if let Some(file_parent) = dest.parent() {
            fs::create_dir_all(file_parent)?;
        }
        fs::write(&dest, &file.content)
            .with_context(|| format!("Failed to write generated file: {:?}", dest))?;
    }

    if clean && output_dir.exists() {
        // Move old output aside first so a crash never leaves us with nothing.
        // The backup is a sibling dir — same filesystem for atomic rename.
        let backup = parent.join(format!(
            ".rusl-backup-{}",
            std::process::id()
        ));
        fs::rename(output_dir, &backup)
            .with_context(|| format!("Failed to back up output directory: {:?}", output_dir))?;

        // Swap new output into place
        match fs::rename(temp_dir.path(), output_dir) {
            Ok(()) => {
                let _ = temp_dir.keep();
                // Clean up backup only after successful swap
                let _ = fs::remove_dir_all(&backup);
            }
            Err(_) => {
                // Cross-device fallback: copy then cleanup
                copy_dir_recursive(temp_dir.path(), output_dir)?;
                let _ = fs::remove_dir_all(&backup);
            }
        }
    } else {
        if !clean {
            fs::create_dir_all(output_dir)?;
        }

        match fs::rename(temp_dir.path(), output_dir) {
            Ok(()) => {
                let _ = temp_dir.keep();
            }
            Err(_) => {
                copy_dir_recursive(temp_dir.path(), output_dir)?;
            }
        }
    }

    Ok(())
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        if src_path.is_dir() {
            copy_dir_recursive(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// --list
// ---------------------------------------------------------------------------

fn list_generators() -> Result<()> {
    let generators = config::load_generators_with_tiers()?;

    if generators.is_empty() {
        println!("{}", "No generators configured.".yellow());
        return Ok(());
    }

    println!("{}", "Generators:".bold());
    for (name, eff) in &generators {
        if !eff.config.enabled {
            continue;
        }
        let default_marker = if eff.config.default { " *" } else { "" };
        let tier = match eff.tier {
            ConfigTier::Global => "(global)",
            ConfigTier::Project => "(project)",
        };
        let cmd = eff
            .config
            .command
            .as_ref()
            .map(|c| c.display())
            .unwrap_or_default();
        let output = eff
            .config
            .output_dir
            .as_deref()
            .unwrap_or("");

        println!(
            "  {}{:<4} {:<10} {:<36} → {}",
            name.bold(),
            default_marker.yellow(),
            tier.dimmed(),
            cmd,
            output.cyan()
        );
    }

    println!();
    println!("{}", "* = default".dimmed());

    Ok(())
}

// ---------------------------------------------------------------------------
// Config validation
// ---------------------------------------------------------------------------

fn validate_config(config: &config::Config) -> Result<()> {
    let enabled: Vec<(&String, &GeneratorConfig)> = config
        .generators
        .iter()
        .filter(|(_, g)| g.enabled)
        .collect();

    // At most one default
    let default_count = enabled.iter().filter(|(_, g)| g.default).count();
    if default_count > 1 {
        bail!("Multiple generators are marked as default. At most one may have `default = true`.");
    }

    // output_dir must be unique across enabled generators
    let mut seen_dirs: HashMap<&str, &str> = HashMap::new();
    for (name, gcfg) in &enabled {
        if let Some(ref dir) = gcfg.output_dir {
            if let Some(other) = seen_dirs.get(dir.as_str()) {
                bail!(
                    "Generators '{}' and '{}' share the same output_dir '{}'. \
                     Each generator must write to a unique directory.",
                    other,
                    name,
                    dir
                );
            }
            seen_dirs.insert(dir, name);
        }
    }

    // output_dir must be relative
    for (name, gcfg) in &enabled {
        if let Some(ref dir) = gcfg.output_dir {
            if Path::new(dir).is_absolute() {
                bail!(
                    "Generator '{}' has an absolute output_dir '{}'. Only relative paths are allowed.",
                    name,
                    dir
                );
            }
        }
    }

    // command is required for stdio generators
    for (name, gcfg) in &enabled {
        if gcfg.plugin_type == "stdio" && gcfg.command.is_none() {
            bail!(
                "Generator '{}' (type=stdio) has no command configured.",
                name
            );
        }
    }

    Ok(())
}
