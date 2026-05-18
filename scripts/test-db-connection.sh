#!/usr/bin/env bash
# Test Postgres reachability for a deployment (api.dev or prod — uses ~/.guepard config).
#
# Usage:
#   ./scripts/test-db-connection.sh <deployment-id> [nomad-port]
#   DB_PORT=28027 PGPASSWORD=postgres ./scripts/test-db-connection.sh <deployment-id>
#
# When `compute -x` 502s, pass the port from a prior run or Nomad (not deploy -x :5432).
#
# Default host is dev EIP (port/db/user from API). Override:
#   DB_HOST=10.0.4.20 ./scripts/test-db-connection.sh <deployment-id>
#
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
GUEPARD_BIN="${GUEPARD_BIN:-}"
[[ -z "$GUEPARD_BIN" && -x "$ROOT/target/release/guepard" ]] && GUEPARD_BIN="$ROOT/target/release/guepard"
GUEPARD_BIN="${GUEPARD_BIN:-guepard}"

DEPLOYMENT_ID="${1:?usage: $0 <deployment-id> [nomad-port]}"
DB_PORT="${DB_PORT:-${2:-}}"
TCP_TIMEOUT="${TCP_TIMEOUT:-5}"
DEV_DB_EIP="${DEV_DB_EIP:-44.239.18.139}"
DB_HOST="${DB_HOST:-${EIP:-$DEV_DB_EIP}}"

need() { command -v "$1" >/dev/null || { echo "missing: $1" >&2; exit 1; }; }
need jq
command -v "$GUEPARD_BIN" >/dev/null || exit 1

section() { printf '\n\033[1m── %s ──\033[0m\n' "$*"; }
ok() { printf '  \033[32m✓\033[0m %s\n' "$*"; }
info() { printf '  \033[36m·\033[0m %s\n' "$*"; }
warn() { printf '  \033[33m!\033[0m %s\n' "$*"; }
fail() { printf '  \033[31m✗\033[0m %s\n' "$*" >&2; }

port_from_deploy_compute() {
  [[ -z "${dep:-}" ]] && return 1
  echo "$dep" | jq -r 'if .compute and .compute.port then (.compute.port | tostring) else empty end'
}

section "API: deploy -x"
if ! dep="$( "$GUEPARD_BIN" deploy -x "$DEPLOYMENT_ID" --json 2>/tmp/guepard-deploy.err)"; then
  fail "deploy -x failed: $(tr '\n' ' ' </tmp/guepard-deploy.err | head -c 300)"
  dep=""
else
  echo "$dep" | jq '{connection, deployment: {id: .deployment.id, name: .deployment.name, repository_name: .deployment.repository_name}}'
fi

section "API: compute (Nomad port)"
comp=""
for attempt in $(seq 1 "${GUEPARD_E2E_RETRIES:-8}"); do
  if comp="$( "$GUEPARD_BIN" compute -x "$DEPLOYMENT_ID" --json 2>/tmp/guepard-compute.err)"; then
    echo "$comp" | jq .
    break
  fi
  if grep -qE '502|503|504|Bad Gateway' /tmp/guepard-compute.err 2>/dev/null && [[ "$attempt" -lt 8 ]]; then
    echo "  retry compute ($attempt/8)…" >&2
    sleep "${GUEPARD_E2E_RETRY_WAIT:-5}"
    continue
  fi
  warn "compute list failed: $(tr '\n' ' ' </tmp/guepard-compute.err | head -c 200)"
  comp="[]"
  break
done

if [[ -z "$comp" || "$comp" == "[]" ]]; then
  st="$( "$GUEPARD_BIN" compute status -x "$DEPLOYMENT_ID" --json 2>/dev/null || echo '{}')"
  echo "  compute status: $(echo "$st" | jq -c . 2>/dev/null || echo "$st")"
fi

# Credentials from deploy -x; host=EIP; port from compute (NOT deploy -x :5432 placeholder)
USER="${DB_USER:-guepard}"
PASS="${PGPASSWORD:-${GUEPARD_DB_PASSWORD:-}}"
DB="${DATABASE_NAME:-}"
URI=""
if [[ -n "$dep" ]]; then
  DB="$(echo "$dep" | jq -r '.connection.database // .deployment.repository_name // empty')"
  USER="$(echo "$dep" | jq -r '.connection.username // empty')"
  PASS="$(echo "$dep" | jq -r '.connection.password // empty')"
  URI="$(echo "$dep" | jq -r '.connection.connection_uri // empty')"
fi

COMPUTE_PORT=""
if [[ -n "$comp" && "$comp" != "[]" ]]; then
  row="$(echo "$comp" | jq 'if type == "array" then .[0] else . end')"
  COMPUTE_PORT="$(echo "$row" | jq -r '.port // empty')"
fi
DEPLOY_COMPUTE_PORT="$(port_from_deploy_compute 2>/dev/null || true)"

PORT="${DB_PORT:-${COMPUTE_PORT:-${DEPLOY_COMPUTE_PORT:-}}}"
HOST="$DB_HOST"

if [[ -z "$PORT" || "$PORT" == "null" ]]; then
  fail "no Nomad port (compute API 502?). Pass it explicitly:"
  echo "    DB_PORT=28027 $0 $DEPLOYMENT_ID" >&2
  echo "    $0 $DEPLOYMENT_ID 28027" >&2
  echo "  Get port when compute works: $GUEPARD_BIN compute -x $DEPLOYMENT_ID --json | jq '.[0].port'" >&2
  exit 1
fi
[[ -n "$DB_PORT" || -n "${2:-}" ]] && info "using port $PORT (manual override)"
if [[ -z "$DB" ]]; then
  fail "set DATABASE_NAME or fix deploy -x"
  exit 1
fi

section "Connection string"
echo "  postgresql://${USER}:****@${HOST}:${PORT}/${DB}?sslmode=require"
if [[ -n "$URI" ]]; then
  echo "  (deploy -x URI is FQDN:5432 placeholder — ignored for port; using compute/Nomad port $PORT)"
fi
echo "  host=$HOST port=$PORT db=$DB user=$USER"
getent hosts "$HOST" 2>/dev/null | sed 's/^/  DNS: /' || true

section "TCP probe (${TCP_TIMEOUT}s)"
if timeout "$TCP_TIMEOUT" bash -c "echo >/dev/tcp/$HOST/$PORT" 2>/dev/null; then
  ok "port $HOST:$PORT reachable"
else
  fail "port $HOST:$PORT not reachable from this machine (timeout/firewall)"
  echo "  Try another DB_HOST (default EIP: $DEV_DB_EIP): DB_HOST=10.0.4.20 $0 $DEPLOYMENT_ID"
  echo "  Or VPN/bastion, or run psql from a host inside the same network as Nomad."
fi

section "psql"
PASS="${PGPASSWORD:-$PASS}"
if [[ -z "$PASS" ]]; then
  echo "  Set PGPASSWORD or GUEPARD_DB_PASSWORD to run psql"
  exit 0
fi

export PGPASSWORD="$PASS"
SSL="${PGSSLMODE:-require}"
enc_user="$(printf %s "$USER" | jq -sRr @uri)"
enc_pass="$(printf %s "$PASS" | jq -sRr @uri)"
PSQL_URI="postgresql://${enc_user}:${enc_pass}@${HOST}:${PORT}/${DB}?sslmode=${SSL}"

echo "  psql \"\$PSQL_URI\" -c 'SELECT 1 AS ok'"
if psql "$PSQL_URI" -v ON_ERROR_STOP=1 -c 'SELECT 1 AS ok, current_database(), inet_server_addr();'; then
  ok "psql connected"
else
  fail "psql failed (TCP blocked or wrong password/sslmode — try PGSSLMODE=disable)"
  exit 1
fi
