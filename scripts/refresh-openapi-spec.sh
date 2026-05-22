#!/usr/bin/env bash
set -euo pipefail

script_dir=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
repo_root=$(cd "$script_dir/.." && pwd)
backend_dir=${RUSL_BACKEND_DIR:-$repo_root/../rusl_ex}
spec_url=${RUSL_OPENAPI_URL:-http://localhost:4000/api/openapi}
output_path=${1:-$repo_root/openapi/rusl-openapi.json}
raw_path=$(mktemp)
tmp_path=$(mktemp)
err_path=$(mktemp)

cleanup() {
  rm -f "$raw_path" "$tmp_path" "$err_path"
}
trap cleanup EXIT

if [ -d "$backend_dir" ] && command -v mix >/dev/null 2>&1; then
  spec_expr='spec = RuslWeb.ApiSpec.spec(); File.write!(System.fetch_env!("RUSL_OPENAPI_OUTPUT_PATH"), Jason.encode!(spec))'
  if command -v direnv >/dev/null 2>&1 && [ -f "$backend_dir/.envrc" ]; then
    if ! direnv exec "$backend_dir" bash -c 'cd "$1"; RUSL_OPENAPI_OUTPUT_PATH="$3" MIX_ENV=dev mix run -e "$2"' bash "$backend_dir" "$spec_expr" "$raw_path" > "$err_path" 2>&1; then
      cat "$err_path" >&2
      exit 1
    fi
  elif ! (
    cd "$backend_dir"
    RUSL_OPENAPI_OUTPUT_PATH="$raw_path" MIX_ENV=dev mix run -e "$spec_expr"
  ) > "$err_path" 2>&1; then
    cat "$err_path" >&2
    exit 1
  fi
else
  curl -fsSL "$spec_url" -o "$raw_path"
fi

node - "$raw_path" "$tmp_path" <<'EOF_NODE'
const fs = require('fs');
const [rawPath, outPath] = process.argv.slice(2);
const raw = fs.readFileSync(rawPath, 'utf8');

function findObjectEnd(start) {
  let depth = 0;
  let inString = false;
  let escaped = false;

  for (let i = start; i < raw.length; i += 1) {
    const char = raw[i];

    if (inString) {
      if (escaped) {
        escaped = false;
      } else if (char === '\\') {
        escaped = true;
      } else if (char === '"') {
        inString = false;
      }
      continue;
    }

    if (char === '"') {
      inString = true;
    } else if (char === '{') {
      depth += 1;
    } else if (char === '}') {
      depth -= 1;
      if (depth === 0) {
        return i + 1;
      }
    }
  }

  return -1;
}

let scanFrom = 0;
while (scanFrom < raw.length) {
  const start = raw.indexOf('{', scanFrom);
  if (start === -1) {
    break;
  }

  const end = findObjectEnd(start);
  if (end === -1) {
    break;
  }

  const json = raw.slice(start, end);
  try {
    const parsed = JSON.parse(json);
    if (
      parsed &&
      typeof parsed === 'object' &&
      parsed.info &&
      parsed.paths &&
      (parsed.openapi || parsed.swagger)
    ) {
      fs.writeFileSync(outPath, JSON.stringify(parsed));
      process.exit(0);
    }
  } catch (_) {
    scanFrom = start + 1;
    continue;
  }

  scanFrom = end;
}

console.error('Failed to locate OpenAPI JSON payload in command output');
process.exit(1);
EOF_NODE

mkdir -p "$(dirname "$output_path")"
mv "$tmp_path" "$output_path"
trap - EXIT
printf 'Wrote OpenAPI spec to %s\n' "$output_path"
