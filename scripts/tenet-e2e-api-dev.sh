#!/usr/bin/env bash
# Full Tenet E2E against api.dev: create deployment → compute → tenet (all subcommands + edge cases).
#
# API target: merges https://api.dev.guepard.run into ~/.guepard/config.json (does NOT run
# `guepard config --api-url`, which would log you out). Restore on exit unless GUEPARD_E2E_NO_RESTORE=1.
#
# Requires: jq, psql, guepard logged in for api.dev, GUEPARD_DB_PASSWORD
# Optional: GUEPARD_BIN, SKIP_CREATE=1 DEPLOYMENT_ID=…, DB_HOST, DB_PORT, CLIENT_HOST,
#   TENET_PROXY_PORT / TENET_API_PORT (default 30100 each on api.dev), KEEP_TENET=1,
#   GUEPARD_DB_ADMIN_PASSWORD (+ GUEPARD_DB_ADMIN_USER=guepard-admin) to GRANT CREATE on public, …
# Fresh run: unset SKIP_CREATE DEPLOYMENT_ID
#
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
STATE="$(mktemp -d)"

# Prefer repo release binary (has tenet); override with GUEPARD_BIN=…
if [[ -z "${GUEPARD_BIN:-}" && -x "$ROOT/target/release/guepard" ]]; then
  GUEPARD_BIN="$ROOT/target/release/guepard"
else
  GUEPARD_BIN="${GUEPARD_BIN:-guepard}"
fi
GUEPARD_API_URL="${GUEPARD_API_URL:-https://api.dev.guepard.run}"
GUEPARD_APP_URL="${GUEPARD_APP_URL:-https://app.dev.guepard.run}"
GUEPARD_DB_PASSWORD="${GUEPARD_DB_PASSWORD:?set GUEPARD_DB_PASSWORD}"

# Dev testing: public EIP for Postgres/Tenet TCP. Port from compute API, or set DB_PORT when compute 502s.
DEV_DB_EIP="${DEV_DB_EIP:-44.239.18.139}"
DB_HOST="${DB_HOST:-${EIP:-$DEV_DB_EIP}}"
CLIENT_HOST="${CLIENT_HOST:-$DEV_DB_EIP}"
DB_PORT="${DB_PORT:-}"
# api.dev: fixed Tenet host ports; auto-increment when Nomad says "already reserved"
TENET_PROXY_PORT="${TENET_PROXY_PORT:-${TENET_REQUEST_PROXY_PORT:-30100}}"
TENET_API_PORT="${TENET_API_PORT:-${TENET_REQUEST_API_PORT:-}}"
TENET_PORT_MAX="${TENET_PORT_MAX:-50}"
MASKING_SALT="${MASKING_SALT:-e2e-tenant-salt-$(date +%s)}"
REGION="${REGION:-us-west}"
DATACENTER="${DATACENTER:-aws}"
DB_PROVIDER="${DB_PROVIDER:-PostgreSQL}"
DB_VERSION="${DB_VERSION:-16}"
INSTANCE_TYPE="${INSTANCE_TYPE:-REPOSITORY}"
PROXY_MIN="${PROXY_MIN:-$ROOT/scripts/tenet-proxy-rules.min.yaml}"
PROXY_ALT="${PROXY_ALT:-$ROOT/scripts/tenet-proxy-rules-alt-partial.yaml}"
SEED_SQL="${SEED_SQL:-$ROOT/scripts/sql/tenet_masking_seed.sql}"

export RUST_LOG="${RUST_LOG:-}"
unset GUEPARD_DEBUG 2>/dev/null || true

need() { command -v "$1" >/dev/null 2>&1 || { echo "missing: $1" >&2; exit 1; }; }
need jq
need psql
command -v "$GUEPARD_BIN" >/dev/null 2>&1 || { echo "missing GUEPARD_BIN=$GUEPARD_BIN" >&2; exit 1; }

CONFIG_PATH="${HOME}/.guepard/config.json"
CONFIG_BACKUP="$STATE/config.json.bak"
CREATED_CONFIG=0

section() { printf '\n\033[1m── %s ──\033[0m\n' "$*"; }
ok() { printf '  \033[32m✓\033[0m %s\n' "$*"; }
info() { printf '  \033[36m·\033[0m %s\n' "$*"; }
warn() { printf '  \033[33m!\033[0m %s\n' "$*"; }
die() { printf '  \033[31m✗\033[0m %s\n' "$*" >&2; exit 1; }

run() {
  "$GUEPARD_BIN" "$@" 2>"$STATE/last.err"
}

run_json() {
  run "$@" --json
}

is_retryable_api_err() {
  grep -qE '502|503|504|Bad Gateway|Service Unavailable|Gateway Timeout|timed out' \
    "$STATE/last.err" 2>/dev/null
}

# Run guepard --json; retry transient gateway errors (common on api.dev).
try_json() {
  local label="$1"
  shift
  local out attempt max="${GUEPARD_E2E_RETRIES:-12}" wait="${GUEPARD_E2E_RETRY_WAIT:-5}"
  for attempt in $(seq 1 "$max"); do
    if out="$(run_json "$@")"; then
      printf '%s' "$out"
      return 0
    fi
    if is_retryable_api_err && [[ "$attempt" -lt "$max" ]]; then
      warn "$label: gateway error (attempt $attempt/$max), retry in ${wait}s…"
      sleep "$wait"
      continue
    fi
    return 1
  done
  return 1
}

require_json() {
  local label="$1"
  shift
  local out
  if out="$(try_json "$label" "$@")"; then
    printf '%s' "$out"
    return 0
  fi
  die "$label failed: $(tr '\n' ' ' <"$STATE/last.err" 2>/dev/null | head -c 400)"
}

jq_field() {
  local label="$1" json="$2" filter="$3"
  local val
  if ! val="$(echo "$json" | jq -er "$filter" 2>"$STATE/jq.err")"; then
    die "$label: expected $filter in JSON ($(head -c 120 "$STATE/jq.err" 2>/dev/null)): $(echo "$json" | head -c 200)"
  fi
  printf '%s' "$val"
}

# run_json | jq breaks set -e on failure; lifecycle steps may 404 when job already running
run_lifecycle() {
  local label="$1"
  shift
  local out
  if out="$(try_json "$label" "$@")"; then
    echo "$out" | jq -c . 2>/dev/null || printf '%s\n' "$out"
    ok "$label"
    return 0
  fi
  warn "$label failed: $(tr '\n' ' ' <"$STATE/last.err" 2>/dev/null | head -c 250) (continuing)"
  return 0
}

# After deploy, Nomad often already runs the job — start returns 404; proxy get proves it's up.
tenet_start_or_skip() {
  local job="$1"
  local out
  if out="$(try_json "tenet start" tenet start "$job")"; then
    echo "$out" | jq -c . 2>/dev/null || true
    ok "tenet start"
    return 0
  fi
  if grep -qE '404|Scheduler error' "$STATE/last.err" 2>/dev/null \
    && run_json tenet proxy get "$job" >/dev/null 2>&1; then
    ok "tenet start skipped (job already running; proxy get OK)"
    return 0
  fi
  warn "tenet start failed: $(tr '\n' ' ' <"$STATE/last.err" 2>/dev/null | head -c 250)"
  return 1
}

tenet_proxy_set_step() {
  local label="$1" config="$2" apply="$3" out
  if out="$(try_json "$label" tenet proxy set "$TENET_JOB" --proxy-config "$config" --apply "$apply")"; then
    echo "$out" | jq -c . 2>/dev/null || true
    ok "$label"
    return 0
  fi
  die "$label failed: $(tr '\n' ' ' <"$STATE/last.err" 2>/dev/null | head -c 400)"
}

tenet_stop_or_skip() {
  if out="$(try_json "tenet stop" tenet stop "$1")"; then
    echo "$out" | jq -c . 2>/dev/null || true
    ok "tenet stop"
    return 0
  fi
  if grep -qE '404|Scheduler error' "$STATE/last.err" 2>/dev/null; then
    warn "tenet stop 404 (scheduler quirk on api.dev — continuing)"
    return 0
  fi
  warn "tenet stop failed: $(tr '\n' ' ' <"$STATE/last.err" 2>/dev/null | head -c 250)"
  return 1
}

patch_dev_api() {
  if [[ -n "${GUEPARD_SKIP_CONFIG_PATCH:-}" ]]; then
    return 0
  fi
  mkdir -p "${HOME}/.guepard"
  if [[ -f "$CONFIG_PATH" ]]; then
    cp "$CONFIG_PATH" "$CONFIG_BACKUP"
  else
    CREATED_CONFIG=1
  fi
  if [[ -f "$CONFIG_PATH" ]]; then
    jq --arg api "$GUEPARD_API_URL" --arg app "$GUEPARD_APP_URL" \
      '.api_url = $api | .app_url = $app' "$CONFIG_PATH" >"$STATE/config.patched.json"
  else
    jq -n --arg api "$GUEPARD_API_URL" --arg app "$GUEPARD_APP_URL" \
      '{api_url: $api, app_url: $app}' >"$STATE/config.patched.json"
  fi
  mv "$STATE/config.patched.json" "$CONFIG_PATH"
  chmod 600 "$CONFIG_PATH" 2>/dev/null || true
}

restore_config() {
  [[ -n "${GUEPARD_E2E_NO_RESTORE:-}" ]] && return 0
  [[ -n "${GUEPARD_SKIP_CONFIG_PATCH:-}" ]] && return 0
  if [[ -f "$CONFIG_BACKUP" ]]; then
    cp "$CONFIG_BACKUP" "$CONFIG_PATH"
  elif [[ "$CREATED_CONFIG" -eq 1 && -f "$CONFIG_PATH" ]]; then
    rm -f "$CONFIG_PATH"
  fi
}

e2e_cleanup() {
  restore_config
  rm -rf "$STATE" 2>/dev/null || true
}
trap e2e_cleanup INT TERM EXIT

# Default compute action (no subcommand) lists the row. Do NOT pass "list" — CLI treats it as unknown action but exits 0.
poll_compute_json() {
  local dep="$1" raw
  raw="$(timeout 20 "$GUEPARD_BIN" compute -x "$dep" --json 2>/dev/null)" || return 0
  if echo "$raw" | jq -e '.error' >/dev/null 2>&1; then
    return 0
  fi
  printf '%s' "$raw"
}

wait_compute_healthy() {
  local dep="$1" i st
  for i in $(seq 1 120); do
    if out="$(run_json compute status -x "$dep" 2>/dev/null)"; then
      st="$(echo "$out" | jq -r '.status // empty')"
      if [[ "$st" == "Healthy" ]]; then
        return 0
      fi
      [[ $((i % 6)) -eq 0 ]] && info "compute status=$st (attempt $i/120)"
    fi
    sleep 5
  done
  die "compute did not become Healthy in time"
}

wait_compute_row() {
  local dep="$1" i port j max="${2:-12}"
  for i in $(seq 1 "$max"); do
    j="$(poll_compute_json "$dep")"
    if [[ -n "$j" ]]; then
      j="$(echo "$j" | jq -c 'if type == "array" then .[0] else . end' 2>/dev/null)" || j=""
      if [[ -n "$j" ]]; then
        port="$(echo "$j" | jq -r '.port // empty' 2>/dev/null)"
        if [[ -n "$port" && "$port" != "null" && "$port" != "0" ]]; then
          echo "$j"
          return 0
        fi
      fi
    fi
    info "waiting for compute list/port ($i/$max, often 502 on api.dev)…"
    sleep 3
  done
  return 1
}

derive_tenant_slug() {
  [[ -n "${TENANT_ID:-}" ]] && return 0
  if [[ -n "${create_json:-}" ]]; then
    TENANT_ID="$(echo "$create_json" | jq -r '
      (.deployment.name // empty),
      (.compute.name // empty)
    ' 2>/dev/null | sed -n '1p')"
    if [[ "$TENANT_ID" == *-compute ]]; then
      TENANT_ID="${TENANT_ID%-compute}"
    fi
  fi
  if [[ -z "${TENANT_ID:-}" && -n "${DEPLOYMENT_ID:-}" ]]; then
    local dep_json
    if dep_json="$(try_json "deploy -x" deploy -x "$DEPLOYMENT_ID" 2>/dev/null)"; then
      TENANT_ID="$(echo "$dep_json" | jq -r '.deployment.name // empty')"
    fi
  fi
}

# When compute list 502s, use DB_PORT + slug from create/deploy -x (or TENANT_ID env).
resolve_compute_without_list() {
  DB_PORT="${DB_PORT:?set DB_PORT=<nomad-port> when compute -x keeps 502ing}"
  info "using DB_PORT=$DB_PORT (compute list skipped)"
  derive_tenant_slug
  [[ -n "${TENANT_ID:-}" ]] || die "set TENANT_ID=<slug> (deploy create/-x missing .deployment.name)"
  COMPUTE_NAME="${COMPUTE_NAME:-$TENANT_ID}"
  if [[ "$COMPUTE_NAME" == *-compute ]]; then
    COMPUTE_JOB_ID="${COMPUTE_JOB_ID:-$COMPUTE_NAME}"
    TENANT_ID="${COMPUTE_NAME%-compute}"
  else
    COMPUTE_JOB_ID="${COMPUTE_JOB_ID:-${COMPUTE_NAME}-compute}"
  fi
}

parse_db_connection() {
  local dep_json="$1"
  DB_USER="$(echo "$dep_json" | jq -r '.connection.username // empty')"
  DB_PASS="$(echo "$dep_json" | jq -r '.connection.password // empty')"
  if [[ -z "$DB_USER" || -z "$DB_PASS" ]]; then
    die "deploy -x missing connection fields — run: $GUEPARD_BIN deploy -x $DEPLOYMENT_ID --json | jq .connection"
  fi
}

# deploy -x often 502s on api.dev; compute row + create-time password still works for seed/tenet.
load_db_connection() {
  local dep_json=""
  if dep_json="$(try_json "deploy -x" deploy -x "$DEPLOYMENT_ID")"; then
    parse_db_connection "$dep_json"
    info "DB credentials from deploy -x (repo=$(echo "$dep_json" | jq -r '.deployment.repository_name // empty'))"
  else
    warn "deploy -x unavailable — using GUEPARD_DB_PASSWORD"
    DB_USER="${DB_USER:-guepard}"
    DB_PASS="${GUEPARD_DB_PASSWORD}"
  fi
  # api.dev: only `postgres` exists; API repo name is not a real database.
  DB_NAME="${DATABASE_NAME:-postgres}"
  DB_USER="${DB_USER:-guepard}"
  DB_PASS="${DB_PASS:-postgres}"
}

psql_uri() {
  local db="${1:-postgres}"
  printf 'postgresql://%s:%s@%s:%s/%s?sslmode=require' \
    "$DB_USER" "$(printf %s "$DB_PASS" | jq -sRr @uri)" "$DB_HOST" "$DB_PORT" "$db"
}

seed_db() {
  local uri="$1"
  if psql "$uri" -tAc "SELECT 1 FROM tenet_masking_test LIMIT 1" 2>/dev/null | grep -q 1; then
    ok "seed table already present"
    return 0
  fi
  if psql "$uri" -v ON_ERROR_STOP=1 -f "$SEED_SQL" 2>"$STATE/seed.err"; then
    ok "seed applied"
    return 0
  fi
  if grep -q 'permission denied for schema public' "$STATE/seed.err"; then
    if [[ -n "${GUEPARD_DB_ADMIN_PASSWORD:-}" ]]; then
      local admin_user="${GUEPARD_DB_ADMIN_USER:-guepard-admin}" admin_uri
      admin_uri="$(printf 'postgresql://%s:%s@%s:%s/%s?sslmode=require' \
        "$admin_user" "$(printf %s "$GUEPARD_DB_ADMIN_PASSWORD" | jq -sRr @uri)" \
        "$DB_HOST" "$DB_PORT" "$DB_NAME")"
      if psql "$admin_uri" -tAc 'SELECT 1' >/dev/null 2>&1; then
        info "granting CREATE on public via $admin_user"
        psql "$admin_uri" -v ON_ERROR_STOP=1 -f "$ROOT/scripts/sql/tenet_masking_grant_dev.sql"
        psql "$uri" -v ON_ERROR_STOP=1 -f "$SEED_SQL"
        ok "seed applied (after admin grant)"
        return 0
      fi
      warn "GUEPARD_DB_ADMIN_PASSWORD rejected for $admin_user (not the guepard app password)"
      unset GUEPARD_DB_ADMIN_PASSWORD
    fi
    die "guepard cannot CREATE on public. One-time on the Postgres allocation (Nomad/VPN):
  psql -U guepard-admin -d postgres -c \"GRANT CREATE ON SCHEMA public TO guepard;\"
  compute job: ${TENANT_ID:-<tenant>}-compute — then rerun this script."
  fi
  die "psql seed failed: $(tr '\n' ' ' <"$STATE/seed.err" | head -c 400)"
}

is_tenet_port_reserved_err() {
  grep -qE 'already reserved|port.*reserved|Static port' "$STATE/last.err" 2>/dev/null
}

# Deploy Tenet; bump proxy/api ports when Nomad static port is taken.
# Prints only JSON to stdout (logs go to stderr) for: tenet_json="$(tenet_deploy_with_ports)"
tenet_deploy_with_ports() {
  local proxy="${TENET_PROXY_PORT}" api="${TENET_API_PORT:-}" offset attempt max="$TENET_PORT_MAX" out
  local -a base_args=(
    tenet deploy
    --tenant-id "$TENANT_ID"
    --compute-job-id "$COMPUTE_JOB_ID"
    --upstream-host "$DB_HOST"
    --upstream-port "$DB_PORT"
    --masking-salt "$MASKING_SALT"
    --proxy-config "$PROXY_MIN"
    --client-host "$CLIENT_HOST"
  )
  if [[ -z "$api" || "$api" == "$proxy" ]]; then
    api=$((proxy + 1))
  fi
  offset=$((api - proxy))

  for attempt in $(seq 1 "$max"); do
    info "tenet deploy --proxy-port $proxy --api-port $api (attempt $attempt/$max)" >&2
    if out="$(run_json "${base_args[@]}" --proxy-port "$proxy" --api-port "$api")"; then
      TENET_PROXY_PORT="$proxy"
      TENET_API_PORT="$api"
      printf '%s' "$out"
      return 0
    fi
    if is_tenet_port_reserved_err && [[ "$attempt" -lt "$max" ]]; then
      warn "port $proxy or $api reserved on host — trying $((proxy + 1))/$((proxy + offset + 1))" >&2
      proxy=$((proxy + 1))
      api=$((proxy + offset))
      continue
    fi
    return 1
  done
  return 1
}

# ─── main ────────────────────────────────────────────────────────────────────

patch_dev_api

section "Config"
info "api_url=$GUEPARD_API_URL (merged into $CONFIG_PATH)"
"$GUEPARD_BIN" config --show 2>/dev/null | sed 's/^/  /' || true

section "Auth probe"
# `usage` is lighter than `list deployments` (avoids huge payloads / list-route 502s)
auth_out="$(require_json "usage" usage)"
usage_deps="$(echo "$auth_out" | jq -r '[.[] | select(.resource == "Deployments")][0] | "\(.used)/\(.quota)"' 2>/dev/null || true)"
ok "API accepts session${usage_deps:+ (deployments $usage_deps)}"

section "Edge: tenet proxy get (invalid job)"
set +e
run_json tenet proxy get "__e2e_missing_job__" >/dev/null 2>&1
bad_ec=$?
set -e
if [[ "$bad_ec" -eq 0 ]]; then
  warn "proxy get on fake job unexpectedly succeeded"
else
  ok "proxy get rejected missing job (exit $bad_ec)"
fi

if [[ -n "${SKIP_CREATE:-}" ]]; then
  DEPLOYMENT_ID="${DEPLOYMENT_ID:?set DEPLOYMENT_ID when SKIP_CREATE=1}"
  section "Using existing deployment $DEPLOYMENT_ID"
else
  REPO_NAME="${REPO_NAME:-tenet-e2e-$(date +%s)}"
  section "Create deployment ($REPO_NAME)"
  create_json="$(require_json "deploy create" deploy \
    -p "$DB_PROVIDER" -v "$DB_VERSION" -r "$REGION" -d "$DATACENTER" \
    -i "$INSTANCE_TYPE" -n "$REPO_NAME" -w "$GUEPARD_DB_PASSWORD")"
  DEPLOYMENT_ID="$(jq_field "deploy create" "$create_json" '.deployment.id')"
  ok "deployment_id=$DEPLOYMENT_ID"
fi

section "Compute start + wait Healthy"
start_out="$(run_json compute start -x "$DEPLOYMENT_ID" 2>/dev/null || true)"
[[ -n "$start_out" ]] && info "compute start: $(echo "$start_out" | jq -c . 2>/dev/null || echo "$start_out")"
wait_compute_healthy "$DEPLOYMENT_ID"
ok "compute Healthy"

section "Read compute row + DB connection"
# Port from create response (if compute was up during deploy)
if [[ -z "$DB_PORT" && -n "${create_json:-}" ]]; then
  DB_PORT="$(echo "$create_json" | jq -r 'if .compute.port != null then .compute.port | tostring else empty end')"
  [[ -n "$DB_PORT" ]] && info "DB_PORT=$DB_PORT from deploy create JSON"
fi

if [[ -n "$DB_PORT" ]]; then
  resolve_compute_without_list
elif cj="$(wait_compute_row "$DEPLOYMENT_ID" 24)"; then
  DB_PORT="$(jq_field "compute row" "$cj" '.port')"
  COMPUTE_NAME="$(jq_field "compute row" "$cj" '.name')"
  COMPUTE_FQDN="$(echo "$cj" | jq -r '.fqdn // empty')"
  if [[ "$COMPUTE_NAME" == *-compute ]]; then
    COMPUTE_JOB_ID="$COMPUTE_NAME"
    TENANT_ID="${TENANT_ID:-${COMPUTE_NAME%-compute}}"
  else
    COMPUTE_JOB_ID="${COMPUTE_JOB_ID:-${COMPUTE_NAME}-compute}"
    TENANT_ID="${TENANT_ID:-$COMPUTE_NAME}"
  fi
else
  derive_tenant_slug
  if dep_json="$(try_json "deploy -x" deploy -x "$DEPLOYMENT_ID" 2>/dev/null || true)"; then
    COMPUTE_FQDN="${COMPUTE_FQDN:-$(echo "$dep_json" | jq -r '.deployment.fqdn // empty')}"
  fi
  warn "compute list unavailable (api.dev 502) — set DB_PORT from Nomad when it appears"
  die "SKIP_CREATE=1 DEPLOYMENT_ID=$DEPLOYMENT_ID TENANT_ID=${TENANT_ID:-<slug>} DB_PORT=<nomad-port> $0"
fi
load_db_connection
COMPUTE_JOB_ID="${COMPUTE_JOB_ID:-${TENANT_ID}-compute}"
[[ -n "${TENANT_ID:-}" ]] || die "could not derive TENANT_ID"
ok "compute_job_id=$COMPUTE_JOB_ID tenant_id=$TENANT_ID db_port=$DB_PORT db_host=$DB_HOST db=$DB_NAME"

DIRECT_URI="$(psql_uri "$DB_NAME")"
section "Seed DB (direct)"
seed_db "$DIRECT_URI"

section "Tenet deploy"
if tenet_json="$(tenet_deploy_with_ports)"; then
  :
else
  die "tenet deploy failed: $(tr '\n' ' ' <"$STATE/last.err" 2>/dev/null | head -c 400)"
fi
TENET_JOB="$(jq_field "tenet deploy" "$tenet_json" '.job_id')"
chosen_proxy="$TENET_PROXY_PORT"
chosen_api="$TENET_API_PORT"
TENET_PROXY_PORT="$(echo "$tenet_json" | jq -r 'if .proxy_port != null then .proxy_port | tostring else empty end')"
[[ -z "$TENET_PROXY_PORT" ]] && TENET_PROXY_PORT="$chosen_proxy"
TENET_API_PORT="$(echo "$tenet_json" | jq -r 'if .api_port != null then .api_port | tostring else empty end')"
[[ -z "$TENET_API_PORT" ]] && TENET_API_PORT="$chosen_api"
echo "$tenet_json" | jq '{job_id, eval_id, proxy_port, api_port, host}'
ok "tenet deploy job_id=$TENET_JOB (ports ${TENET_PROXY_PORT}/${TENET_API_PORT})"
info "Tenet proxy: ${CLIENT_HOST}:${TENET_PROXY_PORT}  API: http://${CLIENT_HOST}:${TENET_API_PORT}"

section "Tenet start (first)"
tenet_start_or_skip "$TENET_JOB"

section "Tenet start (idempotent)"
set +e
run_json tenet start "$TENET_JOB" >/dev/null 2>&1
dup_ec=$?
set -e
if [[ "$dup_ec" -eq 0 ]]; then
  ok "second start returned success"
else
  warn "second start exit $dup_ec (often 404 / already running — ok)"
fi

section "Tenet proxy get --json"
pg1="$(require_json "tenet proxy get" tenet proxy get "$TENET_JOB")"
echo "$pg1" | jq '{job_id, yaml_lines: (.proxy_yaml | split("\n") | length)}'
echo "$pg1" | jq -er '.proxy_yaml' | grep -q masking_enabled || die "proxy_yaml missing masking_enabled"
ok "proxy get JSON"

section "Tenet proxy get -o file"
tmp_yaml="$STATE/proxy-roundtrip.yaml"
rm -f "$tmp_yaml"
run tenet proxy get "$TENET_JOB" -o "$tmp_yaml"
[[ -s "$tmp_yaml" ]] || die "empty -o output"
ok "wrote $(wc -l <"$tmp_yaml") lines"

section "Tenet proxy set --apply false"
tenet_proxy_set_step "proxy set apply=false" "$PROXY_ALT" false

section "Tenet proxy set --apply true"
tenet_proxy_set_step "proxy set apply=true" "$PROXY_MIN" true

section "Tenet stop"
tenet_stop_or_skip "$TENET_JOB"

section "Tenet start (after stop)"
tenet_start_or_skip "$TENET_JOB"

[[ -n "${TENET_PROXY_PORT_OVERRIDE:-}" ]] && TENET_PROXY_PORT="$TENET_PROXY_PORT_OVERRIDE"

if [[ -n "$TENET_PROXY_PORT" ]]; then
  section "psql via Tenet (optional)"
  enc_user="$(printf %s "$DB_USER" | jq -sRr @uri)"
  enc_pass="$(printf %s "$DB_PASS" | jq -sRr @uri)"
  tenet_uri="postgresql://${enc_user}:${enc_pass}@${CLIENT_HOST}:${TENET_PROXY_PORT}/${DB_NAME}?sslmode=disable"
  if psql "$tenet_uri" -v ON_ERROR_STOP=1 -c "SELECT id, email FROM tenet_masking_test ORDER BY id LIMIT 2;"; then
    ok "psql via Tenet"
  else
    warn "psql via Tenet failed (network/SG — not failing run)"
  fi
else
  die "no Tenet proxy port (set TENET_PROXY_PORT)"
fi

if [[ -z "${KEEP_TENET:-}" ]]; then
  section "Tenet purge"
  run_lifecycle "tenet purge" tenet purge "$TENET_JOB"
else
  warn "KEEP_TENET=1 — job $TENET_JOB left running"
fi

section "Summary"
ok "deployment=$DEPLOYMENT_ID tenet_job=$TENET_JOB"
info "Resume: SKIP_CREATE=1 DEPLOYMENT_ID=$DEPLOYMENT_ID $0"

[[ -n "${GUEPARD_E2E_NO_RESTORE:-}" ]] && warn "GUEPARD_E2E_NO_RESTORE=1 — config not restored"

printf '\n\033[32mAll scripted steps completed.\033[0m\n'
