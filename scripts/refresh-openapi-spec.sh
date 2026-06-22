#!/usr/bin/env bash
set -euo pipefail

script_dir=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
repo_root=$(cd "$script_dir/.." && pwd)
spec_url=${RUSL_OPENAPI_URL:-http://localhost:4000/api/openapi/cli}
output_path=${1:-$repo_root/openapi/rusl-openapi.json}

echo "Fetching OpenAPI spec from $spec_url..."
mkdir -p "$(dirname "$output_path")"
curl -fsSL "$spec_url" -o "$output_path"
echo "Wrote OpenAPI spec to $output_path"

echo "Regenerating Rust client..."
"$script_dir/generate-rust-client.sh"
