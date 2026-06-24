# Rusl Schema Manager

`rusl` installs, locks, and vendors schema dependencies from the Rusl registry into your project.

## Install

Homebrew:

```bash
brew install rusl-labs/tap/rusl
```

macOS and Linux installer:

```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/rusl-labs/rusl-cli/releases/latest/download/rusl-installer.sh | sh
```

Windows installer:

```powershell
powershell -ExecutionPolicy Bypass -c "irm https://github.com/rusl-labs/rusl-cli/releases/latest/download/rusl-installer.ps1 | iex"
```

## Get Started

Authenticate with the registry:

```bash
rusl login
rusl whoami
rusl logout
```

Add dependencies:

```bash
rusl add schema rusl/schemas/common
rusl add bundle rusl/bundles/common --version ">=1.0.0"
```

Install and link the resolved schemas:

```bash
rusl install
```

Remove dependencies when they are no longer needed:

```bash
rusl remove schema rusl/schemas/common
rusl remove bundle rusl/bundles/common
```

`rusl add` and `rusl remove` update `rusl.bundle.toml` and then run install so the local schema
snapshot stays current.

## Usage

Example `rusl.bundle.toml`:

```toml
[rusl.resources]
"rusl/schemas/common" = "*"
"rusl/bundles/common" = ">=1.0.0"
```

Useful commands:

```bash
rusl install
rusl list
rusl list --tree
rusl outdated
rusl why rusl/schemas/common
rusl cache --clear
rusl logout
```

## Update Rusl

Update Homebrew installs with:

```bash
brew update && brew upgrade rusl-labs/tap/rusl
```

Update installer-based installs by rerunning the installer command. Manual downloads are available
from the [latest GitHub release](https://github.com/rusl-labs/rusl-cli/releases/latest).

## Configuration

Project configuration lives in `rusl.config.toml` when needed:

```toml
[output]
schema_dir = "schemas/vendor"
suffix = ".schema.json"
```

Bundle dependencies live in `rusl.bundle.toml`. See
[`docs/configuration.md`](docs/configuration.md) for manifest options, config options, defaults,
and compatibility notes.

## Development

### Workspace Layout

- `crates/rusl-cli` - the `rusl` binary; argument parsing and terminal rendering only
- `crates/rusl-app` - shared application/core logic, use cases, and transport-agnostic orchestration
- `crates/rusl-api-client` - generated OpenAPI client plus the thin handwritten auth/retry wrapper
- `openapi/` - committed backend OpenAPI snapshot
- `scripts/` - spec refresh and client regeneration scripts

### Development Commands

Use the repo `Makefile` for local development:

```bash
make help
make install
make build
make release
make test
make check
make clippy
make fmt
make fmt-check
make verify
make openapi-refresh
make openapi-generate
```

Notes:

- `make install` installs the CLI from `crates/rusl-cli`
- `make verify` runs the full required verification suite for the workspace
- `cargo install --path .` does not work because the repo root is a virtual workspace manifest
- publish setup and maintainer steps live in [`docs/publish.md`](docs/publish.md)

### OpenAPI Client Workflow

The backend client is generated and checked in.

- refresh the committed spec snapshot with `make openapi-refresh`
- regenerate the Rust client with `make openapi-generate`

Generated code lives under `crates/rusl-api-client/generated`. Handwritten transport behavior stays in `crates/rusl-api-client/src`.

### Architecture Rules

- `rusl-cli` is a thin gateway
- `rusl-app` owns use cases and orchestration
- `rusl-api-client` owns HTTP transport details, auth token handling, refresh behavior, and generated API bindings
- future gateways, including a local MCP/STDIO server, should depend on `rusl-app` rather than reimplementing logic
