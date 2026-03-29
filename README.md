# Rusl Schema Manager

Rusl is a state-of-the-art, lightning-fast Rust CLI meant to seamlessly resolve, cache, and symlink dependency networks of JSON Schemas. Modeled aggressively after Cargo and `pnpm`, it is powered natively by the **PubGrub** CDCL algorithm to guarantee mathematically perfect topological configurations instantly.

## Architecture

1. **Resolution via PubGrub Engine:** The CLI fetches comprehensive historical metadata matrices from a registry, feeding thousands of versions into the offline mathematical solver to identify the optimal, conflict-free dependency resolution map natively.
2. **Content-Addressable Storage (CAS):** Once the correct versions are proven, `rusl` downloads their schemas and caches them permanently via a SHA-256 integrity hash layer residing in the `~/.local/share/rusl/store/` global OS data tier, guaranteeing you never download the same schema payload twice.
3. **Ghost Symlinking:** To expose the schemas inside your project natively without duplicating disk footprint, `rusl` dynamically hard-links the identical payloads from the OS CAS directly into your working directory `.rusl/schemas/` structure.
4. **Repeatable Builds (Lockfile):** Resolutions are formally baked into a `rusl.lock` configuration manifest to enforce exact checksum repeatability across server grids securely.

---

## Configuration Setup

Rusl defines a **4-Tier precedence chain** for configuration mapping, prioritizing local flexibility while maintaining robust production scaling.

### Configuration Properties
Currently, the `Config` struct exposes two core network domains natively:
```toml
api_base_url = "https://api.registry.rusl.dev"
website_url = "https://registry.rusl.dev"
```
*(Note: For backward compatibility, `registry_url` is silently aliased to `api_base_url` if found in legacy configs).*

### The 4-Tier Precedence Chain

#### 1. Runtime Environment Variables (Highest Priority)
If you need ephemeral CI/CD injection or instant override mapping natively:
```bash
RUSL_API_URL="http://localhost:4000" RUSL_WEBSITE_URL="http://localhost:3000" rusl login
```

#### 2. Project-Level Targeting (`rusl.config.toml`)
Project-level configuration takes complete mathematical precedence over global defaults. The CLI exhaustively scans upward from your current working directory through every parent path natively searching for a `rusl.config.toml`. This allows you to permanently isolate registry URLs to an arbitrary monorepo!

#### 3. User-Level Scope (`~/.config/rusl/config.toml`)
If no project dotfile exists, user-level persistent boundaries override the binary fallbacks natively via: `<OS_CONFIG_DIR>/rusl/config.toml`

#### 4. Compile-Time Default Fallbacks (Lowest Priority)
If a user installs the binary with absolutely zero configuration files and no environment variables deployed, the baseline defaults to the public ecosystem: `https://api.rusl.app` natively.

*Note: You can permanently override this fallback explicitly during binary compilation using rust's compile-time hooks!*
```bash
RUSL_DEFAULT_API_URL="https://private-corp-registry.internal" cargo build --release
```

---

## Usage Example

Initialize your schema definition inside `rusl.bundle.toml`:
```toml
[bundle]
name = "my-company/test-bundle"
version = "0.1.0"

[schemas]
"rusl/common" = "*"
"external/address" = ">= 1.2.0"
```

Simply trigger the resolver:
```bash
rusl install
```
