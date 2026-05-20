# Release Process

See [`publish.md`](publish.md) for the current maintainer workflow.

Rusl CLI releases are built by `dist` from tags in `rusl-labs/rusl-cli`.
Release artifacts are published to GitHub Releases, and Homebrew formula updates are committed to
the Rusl tap.

## One-time Setup

1. Create a public GitHub repo named `rusl-labs/homebrew-tap`.
2. Initialize the tap repo with a README. Do not add a formula by hand unless a release needs
   manual recovery.
3. Create a GitHub token that can push to `rusl-labs/homebrew-tap`.
4. Add that token to `rusl-labs/rusl-cli` as the `HOMEBREW_TAP_TOKEN` repository secret.
5. Confirm `dist` is available locally:

   ```bash
   curl --proto '=https' --tlsv1.2 -LsSf https://github.com/axodotdev/cargo-dist/releases/download/v0.31.0/cargo-dist-installer.sh | sh
   ```

6. Validate the release plan before the first tag:

   ```bash
   dist plan
   ```

## Cutting a Release

1. Bump the CLI version in `crates/rusl-cli/Cargo.toml`.
2. Bump workspace crate versions that must stay aligned with the CLI.
3. Run:

   ```bash
   make verify
   dist plan
   ```

4. Commit the version bump and any release notes.
5. Tag the release with the `vX.Y.Z` form:

   ```bash
   git tag vX.Y.Z
   git push origin master
   git push origin vX.Y.Z
   ```

6. Confirm the GitHub release contains archives, checksums, `rusl-installer.sh`, and
   `rusl-installer.ps1`.
7. Confirm `rusl-labs/homebrew-tap` contains an updated `Formula/rusl.rb`.

## User Install Commands

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

## User Update Commands

Homebrew:

```bash
brew update && brew upgrade rusl-labs/tap/rusl
```

Installer users should rerun the installer command. Manual users should download the current
archive from `https://github.com/rusl-labs/rusl-cli/releases/latest`.

Rusl intentionally does not ship a `rusl upgrade` command or standalone updater. The installer or
package manager that placed the binary on disk owns the update path.

## How the Homebrew Tap Works

`rusl-labs/homebrew-tap` is a normal GitHub repository. Homebrew strips the `homebrew-` prefix from
tap repository names, so the package published in `rusl-labs/homebrew-tap` installs as
`rusl-labs/tap/rusl`.

During a tagged release, `dist` builds the release artifacts, uploads them to GitHub Releases, and
uses `HOMEBREW_TAP_TOKEN` to commit the updated formula to `rusl-labs/homebrew-tap`. The tap repo
must exist before the first release, but `dist` manages the formula contents.

## Recovery Notes

- If the release workflow fails before publishing, fix the issue and rerun the workflow for the tag.
- If GitHub Release artifacts were published but the tap update failed, verify `HOMEBREW_TAP_TOKEN`
  and rerun the workflow job that publishes Homebrew.
- Avoid manual formula edits. If a manual recovery is unavoidable, keep the formula pointed at the
  exact GitHub Release archive URLs and checksums from the release.
