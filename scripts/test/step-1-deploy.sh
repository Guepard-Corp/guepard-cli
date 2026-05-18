#!/usr/bin/env bash
# Step 1 — Create deployment + seed database.
# Saves state to scripts/test/.state/deployment.env for step 2/3.
#
# Usage:
#   export GUEPARD_DB_PASSWORD=postgres
#   ./scripts/test/step-1-deploy.sh
#
# Resume (skip create):
#   SKIP_CREATE=1 DEPLOYMENT_ID=<id> DB_PORT=<port> \
#     ./scripts/test/step-1-deploy.sh
#
# Optional:
#   GUEPARD_DB_ADMIN_PASSWORD=<pw>  — auto-grant CREATE on public if needed
#   REPO_NAME=my-test-name          — override generated name

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=scripts/test/env.sh
source "$SCRIPT_DIR/env.sh"

need jq
need psql
command -v "$GUEPARD_BIN" >/dev/null 2>&1 || die "guepard binary not found: $GUEPARD_BIN"

trap restore_dev_api EXIT INT TERM
patch_dev_api

# ── Auth check ───────────────────────────────────────────────────────────────
section "Auth"
auth_out="$(require_json "usage" usage)"
deps="$(echo "$auth_out" | jq -r '[.[]|select(.resource=="Deployments")][0]|"\(.used)/\(.quota)"' 2>/dev/null || true)"
ok "logged in (deployments ${deps:-?})"

# ── Create or reuse deployment ───────────────────────────────────────────────
if [[ -n "${SKIP_CREATE:-}" ]]; then
  DEPLOYMENT_ID="${DEPLOYMENT_ID:?set DEPLOYMENT_ID when SKIP_CREATE=1}"
  section "Using existing deployment $DEPLOYMENT_ID"
else
  REPO_NAME="${REPO_NAME:-tenet-test-$(date +%s)}"
  section "Create deployment ($REPO_NAME)"
  create_json="$(require_json "deploy create" deploy \
    -p "$DB_PROVIDER" -v "$DB_VERSION" -r "$REGION" -d "$DATACENTER" \
    -i "$INSTANCE_TYPE" -n "$REPO_NAME" -w "$GUEPARD_DB_PASSWORD")"
  DEPLOYMENT_ID="$(echo "$create_json" | jq -er '.deployment.id')"
  ok "deployment_id=$DEPLOYMENT_ID"

  # Port often in create response
  if [[ -z "$DB_PORT" ]]; then
    DB_PORT="$(echo "$create_json" | jq -r 'if .compute.port != null then .compute.port|tostring else empty end')"
  fi
  # Tenant slug from create response
  TENANT_ID="$(echo "$create_json" | jq -r '.deployment.name // empty')"
fi

# ── Compute start + wait Healthy ─────────────────────────────────────────────
section "Compute start"
start_out="$(run_guepard_json compute start -x "$DEPLOYMENT_ID" 2>/dev/null || true)"
[[ -n "$start_out" ]] && info "compute start: $(echo "$start_out" | jq -c . 2>/dev/null || echo "$start_out")"

info "waiting for compute Healthy…"
for i in $(seq 1 120); do
  if out="$(run_guepard_json compute status -x "$DEPLOYMENT_ID" 2>/dev/null)"; then
    st="$(echo "$out" | jq -r '.status // empty')"
    [[ "$st" == "Healthy" ]] && { ok "compute Healthy"; break; }
    [[ $((i % 6)) -eq 0 ]] && info "status=$st (${i}s)"
  fi
  sleep 5
  [[ "$i" -eq 120 ]] && die "compute did not become Healthy in time"
done

# ── Resolve tenant slug + DB port ────────────────────────────────────────────
section "Resolve tenant + DB port"
if [[ -z "${TENANT_ID:-}" || -z "$DB_PORT" ]]; then
  dep_json="$(try_json "deploy -x" deploy -x "$DEPLOYMENT_ID" 2>/dev/null || true)"
  if [[ -n "$dep_json" ]]; then
    [[ -z "${TENANT_ID:-}" ]] && TENANT_ID="$(echo "$dep_json" | jq -r '.deployment.name // empty')"
    [[ -z "$DB_PORT" ]]       && DB_PORT="$(echo "$dep_json"   | jq -r 'if .compute.port != null then .compute.port|tostring else empty end')"
    DB_USER="$(echo "$dep_json" | jq -r '.connection.username // "guepard"')"
    DB_PASS="$(echo "$dep_json" | jq -r '.connection.password // "postgres"')"
  fi
fi
[[ -n "$DB_PORT" ]] || die "DB_PORT unknown — check compute API or pass DB_PORT=<port>"
[[ -n "${TENANT_ID:-}" ]] || die "could not derive TENANT_ID from create/deploy -x"

DB_USER="${DB_USER:-guepard}"
DB_PASS="${DB_PASS:-postgres}"
DB_NAME="${DATABASE_NAME:-postgres}"
COMPUTE_JOB_ID="${TENANT_ID}-compute"
ok "tenant=$TENANT_ID  compute=$COMPUTE_JOB_ID  db=${DB_HOST}:${DB_PORT}/${DB_NAME}"

# ── Seed database ─────────────────────────────────────────────────────────────
section "Seed DB"
URI="$(psql_uri "$DB_NAME")"

# Check if tables already exist
if psql "$URI" -tAc "SELECT 1 FROM customers LIMIT 1" 2>/dev/null | grep -q 1; then
  ok "seed tables already present"
else
  if ! psql "$URI" -v ON_ERROR_STOP=1 -f "$SEED_SQL" 2>/tmp/seed.err; then
    if grep -q 'permission denied for schema public' /tmp/seed.err; then
      if [[ -n "${GUEPARD_DB_ADMIN_PASSWORD:-}" ]]; then
        ADMIN="${GUEPARD_DB_ADMIN_USER:-guepard-admin}"
        ADMIN_URI="$(printf 'postgresql://%s:%s@%s:%s/%s?sslmode=require' \
          "$ADMIN" "$(printf %s "$GUEPARD_DB_ADMIN_PASSWORD" | jq -sRr @uri)" \
          "$DB_HOST" "$DB_PORT" "$DB_NAME")"
        info "granting CREATE on public via $ADMIN"
        psql "$ADMIN_URI" -c "GRANT CREATE ON SCHEMA public TO guepard;"
        psql "$URI" -v ON_ERROR_STOP=1 -f "$SEED_SQL"
        ok "seed applied (after admin grant)"
      else
        die "permission denied for schema public — set GUEPARD_DB_ADMIN_PASSWORD or run:
  psql -U guepard-admin -d postgres -c \"GRANT CREATE ON SCHEMA public TO guepard;\""
      fi
    else
      die "seed failed: $(tr '\n' ' ' </tmp/seed.err | head -c 300)"
    fi
  else
    ok "seed applied"
  fi
fi

# Verify
row_counts="$(psql "$URI" -tAc "SELECT 'customers:'||count(*) FROM customers UNION ALL SELECT 'users:'||count(*) FROM users UNION ALL SELECT 'identities:'||count(*) FROM identities;" 2>/dev/null | tr '\n' ' ')"
info "rows: $row_counts"

# ── Save state ────────────────────────────────────────────────────────────────
section "Save state"
save_state
cat "$STATE_FILE"

section "Done"
ok "deployment=$DEPLOYMENT_ID ready for step-2"
info "Next: ./scripts/test/step-2-tenet.sh"
