#!/usr/bin/env bash
# Shared env for all tenet test steps.
# Source this file; don't run it directly.
#
# Required (set before sourcing or export in shell):
#   GUEPARD_DB_PASSWORD   — password used when creating the deployment
#   GUEPARD_DB_ADMIN_PASSWORD — guepard-admin password for GRANT (if needed)
#
# Persistent state is written to $STATE_DIR so steps 2/3 can resume.

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
export ROOT

# ── binary ──────────────────────────────────────────────────────────────────
if [[ -z "${GUEPARD_BIN:-}" && -x "$ROOT/target/release/guepard" ]]; then
  GUEPARD_BIN="$ROOT/target/release/guepard"
else
  GUEPARD_BIN="${GUEPARD_BIN:-guepard}"
fi
export GUEPARD_BIN

# ── API targets ──────────────────────────────────────────────────────────────
export GUEPARD_API_URL="${GUEPARD_API_URL:-https://api.dev.guepard.run}"
export GUEPARD_APP_URL="${GUEPARD_APP_URL:-https://app.dev.guepard.run}"
export GUEPARD_DB_PASSWORD="${GUEPARD_DB_PASSWORD:?set GUEPARD_DB_PASSWORD}"

# ── Network (dev EIP) ────────────────────────────────────────────────────────
DEV_DB_EIP="${DEV_DB_EIP:-44.239.18.139}"
export DB_HOST="${DB_HOST:-$DEV_DB_EIP}"
export CLIENT_HOST="${CLIENT_HOST:-$DEV_DB_EIP}"
export DB_PORT="${DB_PORT:-}"

# ── Deployment defaults ──────────────────────────────────────────────────────
export REGION="${REGION:-us-west}"
export DATACENTER="${DATACENTER:-aws}"
export DB_PROVIDER="${DB_PROVIDER:-PostgreSQL}"
export DB_VERSION="${DB_VERSION:-16}"
export INSTANCE_TYPE="${INSTANCE_TYPE:-REPOSITORY}"

# ── Tenet ports (auto-increment on conflict) ─────────────────────────────────
export TENET_BASE_PROXY_PORT="${TENET_BASE_PROXY_PORT:-30100}"
export TENET_PORT_MAX="${TENET_PORT_MAX:-50}"

# ── State file (shared across steps) ────────────────────────────────────────
export STATE_DIR="${STATE_DIR:-$ROOT/scripts/test/.state}"
export STATE_FILE="$STATE_DIR/deployment.env"
mkdir -p "$STATE_DIR"

# ── SQL / proxy configs ──────────────────────────────────────────────────────
export SEED_SQL="${SEED_SQL:-$ROOT/scripts/test/sql/seed.sql}"
export PROXY_DIR="$ROOT/scripts/test/proxy"

# ── Helpers ──────────────────────────────────────────────────────────────────
section() { printf '\n\033[1m── %s ──\033[0m\n' "$*"; }
ok()      { printf '  \033[32m✓\033[0m %s\n' "$*"; }
info()    { printf '  \033[36m·\033[0m %s\n' "$*"; }
warn()    { printf '  \033[33m!\033[0m %s\n' "$*"; }
die()     { printf '  \033[31m✗\033[0m %s\n' "$*" >&2; exit 1; }

need() { command -v "$1" >/dev/null 2>&1 || die "missing dependency: $1"; }

# Patch ~/.guepard/config.json for dev API (no guepard config --api-url = no logout)
CONFIG_PATH="$HOME/.guepard/config.json"
_config_backup=""
patch_dev_api() {
  mkdir -p "$HOME/.guepard"
  if [[ -f "$CONFIG_PATH" ]]; then
    _config_backup="$(cat "$CONFIG_PATH")"
  fi
  local patched
  if [[ -f "$CONFIG_PATH" ]]; then
    patched="$(jq --arg api "$GUEPARD_API_URL" --arg app "$GUEPARD_APP_URL" \
      '.api_url=$api | .app_url=$app' "$CONFIG_PATH")"
  else
    patched="$(jq -n --arg api "$GUEPARD_API_URL" --arg app "$GUEPARD_APP_URL" \
      '{api_url:$api,app_url:$app}')"
  fi
  printf '%s' "$patched" > "$CONFIG_PATH"
  chmod 600 "$CONFIG_PATH" 2>/dev/null || true
}
restore_dev_api() {
  [[ -n "${GUEPARD_E2E_NO_RESTORE:-}" ]] && return 0
  if [[ -n "$_config_backup" ]]; then
    printf '%s' "$_config_backup" > "$CONFIG_PATH"
  fi
}

run_guepard()      { "$GUEPARD_BIN" "$@" 2>/tmp/g.err; }
run_guepard_json() { "$GUEPARD_BIN" "$@" --json 2>/tmp/g.err; }

is_retryable() {
  grep -qE '502|503|504|Bad Gateway|Service Unavailable|Gateway Timeout' /tmp/g.err 2>/dev/null
}

try_json() {
  local label="$1"; shift
  local out attempt max="${GUEPARD_E2E_RETRIES:-12}" wait="${GUEPARD_E2E_RETRY_WAIT:-5}"
  for attempt in $(seq 1 "$max"); do
    if out="$(run_guepard_json "$@")"; then printf '%s' "$out"; return 0; fi
    if is_retryable && [[ "$attempt" -lt "$max" ]]; then
      warn "$label: 5xx (attempt $attempt/$max), retry in ${wait}s…"
      sleep "$wait"; continue
    fi
    return 1
  done
  return 1
}

require_json() {
  local label="$1"; shift; local out
  out="$(try_json "$label" "$@")" || die "$label failed: $(tr '\n' ' ' </tmp/g.err | head -c 300)"
  printf '%s' "$out"
}

psql_uri() {
  local db="${1:-postgres}"
  printf 'postgresql://%s:%s@%s:%s/%s?sslmode=require' \
    "${DB_USER:-guepard}" "$(printf %s "${DB_PASS:-postgres}" | jq -sRr @uri)" \
    "$DB_HOST" "$DB_PORT" "$db"
}

save_state() {
  cat > "$STATE_FILE" <<EOF
DEPLOYMENT_ID="${DEPLOYMENT_ID:-}"
TENANT_ID="${TENANT_ID:-}"
DB_PORT="${DB_PORT:-}"
DB_USER="${DB_USER:-guepard}"
DB_PASS="${DB_PASS:-postgres}"
DB_NAME="${DB_NAME:-postgres}"
EOF
  info "state saved → $STATE_FILE" >&2
}

load_state() {
  [[ -f "$STATE_FILE" ]] || die "no state file at $STATE_FILE — run step-1 first"
  # shellcheck source=/dev/null
  source "$STATE_FILE"
  info "state loaded: deployment=$DEPLOYMENT_ID tenant=$TENANT_ID db_port=$DB_PORT" >&2
}

# TCP probe (Postgres port on Tenet proxy host)
tenet_proxy_port_open() {
  local port="$1"
  timeout 2 bash -c "echo >/dev/tcp/${CLIENT_HOST}/${port}" 2>/dev/null
}

# Try explicit env/state ports, then scan TENET_BASE_PROXY_PORT..+SCAN_MAX (step 1)
discover_tenet_proxy_port() {
  local -a candidates=()
  local base="${TENET_BASE_PROXY_PORT:-30100}"
  local span="${TENET_PORT_SCAN_MAX:-40}"
  local p port uri

  [[ -n "${TENET_PROXY_PORT:-}" ]] && candidates+=("$TENET_PROXY_PORT")
  [[ -n "${PROXY_PORT_A:-}" ]] && candidates+=("$PROXY_PORT_A")
  [[ -n "${PROXY_PORT_B:-}" ]] && candidates+=("$PROXY_PORT_B")
  [[ -n "${PROXY_PORT_C:-}" ]] && candidates+=("$PROXY_PORT_C")
  for ((p = base; p < base + span; p++)); do
    candidates+=("$p")
  done

  for port in $(printf '%s\n' "${candidates[@]}" | sort -nu); do
    if ! tenet_proxy_port_open "$port"; then
      continue
    fi
    uri="$(printf 'postgresql://%s:%s@%s:%s/postgres?sslmode=disable' \
      "${DB_USER:-guepard}" "$(printf %s "${DB_PASS:-postgres}" | jq -sRr @uri)" \
      "$CLIENT_HOST" "$port")"
    if psql "$uri" -tAc 'SELECT 1' >/dev/null 2>&1; then
      printf '%s' "$port"
      return 0
    fi
    info "port $port open but psql failed — trying next" >&2
  done

  die "no Tenet proxy on $CLIENT_HOST (scanned $base..$((base + span - 1)); is tenet running?)"
}
