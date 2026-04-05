#!/usr/bin/env bash
set -euo pipefail

script_dir=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
repo_root=$(cd "$script_dir/.." && pwd)
spec_path=${RUSL_OPENAPI_SPEC:-$repo_root/openapi/rusl-openapi.json}
output_dir=${RUSL_RUST_CLIENT_DIR:-$repo_root/crates/rusl-api-client/generated}
generator_cli_version=${OPENAPI_GENERATOR_CLI_VERSION:-2.31.0}
tmp_dir=$(mktemp -d)

cleanup() {
  rm -rf "$tmp_dir"
}
trap cleanup EXIT

if [ ! -f "$spec_path" ]; then
  printf 'OpenAPI spec not found at %s\n' "$spec_path" >&2
  printf 'Run scripts/refresh-openapi-spec.sh first.\n' >&2
  exit 1
fi

npx --yes "@openapitools/openapi-generator-cli@${generator_cli_version}" generate \
  -g rust \
  -i "$spec_path" \
  -o "$tmp_dir" \
  --additional-properties=library=reqwest,packageName=rusl-openapi-client,packageVersion=0.1.0,hideGenerationTimestamp=true

python - <<'PY_FIX' "$tmp_dir"
from pathlib import Path
import sys

root = Path(sys.argv[1])
for path in root.rglob("*.rs"):
    text = path.read_text()
    updated = text.replace("models::serde_json::Value", "serde_json::Value")
    if path.name == "lib.rs" and path.parent.name == "src":
        updated = updated.replace(
            "#![allow(unused_imports)]\n#![allow(clippy::too_many_arguments)]\n",
            "#![allow(clippy::all)]\n#![allow(unused_imports)]\n#![allow(clippy::too_many_arguments)]\n",
            1,
        )
    if updated != text:
        path.write_text(updated)
PY_FIX

rm -rf "$output_dir"
mkdir -p "$(dirname "$output_dir")"
mv "$tmp_dir" "$output_dir"
trap - EXIT
printf 'Generated Rust client in %s\n' "$output_dir"
