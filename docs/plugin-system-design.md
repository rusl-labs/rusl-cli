# rusl Plugin System — Code Generation Design

## Overview

rusl's plugin system enables code generation from resolved JSON schemas via an extensible, protocol-based architecture. Plugins are external processes that receive schema data on stdin and return file contents on stdout. rusl controls all filesystem writes, providing safety, consistency, and a uniform developer experience.

The design prioritizes:
- **Zero SDK requirement** — any language, any runtime can be a plugin
- **Config-driven** — generators are declared in rusl's existing config hierarchy
- **Cross-platform** — no shell invocation, direct process spawning
- **Dogfooding** — the protocol schemas are published as rusl schemas themselves

## Plugin Types

### Phase 1: stdio plugins (`type = "stdio"`)

protoc-inspired model. rusl spawns a child process, pipes a JSON `GenerationRequest` to stdin, reads a JSON `GenerationResponse` from stdout. stderr streams to the user's terminal for progress/warnings.

### Phase 2: WASM plugins (`type = "wasm"`)

Same protocol, different transport. rusl fetches a WASM module (from URL or local file), instantiates it in a sandbox, and passes the same GenerationRequest/GenerationResponse payloads. Deferred to a future iteration — the protocol is designed to support both.

---

## Configuration

Generators are declared in `rusl.config.toml` using the existing 4-tier config hierarchy (env vars → project config → global config → compile-time defaults). This means a developer can set up their preferred generators globally and every project inherits them, or override per-project.

### Config Schema

```toml
# A generator with all options shown
[generators.<name>]
type = "stdio"                        # "stdio" (default) | "wasm" (future)
command = "bunx rusl-gen-typescript"   # string or array of strings
output_dir = "./generated/types"      # relative to project root, required
default = false                       # at most one generator may be default
enabled = true                        # set false to disable a global generator in this project
clean = true                          # wipe output_dir before generating (default: true)
filter = []                           # glob patterns for schema selection (default: all)

# Opaque key-value bag passed to the plugin as `options`
[generators.<name>.args]
# anything the plugin expects
style = "interface"
strict_nulls = true
```

### Field Details

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `type` | `"stdio"` \| `"wasm"` | `"stdio"` | Plugin transport mechanism |
| `command` | `string` \| `string[]` | required | Executable and arguments. String form is split on whitespace. Array form is used as-is. **Caveat**: string form splits naively on whitespace — if any argument contains spaces, you must use the array form. |
| `output_dir` | `string` | required | Relative path from project root where generated files are written |
| `default` | `bool` | `false` | Whether this generator is invoked by bare `rusl generate` |
| `enabled` | `bool` | `true` | Set to `false` to disable a globally-configured generator in a project config |
| `clean` | `bool` | `true` | Remove all contents of `output_dir` before generating |
| `filter` | `string[]` | `[]` (all) | Glob patterns matching schema names to include as targets |
| `args` | `table` | `{}` | Arbitrary options forwarded to the plugin |

### Config Validation Rules

1. **At most one `default = true`** per config scope (project or global). If a project config defines a default, it overrides the global default.
2. **`output_dir` must be unique** across all effective (enabled) generators. Overlapping output directories are a config error — rusl errors immediately with a clear message. Uniqueness is checked after config merging, not across raw tiers.
3. **`command` is required** for stdio plugins.
4. **`output_dir` must be relative** — absolute paths are rejected.
5. **`enabled = false`** disables a generator entirely. Use this to suppress a globally-configured generator in a specific project without redefining it.

### Config Examples

**Global config** (`~/.config/rusl/config.toml`) — developer's standard generators:

```toml
[generators.typescript]
command = "bunx rusl-gen-typescript"
output_dir = "./generated/ts"
default = true

[generators.typescript.args]
style = "interface"

[generators.python]
command = ["python3", "-m", "rusl_gen_python"]
output_dir = "./generated/python"

[generators.python.args]
pydantic = true
```

**Project config** (`rusl.config.toml`) — project-specific overrides and additions:

```toml
[generators.typescript]
command = "bunx rusl-gen-typescript"
output_dir = "./src/types"
default = true

[generators.typescript.args]
style = "type"
strict_nulls = true

[generators.zod]
command = "bunx rusl-gen-zod"
output_dir = "./src/schemas"

[generators.form-components]
command = "bunx rusl-gen-forms"
output_dir = "./src/components/generated"
filter = ["forms/*", "shared/contact-*"]

[generators.form-components.args]
framework = "react"
component_lib = "shadcn"
```

---

## CLI Interface

### Commands

```
rusl generate                       # invoke the default generator
rusl generate <name>                # invoke a named generator
rusl generate --list                # list available generators with source tier
rusl generate <name> --print-request  # dump GenerationRequest JSON to stdout (no plugin invoked)
```

### `rusl generate`

Invokes the generator marked `default = true`. If no default is configured, errors with:

```
Error: No default generator configured.

Available generators:
  typescript    (global)   bunx rusl-gen-typescript
  zod           (project)  bunx rusl-gen-zod

Tip: Set `default = true` on a generator in rusl.config.toml, or specify one: `rusl generate typescript`
```

### `rusl generate <name>`

Invokes the named generator. If the name doesn't match any configured generator, errors with available options.

### `rusl generate --list`

Shows all generators across config tiers:

```
Generators:
  typescript *     (global)   bunx rusl-gen-typescript     → ./generated/ts
  python           (global)   python3 -m rusl_gen_python   → ./generated/python
  typescript       (project)  bunx rusl-gen-typescript     → ./src/types
  zod              (project)  bunx rusl-gen-zod            → ./src/schemas
  form-components  (project)  bunx rusl-gen-forms          → ./src/components/generated

* = default
```

Project-level configs override global configs of the same name (standard 4-tier precedence).

### `rusl generate <name> --print-request`

Builds the full `GenerationRequest` JSON and prints it to stdout without spawning the plugin. This is the primary tool for plugin developers:

```bash
# Capture the request for local plugin development
rusl generate typescript --print-request > request.json

# Iterate on your plugin without touching rusl config
./my-plugin < request.json | jq .

# Pipe directly for quick testing
rusl generate typescript --print-request | ./my-plugin
```

---

## Protocol Specification

### GenerationRequest (rusl → plugin stdin)

```json
{
  "version": "1",
  "options": {},
  "schemas": []
}
```

| Field | Type | Description |
|-------|------|-------------|
| `version` | `string` | Protocol version. Always `"1"` for now. Plugins should check this. |
| `options` | `object` | Contents of `[generators.<name>.args]` from config. Opaque to rusl. |
| `schemas` | `Schema[]` | Ordered array of schemas. See below. |

### Schema Object

```json
{
  "name": "rusl/common",
  "version": "1.2.0",
  "kind": "schema",
  "target": true,
  "content": {},
  "dependencies": ["rusl/base"]
}
```

| Field | Type | Description |
|-------|------|-------------|
| `name` | `string` | Fully qualified schema name (`account/slug`) |
| `version` | `string` | Exact resolved version |
| `kind` | `"schema"` \| `"bundle"` \| `"external"` | Source type of the dependency |
| `target` | `bool` | `true` if this schema matched the filter, `false` if included as a transitive dependency |
| `content` | `object \| null` | The full JSON Schema content, inline. `null` when `content_ref` is provided instead (see below). |
| `content_ref` | `string \| null` | Relative file path to the schema on disk (e.g., `.rusl/schemas/rusl/common.json`). Provided as a fallback when `content` is `null` for large schemas. Plugins should prefer `content` when present. |
| `dependencies` | `string[]` | Direct dependency names (for graph awareness) |

### Content Delivery Strategy

Both `content` (inline JSON) and `content_ref` (file path) are provided for every schema. For v1, `content` is always populated and `content_ref` is always populated — plugins can use whichever is more convenient. In a future version, rusl may omit `content` (set to `null`) for very large schemas to avoid multi-megabyte request payloads, relying on `content_ref` as the fallback. Plugins should handle both cases.

### Schema Ordering

Schemas are provided in **reverse topological order** (leaves first, root last). This guarantees that when a plugin processes schema N, all schemas that N depends on have already appeared in the array.

Example dependency graph:
```
forms/contact → rusl/address → rusl/base
forms/signup  → rusl/base
```

Resulting order in the request:
```
1. rusl/base        (leaf, no deps)
2. rusl/address     (depends on base, which is already listed)
3. forms/contact    (depends on address, already listed)
4. forms/signup     (depends on base, already listed)
```

### GenerationResponse (plugin stdout → rusl)

```json
{
  "version": "1",
  "files": []
}
```

| Field | Type | Description |
|-------|------|-------------|
| `version` | `string` | Protocol version |
| `files` | `File[]` | Array of files to write |

### File Object

```json
{
  "path": "types/common.ts",
  "content": "export interface Common {\n  name: string;\n}\n"
}
```

| Field | Type | Description |
|-------|------|-------------|
| `path` | `string` | Relative path within `output_dir`. Must not contain `..` or be absolute. |
| `content` | `string` | Full file content |

### Path Safety

rusl validates all file paths in the response:
- Must be relative (no leading `/`)
- Must not contain `..` segments
- Must not escape `output_dir` after resolution
- Violation = generation failure with clear error

### Error Handling

- **stderr**: Streamed to the user's terminal in real time. Plugins use this for progress messages, warnings, and error details.
- **Exit code 1**: Plugin failure. rusl reports the failure and does NOT write any files (atomic — all or nothing).
- **Exit code 2**: Protocol version mismatch. The plugin received a `version` it does not support. rusl reports this specifically: `"Plugin does not support protocol version 1. Check for updates."`. Plugins SHOULD exit 2 with a human-readable message on stderr when they encounter an unknown version.
- **Exit code (other non-zero)**: Treated as plugin failure (same as exit code 1).
- **Invalid JSON**: If stdout is not valid JSON or doesn't match the response schema, rusl reports a parse error with the first 2KB of raw output for debugging, plus a note on how to capture the full output using `--print-request` for reproduction.
- **Timeout**: TBD — may want a configurable timeout for long-running generators.

---

## Execution Flow

### Detailed Step-by-Step

```
rusl generate [name]
│
├─ 1. RESOLVE GENERATOR
│     Parse rusl.config.toml (project → global)
│     Find generator by name, or find default
│     Skip generators with enabled=false
│     Validate config (unique output_dirs, valid paths, etc.)
│
├─ 2. VERIFY COMMAND EXISTS
│     Parse command (string → split on whitespace, array → use as-is)
│     Check that the executable exists on PATH (or is a valid relative path)
│     If not found: error immediately, BEFORE touching any files
│     "Generator command not found: bunx. Is it installed?"
│
├─ 3. VERIFY INSTALLATION
│     Check .rusl/schemas/ directory exists
│     Check rusl.lock exists
│     If not: error "No schemas installed. Run `rusl install` first."
│
├─ 4. LOAD DEPENDENCY GRAPH
│     Parse rusl.lock for resolved versions and integrity hashes
│     Build in-memory dependency graph from lock dependencies
│
├─ 5. APPLY FILTER & COMPUTE CLOSURE
│     If filter is set:
│       Match schema names against glob patterns → "target set"
│       Walk dependency graph from targets → collect transitive deps
│       Mark targets as target=true, deps as target=false
│     If no filter:
│       All schemas are target=true
│
├─ 6. TOPOLOGICAL SORT
│     Sort schemas in reverse topological order (leaves first)
│     Cycle detection (should not happen with valid lock, but safety check)
│
├─ 7. BUILD REQUEST
│     For each schema in sorted order:
│       Read content from .rusl/schemas/<account>/<slug>.json
│       Build Schema object with name, version, kind, target, content,
│         content_ref, dependencies
│     Assemble GenerationRequest with version, options (from args), schemas
│     Serialize to JSON
│
│     [If --print-request: print JSON to stdout and exit here]
│
├─ 8. SPAWN PLUGIN (async, via tokio::process::Command)
│     Spawn child process (no shell):
│       stdin  = piped (we write to it)
│       stdout = piped (we read from it)
│       stderr = inherited (streams to user terminal)
│
│     CRITICAL: Write stdin and read stdout CONCURRENTLY using tokio::join!
│     to avoid pipe deadlock. If rusl writes a large request while the plugin
│     writes a large response, both processes can block on full pipe buffers
│     (~64KB on most OS). Concurrent I/O prevents this.
│
│     tokio::join!(
│       async { write request to stdin; drop stdin to signal EOF },
│       async { read all of stdout into buffer }
│     )
│
│     Wait for process exit.
│
├─ 9. VALIDATE RESPONSE
│     If exit code 2: report protocol version mismatch
│     If exit code non-zero: report failure, do NOT write files, exit
│     Parse stdout buffer as JSON → GenerationResponse
│     If invalid JSON: report parse error with first 2KB of raw output
│     Validate: version field, files array present
│     Validate ALL file paths for safety (no .., no absolute, within output_dir)
│     If any path is unsafe: report error, do NOT write files
│
├─ 10. WRITE FILES (atomic via temp directory)
│      Create a temp directory alongside output_dir (e.g., output_dir.tmp.XXXXX)
│      For each file in response.files:
│        Compute full path: temp_dir / file.path
│        Create parent directories as needed
│        Write file content
│      If clean=true (default):
│        Remove existing output_dir
│      Rename temp_dir → output_dir (atomic on same filesystem)
│      If rename fails (cross-device): copy temp_dir → output_dir, remove temp_dir
│
└─ 11. REPORT
       Print summary: "✓ Generated N files → ./generated/types"
       List files if verbose mode
```

### Atomicity

Generation is all-or-nothing via temp directory staging:
- All generated files are written to a temp directory first
- Only after ALL files are written successfully does the output_dir get replaced
- If the plugin fails (non-zero exit), the temp directory is cleaned up and output_dir is untouched
- If path validation fails on ANY file, nothing is written and output_dir is untouched
- If the machine crashes mid-write, only the temp directory is left behind (orphaned), and the previous output_dir remains intact
- The temp directory is created as a sibling of output_dir (same parent) to maximize the chance of atomic rename on the same filesystem

This means a failed generation **preserves the previous generated output** rather than leaving an empty directory. The previous output may be stale, but stale code that compiles is better than an empty directory that breaks the build.

---

## Schema Filtering & Dependency Closure

### Filter Syntax

Filters are glob patterns matched against schema names (`account/slug`):

```toml
filter = ["forms/*"]                    # all schemas in the forms account
filter = ["rusl/common", "rusl/base"]   # specific schemas
filter = ["forms/*", "shared/contact-*"] # multiple patterns
filter = []                             # no filter = all schemas (default)
```

### Dependency Closure Algorithm

```
input:  filter globs, full dependency graph
output: ordered list of schemas with target flags

1. Match all schema names against filter globs → target_set
2. Initialize closure = target_set
3. For each schema in target_set:
     Walk dependencies recursively (DFS)
     Add all transitive deps to closure
4. For each schema in closure:
     schema.target = (schema ∈ target_set)
5. Topological sort closure (leaves first)
6. Return sorted list
```

### Why Dependency Closure Matters

A generator for `forms/contact` that depends on `rusl/address` needs the address schema to:
- Resolve `$ref` pointers in the JSON Schema
- Generate import statements / type references
- Understand the full shape of nested objects

Without the dependency closure, the plugin would receive a partial view and produce broken output.

---

## Dogfooding: Protocol Schemas on rusl

The GenerationRequest and GenerationResponse are themselves JSON Schemas, published as rusl packages:

```
rusl/gen-request@1.0.0    — JSON Schema for the generation request
rusl/gen-response@1.0.0   — JSON Schema for the generation response
```

### Benefits

1. **Plugin authors** can `rusl add schema rusl/gen-request` and use their own typescript/python/go generator to create typed bindings for the protocol
2. **rusl CLI itself** uses these schemas — the Rust types for the protocol are generated from the schemas using rusl's own codegen (true dogfooding)
3. **Validation** — rusl can validate plugin output against `rusl/gen-response` before writing files
4. **Versioning** — protocol changes are tracked as schema version bumps with semver

### Bootstrap

The initial implementation will hardcode the Rust types. Once the plugin system works, a generator can be written to produce the Rust types from the schemas, replacing the hardcoded versions. This is the classic bootstrap problem — write it by hand first, then use the tool to maintain it.

---

## Command Specification (CommandSpec)

### Deserialization

```rust
#[derive(Deserialize)]
#[serde(untagged)]
enum CommandSpec {
    Simple(String),        // "bunx rusl-gen-typescript"
    Explicit(Vec<String>), // ["python3", "-m", "rusl_gen_python"]
}
```

### Resolution

- **Simple**: Split on whitespace. First element = executable, rest = arguments. **Warning**: this is a naive split — shell quoting rules are NOT applied. If any argument contains spaces (e.g., a path like `"C:\Program Files\gen.exe"`), you must use the array form.
- **Explicit**: First element = executable, rest = arguments. No splitting. Use this for any command with arguments that contain spaces or special characters.

### Execution

All plugins are spawned via `tokio::process::Command` — no shell invocation. The executable is resolved from the system PATH. This works identically on Unix, macOS, and Windows. Using the async variant is critical to avoid pipe deadlocks (see Execution Flow).

```rust
use tokio::process::Command;
use tokio::io::{AsyncWriteExt, AsyncReadExt};

let mut child = Command::new(&args[0])
    .args(&args[1..])
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .stderr(Stdio::inherit())
    .spawn()?;

let mut stdin = child.stdin.take().unwrap();
let mut stdout = child.stdout.take().unwrap();

// Concurrent read/write to prevent pipe deadlock
let (write_result, response_bytes) = tokio::join!(
    async {
        stdin.write_all(&request_json).await?;
        drop(stdin); // close stdin to signal EOF
        Ok::<_, io::Error>(())
    },
    async {
        let mut buf = Vec::new();
        stdout.read_to_end(&mut buf).await?;
        Ok::<_, io::Error>(buf)
    }
);

let status = child.wait().await?;
```

### Examples

```toml
command = "bunx rusl-gen-typescript"
# → Command::new("bunx").arg("rusl-gen-typescript")

command = ["python3", "-m", "rusl_gen_python"]
# → Command::new("python3").arg("-m").arg("rusl_gen_python")

command = "./scripts/generate.sh"
# → Command::new("./scripts/generate.sh")

command = "npx @rusl/gen-zod"
# → Command::new("npx").arg("@rusl/gen-zod")
```

---

## Config Merging Behavior

Generators follow the same precedence as existing config: project overrides global.

### Rules

1. If a generator name exists in both project and global config, the **project definition wins entirely** (no field-level merge). This keeps behavior predictable.
2. `rusl generate --list` shows all generators with their source tier so the user can see what's active.
3. The `default` flag is resolved after merging — the effective default is the one from the highest-priority config tier.

### Example

Global config:
```toml
[generators.typescript]
command = "bunx rusl-gen-typescript"
output_dir = "./generated/ts"
default = true
```

Project config:
```toml
[generators.typescript]
command = "bunx rusl-gen-typescript"
output_dir = "./src/types"
default = true

[generators.typescript.args]
style = "type"
```

Effective config: project's `typescript` wins entirely. `output_dir` is `./src/types`, `args.style` is `"type"`. The global definition is completely shadowed.

### Disabling a Global Generator

A project that doesn't want a globally-configured generator can disable it without redefining:

```toml
# Project config — this is a pure TypeScript project, no Python needed
[generators.python]
enabled = false
```

This suppresses the global `python` generator for this project only. No other fields are needed.

---

## Man Page Documentation

The `rusl generate` command must ship with comprehensive man page documentation. This includes:

### `rusl-generate(1)`

```
NAME
    rusl-generate — generate code from resolved schemas using plugins

SYNOPSIS
    rusl generate [<name>] [--list] [--print-request]

DESCRIPTION
    Invokes a configured code generator plugin against the project's resolved
    schemas. Plugins receive schema data on stdin and return generated file
    contents on stdout. rusl controls all file writes.

    If <name> is provided, the named generator from rusl.config.toml is used.
    If omitted, the generator marked default=true is invoked.

OPTIONS
    --list
        List all available generators across config tiers (project and global)
        with their source, command, and output directory.

    --print-request
        Build the GenerationRequest JSON and print it to stdout without
        spawning the plugin. Useful for plugin development and debugging.

CONFIGURATION
    Generators are configured in rusl.config.toml:

        [generators.typescript]
        type = "stdio"
        command = "bunx rusl-gen-typescript"
        output_dir = "./generated/types"
        default = true

        [generators.typescript.args]
        style = "interface"

    See rusl-config(5) for full configuration reference.

EXIT STATUS
    0   Generation completed successfully.
    1   Generation failed (plugin error, config error, or validation error).

EXAMPLES
    Generate using the default generator:
        $ rusl generate

    Generate using a named generator:
        $ rusl generate typescript

    List available generators:
        $ rusl generate --list

    Debug a plugin by capturing the request:
        $ rusl generate typescript --print-request > request.json
        $ ./my-plugin < request.json

SEE ALSO
    rusl-install(1), rusl-config(5)
```

---

## Future Considerations (Out of Scope for v1)

### WASM Plugins (Phase 2)

```toml
[generators.typescript-wasm]
type = "wasm"
source = "https://registry.rusl.app/plugins/gen-typescript/1.0.0.wasm"
output_dir = "./generated/ts"

[generators.typescript-wasm.args]
style = "interface"
```

Same protocol, sandboxed execution. The WASM module exports a function that accepts GenerationRequest bytes and returns GenerationResponse bytes. No filesystem access, no network access — pure computation.

### Plugin Timeout

Configurable timeout per generator:
```toml
timeout = 30  # seconds, default TBD
```

### Watch Mode

```
rusl generate --watch typescript
```

Re-runs generation when schemas change (file watcher on `.rusl/schemas/`).

### Chained Generators

Run multiple generators in sequence or parallel:
```
rusl generate typescript zod form-components
```

### CLI Filter Override

```
rusl generate forms --filter "forms/contact"
```

Override the config-level filter from the command line.
