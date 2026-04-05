#!/usr/bin/env bash
set -euo pipefail

script_dir=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
repo_root=$(cd "$script_dir/.." && pwd)
backend_dir=${RUSL_BACKEND_DIR:-$repo_root/../rusl_ex}
spec_url=${RUSL_OPENAPI_URL:-http://localhost:4000/api/openapi}
output_path=${1:-$repo_root/openapi/rusl-openapi.json}
raw_path=$(mktemp)
tmp_path=$(mktemp)

cleanup() {
  rm -f "$raw_path" "$tmp_path"
}
trap cleanup EXIT

if [ -d "$backend_dir" ] && command -v mix >/dev/null 2>&1; then
  (
    cd "$backend_dir"
    MIX_ENV=dev mix run -e 'spec = RuslWeb.ApiSpec.spec(); IO.binwrite(Jason.encode!(spec))'
  ) > "$raw_path" 2>&1
else
  curl -fsSL "$spec_url" -o "$raw_path"
fi

node - "$raw_path" "$tmp_path" <<'EOF_NODE'
const fs = require('fs');
const [rawPath, outPath] = process.argv.slice(2);
const raw = fs.readFileSync(rawPath, 'utf8');
const start = raw.indexOf('{');
if (start === -1) {
  console.error('Failed to locate JSON payload in OpenAPI output');
  process.exit(1);
}
const json = raw.slice(start);
JSON.parse(json);
fs.writeFileSync(outPath, json);
EOF_NODE

mkdir -p "$(dirname "$output_path")"
mv "$tmp_path" "$output_path"
trap - EXIT
printf 'Wrote OpenAPI spec to %s\n' "$output_path"
