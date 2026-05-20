#!/usr/bin/env bash
set -euo pipefail

if [ "$#" -ne 1 ]; then
  printf '%s\n' "Usage: $0 <version>"
  printf '%s\n' "Example: $0 0.2.0"
  exit 1
fi

version="${1#v}"

if ! printf '%s' "$version" | grep -Eq '^[0-9]+\.[0-9]+\.[0-9]+([+-][0-9A-Za-z.-]+)?$'; then
  printf '%s\n' "Invalid version: $1"
  printf '%s\n' "Use semantic versions such as 0.2.0 or 0.2.0-rc.1."
  exit 1
fi

set_package_version() {
  local manifest="$1"
  local package="$2"

  VERSION="$version" PACKAGE="$package" perl -0pi -e '
    my $package = quotemeta($ENV{"PACKAGE"});
    my $version = $ENV{"VERSION"};
    my $changed = s/(\[package\]\s+name = "$package"\s+version = ")[^"]+(")/$1$version$2/s;
    die "Package $ENV{PACKAGE} not found in $ARGV\n" unless $changed;
  ' "$manifest"
}

set_package_version "crates/rusl-cli/Cargo.toml" "rusl"
set_package_version "crates/rusl-app/Cargo.toml" "rusl-app"
set_package_version "crates/rusl-api-client/Cargo.toml" "rusl-api-client"

cargo check --workspace

printf '%s\n' "Prepared release v$version."
printf '%s\n' "Next:"
printf '%s\n' "  make verify"
printf '%s\n' "  dist plan"
printf '%s\n' "  git add crates/rusl-cli/Cargo.toml crates/rusl-app/Cargo.toml crates/rusl-api-client/Cargo.toml Cargo.lock"
printf '%s\n' "  git commit -m \"Release v$version\""
printf '%s\n' "  git tag v$version"
printf '%s\n' "  git push origin HEAD"
printf '%s\n' "  git push origin v$version"
