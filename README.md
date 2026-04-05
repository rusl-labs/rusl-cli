# Rusl Schema Manager

`rusl` is a Rust workspace for resolving, caching, linking, and generating code from schema dependencies.

## Workspace Layout

- `crates/rusl-cli` - the `rusl` binary; argument parsing and terminal rendering only
- `crates/rusl-app` - shared application/core logic, use cases, and transport-agnostic orchestration
- `crates/rusl-api-client` - generated OpenAPI client plus the thin handwritten auth/retry wrapper
- `openapi/` - committed backend OpenAPI snapshot
- `scripts/` - spec refresh and client regeneration scripts

The user-facing command is still `rusl`. The workspace split is internal architecture, not a product rename.

## Common Commands

Use the repo `Makefile` for the common flows:

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

## Configuration

Project configuration lives in `rusl.config.toml`:

```toml
api_base_url = "http://localhost:4000"
website_url = "http://localhost:3000"

[generators.typescript]
command = "bunx rusl-gen-typescript"
output_dir = "./generated/types"
default = true
```

Resolution order:

1. `RUSL_API_URL` / `RUSL_WEBSITE_URL`
2. project `rusl.config.toml`
3. user config from the OS config directory for `rusl`
4. compile-time defaults

`api_base_url` and `website_url` are separate. Do not derive one from the other.

## Usage

Example `rusl.bundle.toml`:

```toml
[bundle]
name = "my-company/test-bundle"
version = "0.1.0"

[schemas]
"rusl/common" = "*"
"external/address" = ">= 1.2.0"
```

Install dependencies:

```bash
rusl install
```

Generate code with the default configured generator:

```bash
rusl generate
```

Authenticate with the local or configured backend:

```bash
rusl login
rusl whoami
```

## OpenAPI Client Workflow

The backend client is generated and checked in.

- refresh the committed spec snapshot with `make openapi-refresh`
- regenerate the Rust client with `make openapi-generate`

Generated code lives under `crates/rusl-api-client/generated`. Handwritten transport behavior stays in `crates/rusl-api-client/src`.

## Architecture Rules

- `rusl-cli` is a thin gateway
- `rusl-app` owns use cases and orchestration
- `rusl-api-client` owns HTTP transport details, auth token handling, refresh behavior, and generated API bindings
- future gateways, including a local MCP/STDIO server, should depend on `rusl-app` rather than reimplementing logic
