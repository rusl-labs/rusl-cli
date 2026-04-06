use crate::config::{self, ConfigTier, GeneratorConfig};
use crate::generate::protocol::{
    GeneratedFile, GenerationRequest, GenerationResponse, SchemaEntry,
};
use crate::manifest::lock::LockManifest;
use anyhow::{Context, Result, bail};
use colored::Colorize;
use std::collections::{HashMap, HashSet, VecDeque};
use std::path::Path;
use std::{env, fs};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::process::Command;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenerateRequestArgs {
    pub name: Option<String>,
    pub list: bool,
    pub print_request: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GenerateOutput {
    Listed(Vec<GeneratorSummary>),
    PrintedRequest(String),
    Generated(GenerateResult),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenerateResult {
    pub generator_name: String,
    pub file_count: usize,
    pub output_dir: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeneratorSummary {
    pub name: String,
    pub is_default: bool,
    pub tier_label: &'static str,
    pub command: String,
    pub output_dir: String,
}

pub async fn run_generate(request: GenerateRequestArgs) -> Result<GenerateOutput> {
    if request.list {
        return list_generators();
    }

    let config = config::load().context("Failed to load configuration")?;
    let cwd = env::current_dir().context("Failed to get current working directory")?;

    let (generator_name, generator_config) = resolve_generator(request.name.as_ref(), &config)?;
    validate_config(&config)?;

    let argv = generator_config
        .command
        .as_ref()
        .with_context(|| format!("Generator '{}' has no command configured.", generator_name))?
        .to_argv();

    if argv.is_empty() {
        bail!("Generator '{}' has an empty command.", generator_name);
    }

    verify_command_exists(&argv[0])?;

    let schemas_dir = cwd.join(config.schema_dir());
    let lock_path = cwd.join("rusl.lock");

    if !schemas_dir.exists() || !lock_path.exists() {
        bail!("No schemas installed. Run `rusl install` first.");
    }

    let lock_str = fs::read_to_string(&lock_path).context("Failed to read rusl.lock")?;
    let lock: LockManifest = toml::from_str(&lock_str).context("Failed to parse rusl.lock")?;

    let (adjacency, dep_meta) = build_graph_from_lock(&lock);
    let all_names: Vec<String> = adjacency.keys().cloned().collect();
    let (target_set, closure) = compute_closure(&all_names, &generator_config.filter, &adjacency)?;
    let sorted = topological_sort(&closure, &adjacency)?;
    let generation_request = build_request(
        &sorted,
        &target_set,
        &dep_meta,
        &schemas_dir,
        generator_config,
    )?;

    if request.print_request {
        let json = serde_json::to_string_pretty(&generation_request)
            .context("Failed to serialize GenerationRequest")?;
        return Ok(GenerateOutput::PrintedRequest(json));
    }

    let response_bytes = spawn_plugin(&argv, &generation_request).await?;
    let response_schema_path = schemas_dir.join("rusl").join("cli-gen-response.json");
    let response = validate_and_parse_response(&response_bytes, &response_schema_path)?;

    for file in &response.files {
        validate_file_path(&file.path)?;
    }

    let output_dir = cwd.join(
        generator_config
            .output_dir
            .as_ref()
            .context("Generator has no output_dir configured")?,
    );
    atomic_write(&output_dir, &response.files, generator_config.clean)?;

    Ok(GenerateOutput::Generated(GenerateResult {
        generator_name,
        file_count: response.files.len(),
        output_dir: generator_config
            .output_dir
            .as_deref()
            .unwrap_or(".")
            .to_string(),
    }))
}

fn list_generators() -> Result<GenerateOutput> {
    let summaries = config::load_generators_with_tiers()?
        .into_iter()
        .filter(|(_, effective)| effective.config.enabled)
        .map(|(name, effective)| GeneratorSummary {
            name,
            is_default: effective.config.default,
            tier_label: match effective.tier {
                ConfigTier::Global => "(global)",
                ConfigTier::Project => "(project)",
            },
            command: effective
                .config
                .command
                .as_ref()
                .map(|command| command.display())
                .unwrap_or_default(),
            output_dir: effective.config.output_dir.unwrap_or_default(),
        })
        .collect::<Vec<_>>();

    Ok(GenerateOutput::Listed(summaries))
}

fn resolve_generator<'a>(
    requested_name: Option<&String>,
    config: &'a config::Config,
) -> Result<(String, &'a GeneratorConfig)> {
    let enabled: HashMap<&String, &GeneratorConfig> = config
        .generators
        .iter()
        .filter(|(_, generator)| generator.enabled)
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

    match requested_name {
        Some(name) => {
            let generator = enabled.get(name).copied().ok_or_else(|| {
                let available = enabled
                    .keys()
                    .map(|item| format!("  {}", item))
                    .collect::<Vec<_>>()
                    .join("\n");
                anyhow::anyhow!(
                    "Generator '{}' not found.\n\nAvailable generators:\n{}",
                    name,
                    available
                )
            })?;
            Ok((name.clone(), generator))
        }
        None => {
            let defaults: Vec<_> = enabled
                .iter()
                .filter(|(_, generator)| generator.default)
                .collect();
            match defaults.len() {
                0 => {
                    let list = enabled
                        .iter()
                        .map(|(name, generator)| {
                            format!(
                                "  {:<16} {}",
                                name,
                                generator
                                    .command
                                    .as_ref()
                                    .map(|command| command.display())
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
                    let (name, generator) = defaults[0];
                    Ok(((*name).clone(), *generator))
                }
                _ => bail!(
                    "Multiple generators are marked as default. At most one may have `default = true`."
                ),
            }
        }
    }
}

fn verify_command_exists(executable: &str) -> Result<()> {
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

struct DepMeta {
    version: String,
    kind: String,
    dependencies: Vec<String>,
}

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
) -> (HashMap<String, Vec<String>>, HashMap<String, DepMeta>) {
    let mut adjacency: HashMap<String, Vec<String>> = HashMap::new();
    let mut dep_meta: HashMap<String, DepMeta> = HashMap::new();

    for (key, dependency) in &lock.dependencies {
        let (kind, name) = parse_lock_key(key);
        let dependency_names: Vec<String> = dependency
            .dependencies
            .iter()
            .map(|item| parse_lock_key(item).1)
            .collect();

        adjacency.insert(name.clone(), dependency_names.clone());
        dep_meta.insert(
            name.clone(),
            DepMeta {
                version: dependency.version.clone(),
                kind,
                dependencies: dependency_names,
            },
        );
    }

    (adjacency, dep_meta)
}

fn compute_closure(
    all_names: &[String],
    filter: &[String],
    adjacency: &HashMap<String, Vec<String>>,
) -> Result<(HashSet<String>, HashSet<String>)> {
    let target_set: HashSet<String> = if filter.is_empty() {
        all_names.iter().cloned().collect()
    } else {
        let patterns: Vec<glob::Pattern> = filter
            .iter()
            .map(|pattern| {
                glob::Pattern::new(pattern)
                    .with_context(|| format!("Invalid filter glob pattern: {}", pattern))
            })
            .collect::<Result<Vec<_>>>()?;

        all_names
            .iter()
            .filter(|name| patterns.iter().any(|pattern| pattern.matches(name)))
            .cloned()
            .collect()
    };

    let mut closure = target_set.clone();
    let mut queue: VecDeque<String> = target_set.iter().cloned().collect();

    while let Some(name) = queue.pop_front() {
        if let Some(dependencies) = adjacency.get(&name) {
            for dependency in dependencies {
                if closure.insert(dependency.clone()) {
                    queue.push_back(dependency.clone());
                }
            }
        }
    }

    Ok((target_set, closure))
}

fn topological_sort(
    closure: &HashSet<String>,
    adjacency: &HashMap<String, Vec<String>>,
) -> Result<Vec<String>> {
    let mut in_degree: HashMap<&String, usize> = HashMap::new();
    for name in closure {
        in_degree.entry(name).or_insert(0);
    }

    for name in closure {
        if let Some(dependencies) = adjacency.get(name) {
            for dependency in dependencies {
                if closure.contains(dependency) {
                    *in_degree.entry(dependency).or_insert(0) += 1;
                }
            }
        }
    }

    let mut queue: VecDeque<&String> = in_degree
        .iter()
        .filter(|(_, degree)| **degree == 0)
        .map(|(name, _)| *name)
        .collect();

    let mut sorted = Vec::new();
    while let Some(node) = queue.pop_front() {
        sorted.push(node.clone());
        if let Some(dependencies) = adjacency.get(node) {
            for dependency in dependencies {
                if let Some(degree) = in_degree.get_mut(dependency) {
                    *degree = degree.saturating_sub(1);
                    if *degree == 0 {
                        queue.push_back(dependency);
                    }
                }
            }
        }
    }

    if sorted.len() != closure.len() {
        bail!("Cycle detected in dependency graph. This should not happen with a valid lockfile.");
    }

    sorted.reverse();
    Ok(sorted)
}

fn build_request(
    sorted: &[String],
    target_set: &HashSet<String>,
    dep_meta: &HashMap<String, DepMeta>,
    schemas_dir: &Path,
    generator_config: &GeneratorConfig,
) -> Result<GenerationRequest> {
    let mut schemas = Vec::new();

    for name in sorted {
        let meta = dep_meta
            .get(name)
            .with_context(|| format!("Missing metadata for schema: {}", name))?;

        let parts: Vec<&str> = name.split('/').collect();
        let schema_path = if parts.len() == 2 {
            schemas_dir
                .join(parts[0])
                .join(format!("{}.json", parts[1]))
        } else {
            schemas_dir.join(format!("{}.json", name))
        };

        let has_schema_file = schema_path.exists();
        let content = if has_schema_file {
            let raw = fs::read_to_string(&schema_path)
                .with_context(|| format!("Failed to read schema file: {:?}", schema_path))?;
            Some(
                serde_json::from_str(&raw)
                    .with_context(|| format!("Invalid JSON in schema file: {:?}", schema_path))?,
            )
        } else {
            None
        };

        let content_ref = if has_schema_file {
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
        options: generator_config.args.clone(),
        schemas,
    })
}

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

    let mut stdin = child
        .stdin
        .take()
        .context("Failed to open generator stdin")?;
    let mut stdout = child
        .stdout
        .take()
        .context("Failed to open generator stdout")?;

    let (write_result, read_result) = tokio::join!(
        async move {
            stdin.write_all(&request_json).await?;
            stdin.shutdown().await?;
            Ok::<_, std::io::Error>(())
        },
        async move {
            let mut buffer = Vec::new();
            stdout.read_to_end(&mut buffer).await?;
            Ok::<_, std::io::Error>(buffer)
        }
    );

    write_result.context("Failed to write request to plugin stdin")?;
    let response_bytes = read_result.context("Failed to read plugin stdout")?;

    let status = child.wait().await.context("Failed to wait for plugin")?;
    match status.code() {
        Some(0) => {}
        Some(2) => {
            bail!("Plugin does not support protocol version 1. Check for updates.");
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

fn validate_and_parse_response(
    response_bytes: &[u8],
    schema_path: &Path,
) -> Result<GenerationResponse> {
    let response_value: serde_json::Value =
        serde_json::from_slice(response_bytes).map_err(|error| {
            let preview =
                String::from_utf8_lossy(&response_bytes[..response_bytes.len().min(2048)]);
            anyhow::anyhow!(
                "Failed to parse generator response: {}\n\n\
             First 2KB of raw output:\n{}\n\n\
             Tip: Use `rusl generate <name> --print-request` to capture the request \
             and test your plugin manually.",
                error,
                preview
            )
        })?;

    if schema_path.exists() {
        let schema_str = fs::read_to_string(schema_path)
            .with_context(|| format!("Failed to read response schema: {:?}", schema_path))?;
        let schema_value: serde_json::Value = serde_json::from_str(&schema_str)
            .with_context(|| format!("Invalid JSON in response schema: {:?}", schema_path))?;

        let validator = jsonschema::validator_for(&schema_value)
            .with_context(|| "Failed to compile response schema")?;
        let errors: Vec<String> = validator
            .iter_errors(&response_value)
            .map(|error| {
                let path = error.instance_path().to_string();
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
                    error
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

    serde_json::from_value(response_value).context("Failed to deserialize generator response")
}

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

fn atomic_write(output_dir: &Path, files: &[GeneratedFile], clean: bool) -> Result<()> {
    let parent = output_dir.parent().unwrap_or_else(|| Path::new("."));
    fs::create_dir_all(parent)
        .with_context(|| format!("Failed to create parent directory: {:?}", parent))?;

    let temp_dir = tempfile::tempdir_in(parent)
        .context("Failed to create temporary directory for atomic write")?;

    for file in files {
        let destination = temp_dir.path().join(&file.path);
        if let Some(file_parent) = destination.parent() {
            fs::create_dir_all(file_parent)?;
        }
        fs::write(&destination, &file.content)
            .with_context(|| format!("Failed to write generated file: {:?}", destination))?;
    }

    if clean && output_dir.exists() {
        let backup = parent.join(format!(".rusl-backup-{}", std::process::id()));
        fs::rename(output_dir, &backup)
            .with_context(|| format!("Failed to back up output directory: {:?}", output_dir))?;

        match fs::rename(temp_dir.path(), output_dir) {
            Ok(()) => {
                let _ = temp_dir.keep();
                let _ = fs::remove_dir_all(&backup);
            }
            Err(_) => {
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

fn validate_config(config: &config::Config) -> Result<()> {
    let enabled: Vec<(&String, &GeneratorConfig)> = config
        .generators
        .iter()
        .filter(|(_, generator)| generator.enabled)
        .collect();

    let default_count = enabled
        .iter()
        .filter(|(_, generator)| generator.default)
        .count();
    if default_count > 1 {
        bail!("Multiple generators are marked as default. At most one may have `default = true`.");
    }

    let mut seen_dirs: HashMap<&str, &str> = HashMap::new();
    for (name, generator) in &enabled {
        if let Some(ref dir) = generator.output_dir {
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

    for (name, generator) in &enabled {
        if let Some(ref dir) = generator.output_dir
            && Path::new(dir).is_absolute()
        {
            bail!(
                "Generator '{}' has an absolute output_dir '{}'. Only relative paths are allowed.",
                name,
                dir
            );
        }
    }

    for (name, generator) in &enabled {
        if generator.plugin_type == "stdio" && generator.command.is_none() {
            bail!(
                "Generator '{}' (type=stdio) has no command configured.",
                name
            );
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{GenerateOutput, GenerateRequestArgs, run_generate, validate_file_path};
    use crate::generate::protocol::GenerationRequest;
    use serial_test::serial;
    use std::{ffi::OsString, path::PathBuf};
    use tempfile::TempDir;

    struct EnvGuard {
        previous_home: Option<OsString>,
        previous_dir: PathBuf,
    }

    impl EnvGuard {
        fn new(home_dir: &std::path::Path, workspace_dir: &std::path::Path) -> Self {
            let previous_home = std::env::var_os(home_var_name());
            let previous_dir = std::env::current_dir().expect("current dir");
            unsafe { std::env::set_var(home_var_name(), home_dir.as_os_str()) };
            std::env::set_current_dir(workspace_dir).expect("set current dir");
            Self {
                previous_home,
                previous_dir,
            }
        }
    }

    impl Drop for EnvGuard {
        fn drop(&mut self) {
            match self.previous_home.as_ref() {
                Some(value) => unsafe { std::env::set_var(home_var_name(), value) },
                None => unsafe { std::env::remove_var(home_var_name()) },
            }
            std::env::set_current_dir(&self.previous_dir).expect("restore current dir");
        }
    }

    #[tokio::test]
    #[serial]
    async fn print_request_marks_only_filtered_schemas_as_targets() {
        let temp_dir = TempDir::new().expect("create temp dir");
        let home_dir = temp_dir.path().join("home");
        let workspace_dir = temp_dir.path().join("workspace");
        std::fs::create_dir_all(&home_dir).expect("create home dir");
        std::fs::create_dir_all(&workspace_dir).expect("create workspace dir");
        let _guard = EnvGuard::new(&home_dir, &workspace_dir);

        let plugin_path = write_plugin_script(
            temp_dir.path(),
            r#"cat >/dev/null
printf '%s' '{"version":"1","files":[]}'
"#,
        );
        write_generate_fixture(
            &workspace_dir,
            &plugin_path,
            Some(r#"filter = ["hassox/root"]"#),
        );

        let output = run_generate(GenerateRequestArgs {
            name: None,
            list: false,
            print_request: true,
        })
        .await
        .expect("print request");

        let GenerateOutput::PrintedRequest(json) = output else {
            panic!("expected printed request");
        };
        let request: GenerationRequest =
            serde_json::from_str(&json).expect("parse printed request");

        assert_eq!(
            request
                .schemas
                .iter()
                .map(|schema| schema.name.as_str())
                .collect::<Vec<_>>(),
            vec!["hassox/dep", "hassox/root"]
        );
        assert!(!request.schemas[0].target);
        assert!(request.schemas[1].target);
        assert_eq!(request.schemas[1].dependencies, vec!["hassox/dep"]);
    }

    #[tokio::test]
    #[serial]
    #[cfg(unix)]
    async fn run_generate_executes_plugin_and_replaces_existing_output() {
        let temp_dir = TempDir::new().expect("create temp dir");
        let home_dir = temp_dir.path().join("home");
        let workspace_dir = temp_dir.path().join("workspace");
        std::fs::create_dir_all(&home_dir).expect("create home dir");
        std::fs::create_dir_all(&workspace_dir).expect("create workspace dir");
        let _guard = EnvGuard::new(&home_dir, &workspace_dir);

        let plugin_path = write_plugin_script(
            temp_dir.path(),
            r#"cat >/dev/null
printf '%s' '{"version":"1","files":[{"path":"types.ts","content":"export const value = 1;\n"}]}'
"#,
        );
        write_generate_fixture(&workspace_dir, &plugin_path, None);
        let output_dir = workspace_dir.join("generated").join("types");
        std::fs::create_dir_all(&output_dir).expect("create output dir");
        std::fs::write(output_dir.join("stale.ts"), "stale").expect("write stale file");

        let output = run_generate(GenerateRequestArgs {
            name: None,
            list: false,
            print_request: false,
        })
        .await
        .expect("run generator");

        let GenerateOutput::Generated(result) = output else {
            panic!("expected generated output");
        };
        assert_eq!(result.generator_name, "typescript");
        assert_eq!(result.file_count, 1);
        assert_eq!(result.output_dir, "generated/types");
        assert_eq!(
            std::fs::read_to_string(output_dir.join("types.ts")).expect("generated file"),
            "export const value = 1;\n"
        );
        assert!(!output_dir.join("stale.ts").exists());
    }

    #[test]
    fn rejects_unsafe_generated_file_paths() {
        let traversal_error =
            validate_file_path("../escape.ts").expect_err("expected traversal error");
        assert!(traversal_error.to_string().contains("Path traversal"));

        let absolute_path = absolute_test_path("escape.ts");
        let absolute_error =
            validate_file_path(&absolute_path).expect_err("expected absolute path error");
        assert!(
            absolute_error
                .to_string()
                .contains("Absolute paths are not allowed")
        );
    }

    fn write_generate_fixture(
        workspace_dir: &std::path::Path,
        plugin_path: &std::path::Path,
        extra_generator_fields: Option<&str>,
    ) {
        std::fs::write(
            workspace_dir.join("rusl.config.toml"),
            format!(
                r#"
[generators.typescript]
type = "stdio"
command = ["{}"]
output_dir = "generated/types"
default = true
{}
"#,
                plugin_path.display(),
                extra_generator_fields.unwrap_or("")
            ),
        )
        .expect("write generator config");

        std::fs::write(
            workspace_dir.join("rusl.lock"),
            r#"
version = "1"

[dependencies."schema:hassox/root"]
version = "1.0.0"
integrity = "root"
source = "https://api.rusl.app"
dependencies = ["schema:hassox/dep"]

[dependencies."schema:hassox/dep"]
version = "1.0.0"
integrity = "dep"
source = "https://api.rusl.app"
"#,
        )
        .expect("write lockfile");

        let schemas_dir = workspace_dir.join(".rusl").join("schemas").join("hassox");
        std::fs::create_dir_all(&schemas_dir).expect("create schemas dir");
        std::fs::write(
            schemas_dir.join("root.json"),
            r#"{"title":"root","type":"object"}"#,
        )
        .expect("write root schema");
        std::fs::write(
            schemas_dir.join("dep.json"),
            r#"{"title":"dep","type":"object"}"#,
        )
        .expect("write dep schema");

        let response_schema_dir = workspace_dir.join(".rusl").join("schemas").join("rusl");
        std::fs::create_dir_all(&response_schema_dir).expect("create response schema dir");
        std::fs::write(
            response_schema_dir.join("cli-gen-response.json"),
            r#"
{
  "type": "object",
  "required": ["version", "files"],
  "properties": {
    "version": { "type": "string" },
    "files": {
      "type": "array",
      "items": {
        "type": "object",
        "required": ["path", "content"],
        "properties": {
          "path": { "type": "string" },
          "content": { "type": "string" }
        }
      }
    }
  }
}
"#,
        )
        .expect("write response schema");
    }

    #[cfg(unix)]
    fn write_plugin_script(dir: &std::path::Path, body: &str) -> PathBuf {
        use std::os::unix::fs::PermissionsExt;

        let path = dir.join("plugin.sh");
        std::fs::write(&path, format!("#!/bin/sh\n{body}")).expect("write plugin script");
        let mut perms = std::fs::metadata(&path)
            .expect("plugin metadata")
            .permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&path, perms).expect("chmod plugin script");
        path
    }

    #[cfg(not(unix))]
    fn write_plugin_script(dir: &std::path::Path, _body: &str) -> PathBuf {
        let path = dir.join("plugin.cmd");
        std::fs::write(
            &path,
            "@echo off\r\npython -c \"import sys; sys.stdin.read(); print('{\\\"version\\\":\\\"1\\\",\\\"files\\\":[]}')\"",
        )
        .expect("write plugin script");
        path
    }

    #[cfg(windows)]
    fn absolute_test_path(name: &str) -> String {
        format!(r"C:\tmp\{name}")
    }

    #[cfg(not(windows))]
    fn absolute_test_path(name: &str) -> String {
        format!("/tmp/{name}")
    }

    #[cfg(windows)]
    fn home_var_name() -> &'static str {
        "USERPROFILE"
    }

    #[cfg(not(windows))]
    fn home_var_name() -> &'static str {
        "HOME"
    }
}
