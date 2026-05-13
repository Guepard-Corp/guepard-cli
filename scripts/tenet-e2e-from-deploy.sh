#!/usr/bin/env bash
# End-to-end: create deployment → wait for healthy compute → read Nomad static DB port
# from compute API → seed via psql → deploy Tenet (upstream = EIP:port) → start →
# exercise psql in several configurations.
#
# Postgres TCP host/port come from the compute API (`cargo run -q -- compute -x <id> --json`).
# Tenet **client** address (psql through Tenet, `--client-host`) defaults to **10.0.4.20**;
# upstream DB host defaults to the same unless you set DB_HOST (e.g. public EIP).
#
# Required env:
#   GUEPARD_DB_PASSWORD
# Optional:
#   DB_HOST          Postgres upstream + seed psql host (default: 10.0.4.20, or EIP if you export it)
#   CLIENT_HOST      Tenet client / `--client-host` (default: 10.0.4.20)
#   EIP              legacy: if set and DB_HOST unset, used as DB_HOST only
#   USE_INSTALLED_GUEPARD=1  use `guepard` from PATH instead of `cargo run` (may lack `tenet`)
#   REGION, DATACENTER, DB_PROVIDER, DB_VERSION, INSTANCE_TYPE
#   REPO_NAME        default: tenet-smoke-<epoch>
#   TENANT_ID        default: compute job name with trailing `-compute` removed
#   MASKING_SALT
#   PROXY_CONFIG     default: scripts/tenet-proxy-rules.min.yaml (repo root)
#   TENET_REQUEST_PROXY_PORT / TENET_REQUEST_API_PORT  forwarded to `tenet deploy` (Nomad static ports)
#   TENET_PROXY_PORT_OVERRIDE  if API omits proxy_port, set the host port Tenet listens on
#   DEPLOYMENT_ID    required when SKIP_CREATE=1

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DB_HOST="${DB_HOST:-${EIP:-10.0.4.20}}"
CLIENT_HOST="${CLIENT_HOST:-10.0.4.20}"
GUEPARD_DB_PASSWORD="${GUEPARD_DB_PASSWORD:?set GUEPARD_DB_PASSWORD}"
MASKING_SALT="${MASKING_SALT:-tenant-salt}"
REGION="${REGION:-us-west}"
DATACENTER="${DATACENTER:-aws}"
DB_PROVIDER="${DB_PROVIDER:-PostgreSQL}"
DB_VERSION="${DB_VERSION:-16}"
INSTANCE_TYPE="${INSTANCE_TYPE:-REPOSITORY}"
PROXY_CONFIG="${PROXY_CONFIG:-$ROOT/scripts/tenet-proxy-rules.min.yaml}"
SEED_SQL="${SEED_SQL:-$ROOT/scripts/sql/tenet_masking_seed.sql}"

need() { command -v "$1" >/dev/null 2>&1 || { echo "missing: $1" >&2; exit 1; }; }
need jq
need psql

if [[ -n "${USE_INSTALLED_GUEPARD:-}" ]]; then
  need guepard
  run_cli() { command guepard "$@"; }
else
  need cargo
  run_cli() { (cd "$ROOT" && cargo run -q -- "$@"); }
fi

poll_compute_json() {
  local dep="$1"
  run_cli compute -x "$dep" --json 2>/dev/null || true
}

wait_compute_healthy() {
  local dep="$1"
  local i
  for i in $(seq 1 120); do
    if out="$(run_cli compute status -x "$dep" --json 2>/dev/null)"; then
      st="$(echo "$out" | jq -r '.status // empty')"
      if [[ "$st" == "Healthy" ]]; then
        return 0
      fi
    fi
    sleep 5
  done
  echo "compute did not become Healthy in time" >&2
  return 1
}

wait_compute_row() {
  local dep="$1"
  local i port
  for i in $(seq 1 120); do
    j="$(poll_compute_json "$dep")"
    [[ -z "$j" ]] && { sleep 5; continue; }
    j="$(echo "$j" | jq 'if type == "array" then .[0] else . end')"
    port="$(echo "$j" | jq -r '.port // empty')"
    if [[ -n "$port" && "$port" != "null" ]]; then
      echo "$j"
      return 0
    fi
    sleep 5
  done
  echo "compute list never returned a port" >&2
  return 1
}

if [[ -n "${SKIP_CREATE:-}" ]]; then
  DEPLOYMENT_ID="${DEPLOYMENT_ID:?set DEPLOYMENT_ID when SKIP_CREATE=1}"
else
  REPO_NAME="${REPO_NAME:-tenet-smoke-$(date +%s)}"
  echo "==> Creating deployment repo=$REPO_NAME ..."
  create_json="$(run_cli deploy \
    -p "$DB_PROVIDER" -v "$DB_VERSION" -r "$REGION" -d "$DATACENTER" \
    -i "$INSTANCE_TYPE" -n "$REPO_NAME" -w "$GUEPARD_DB_PASSWORD" --json)"
  DEPLOYMENT_ID="$(echo "$create_json" | jq -r '.deployment.id')"
  [[ -n "$DEPLOYMENT_ID" && "$DEPLOYMENT_ID" != "null" ]] || { echo "$create_json" >&2; exit 1; }
  echo "    deployment_id=$DEPLOYMENT_ID"
fi

echo "==> Waiting for compute healthy ..."
run_cli compute start -x "$DEPLOYMENT_ID" --json 2>/dev/null || true
wait_compute_healthy "$DEPLOYMENT_ID"

echo "==> Reading compute row (Nomad static port) ..."
cj="$(wait_compute_row "$DEPLOYMENT_ID")"
DB_PORT="$(echo "$cj" | jq -r '.port')"
COMPUTE_NAME="$(echo "$cj" | jq -r '.name')"
[[ -n "$DB_PORT" && "$DB_PORT" != "null" ]] || { echo "$cj" >&2; exit 1; }

dep_json="$(run_cli deploy -x "$DEPLOYMENT_ID" --json)"
DB_USER="$(echo "$dep_json" | jq -r '.connection.username')"
DB_PASS="$(echo "$dep_json" | jq -r '.connection.password')"
DB_NAME="$(echo "$dep_json" | jq -r '.connection.database')"

TENANT_ID="${TENANT_ID:-${COMPUTE_NAME%-compute}}"
[[ -n "$TENANT_ID" ]] || { echo "could not derive TENANT_ID from compute name=$COMPUTE_NAME" >&2; exit 1; }

echo "    compute_job_id=$COMPUTE_NAME  tenant_id=$TENANT_ID  db_port=$DB_PORT"
echo "    DB_HOST=$DB_HOST  CLIENT_HOST=$CLIENT_HOST (Tenet client / hints)"

DIRECT_URI="postgresql://${DB_USER}:$(printf %s "$DB_PASS" | jq -sRr @uri)@${DB_HOST}:${DB_PORT}/${DB_NAME}?sslmode=require"

echo "==> Seeding (direct to DB via DB_HOST) ..."
psql "$DIRECT_URI" -v ON_ERROR_STOP=1 -f "$SEED_SQL"

echo "==> Tenet deploy ..."
tenet_deploy_args=(
  tenet deploy
  --tenant-id "$TENANT_ID"
  --compute-job-id "$COMPUTE_NAME"
  --upstream-host "$DB_HOST"
  --upstream-port "$DB_PORT"
  --masking-salt "$MASKING_SALT"
  --proxy-config "$PROXY_CONFIG"
  --client-host "$CLIENT_HOST"
  --json
)
[[ -n "${TENET_REQUEST_PROXY_PORT:-}" ]] && tenet_deploy_args+=(--proxy-port "$TENET_REQUEST_PROXY_PORT")
[[ -n "${TENET_REQUEST_API_PORT:-}" ]] && tenet_deploy_args+=(--api-port "$TENET_REQUEST_API_PORT")
tenet_json="$(run_cli "${tenet_deploy_args[@]}")"
echo "$tenet_json" | jq .
TENET_JOB="$(echo "$tenet_json" | jq -r '.job_id')"
TENET_PROXY_PORT="${TENET_PROXY_PORT_OVERRIDE:-$(echo "$tenet_json" | jq -r '.proxy_port // empty')}"

echo "==> Tenet start ..."
run_cli tenet start "$TENET_JOB" --json || true

if [[ -z "$TENET_PROXY_PORT" || "$TENET_PROXY_PORT" == "null" ]]; then
  echo "WARN: deploy JSON has no proxy_port; set TENET_PROXY_PORT_OVERRIDE or pass --proxy-port on deploy; skipping Tenet psql tests." >&2
  exit 0
fi

enc_user="$(printf %s "$DB_USER" | jq -sRr @uri)"
enc_pass="$(printf %s "$DB_PASS" | jq -sRr @uri)"

tenet_uri_disable="postgresql://${enc_user}:${enc_pass}@${CLIENT_HOST}:${TENET_PROXY_PORT}/${DB_NAME}?sslmode=disable"
tenet_uri_require="postgresql://${enc_user}:${enc_pass}@${CLIENT_HOST}:${TENET_PROXY_PORT}/${DB_NAME}?sslmode=require"

run_cfg() {
  local label="$1"
  local uri="$2"
  echo "---- psql ($label) ----"
  psql "$uri" -v ON_ERROR_STOP=1 -c "SELECT id, email, phone, secret FROM tenet_masking_test ORDER BY id LIMIT 3;" || {
    echo "FAIL: $label" >&2
    return 1
  }
}

echo "==> Multi-configuration psql checks ..."
echo "    (1) direct DB TLS"
psql "$DIRECT_URI" -v ON_ERROR_STOP=1 -c "SELECT 'direct_ok' AS probe, count(*) FROM tenet_masking_test;"

echo "    (2) via Tenet sslmode=disable (typical when proxy has no TLS)"
run_cfg "tenet-disable" "$tenet_uri_disable"

echo "    (3) via Tenet sslmode=require (fails if proxy has no server TLS — expected in some stacks)"
if run_cfg "tenet-require" "$tenet_uri_require"; then
  echo "    tenet TLS: ok"
else
  echo "    tenet TLS: skipped or failed (often expected)"
fi

echo "Done. tenet_job=$TENET_JOB deployment=$DEPLOYMENT_ID"
