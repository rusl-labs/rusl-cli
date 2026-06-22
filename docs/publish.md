# Publish Rusl CLI

Publishing is intentionally a two-step process:

1. `make prepare-release VERSION=x.y.z` updates version files and refreshes `Cargo.lock`.
2. A maintainer reviews the diff, commits it, creates the tag, and pushes the tag.

Do not add a one-shot `make publish` that commits, tags, pushes, and bumps the next development
version. That is too much release authority for one command. The tag push is the publishing action,
so it should stay explicit.

## Default Production Servers

Release builds default to:

| Setting | Default |
| --- | --- |
| API server | `https://resources.rusl.com` |
| Website server | `https://rusl.com` |

Maintainers can override these at build time with `RUSL_DEFAULT_API_URL` and
`RUSL_DEFAULT_WEBSITE_URL`. Users can override them at runtime with `RUSL_API_URL` and
`RUSL_WEBSITE_URL`, or with `api_base_url` and `website_url` in `rusl.config.toml`.

## One-time Homebrew Setup

1. Create the public repo `rusl-labs/homebrew-tap`.
2. Initialize it with a README. Do not create `Formula/rusl.rb` by hand.
3. Create a GitHub token that can push to `rusl-labs/homebrew-tap`.
4. Add that token to `rusl-labs/rusl-cli` as `HOMEBREW_TAP_TOKEN`.

`dist` will create and update `Formula/rusl.rb` during tagged releases.

## Release Checklist

1. Confirm the worktree is clean except for intended release changes:

   ```bash
   git status --short
   ```

2. Prepare the release version:

   ```bash
   make prepare-release VERSION=0.2.0
   ```

3. Verify and inspect the release plan:

   ```bash
   make verify
   dist plan
   git diff
   ```

4. Commit the release bump:

   ```bash
   git add crates/rusl-cli/Cargo.toml crates/rusl-app/Cargo.toml crates/rusl-api-client/Cargo.toml Cargo.lock
   git commit -m "Release v0.2.0"
   ```

5. Tag and publish:

   ```bash
   git tag v0.2.0
   git push origin HEAD
   git push origin v0.2.0
   ```

The tag push starts `.github/workflows/release.yml`. That workflow builds binaries, creates the
GitHub Release, uploads installer scripts, and updates the Homebrew tap.

## Post-release Checks

1. Confirm the GitHub Release includes:
   - `rusl-installer.sh`
   - `rusl-installer.ps1`
   - checksums
   - macOS arm64/x64 archives
   - Linux arm64/x64 archives
   - Windows x64 archive
2. Confirm `rusl-labs/homebrew-tap` contains `Formula/rusl.rb`.
3. Install from Homebrew:

   ```bash
   brew install rusl-labs/tap/rusl
   rusl --version
   ```

4. Install with the shell installer on macOS or Linux:

   ```bash
   curl --proto '=https' --tlsv1.2 -LsSf https://github.com/rusl-labs/rusl-cli/releases/latest/download/rusl-installer.sh | sh
   rusl --version
   ```

## Recovery

- If release CI fails before a GitHub Release is created, fix the issue and rerun the workflow for
  the tag.
- If artifacts publish but Homebrew fails, check `HOMEBREW_TAP_TOKEN`, then rerun the Homebrew
  publish job.
- Avoid manual formula edits unless release recovery requires it. If manual recovery is unavoidable,
  use the exact GitHub Release artifact URLs and checksums from the published release.
