#!/usr/bin/env bash
# Step 2 — Deploy 3 Tenet instances (tenants A/B/C) with different proxy rules.
# Reads deployment state from step-1; saves tenet job IDs to state.
#
# KNOWN LIMIT (api.dev scheduler): job_id is always {compute-slug}-tenet (e.g. fierce-forest-psfzcc-tenet),
# not {slug}-a-tenet / -b-tenet / -c-tenet. Each deploy overwrites the same Nomad job; only the last
# proxy config (tenant C) survives. Step 3 detects this and skips invalid multi-tenant assertions.
# Re-run after the platform supports one Tenet job per profile / distinct job ids.
#
# Usage:
#   ./scripts/test/step-2-tenet.sh
#
# Optional:
#   MASKING_SALT=my-salt         — deterministic masking salt
#   TENET_BASE_PROXY_PORT=30100  — starting host port (auto-increments on conflict)
#   KEEP_EXISTING=1              — skip deploy if job already registered

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=scripts/test/env.sh
source "$SCRIPT_DIR/env.sh"

need jq

# ── Load state from step 1 ────────────────────────────────────────────────────
load_state

MASKING_SALT="${MASKING_SALT:-tenet-test-salt-$(date +%s)}"
COMPUTE_JOB_ID="${TENANT_ID}-compute"

# ── Port allocation (each tenant needs proxy + api port) ─────────────────────
BASE="${TENET_BASE_PROXY_PORT}"

# Try to deploy a tenet job; auto-increment ports on "already reserved"
deploy_tenant() {
  local name="$1" proxy_yaml="$2" proxy_port="$3" api_port="$4"
  local tenant_suffix="${TENANT_ID}-${name}"
  local out attempt max="$TENET_PORT_MAX"

  for attempt in $(seq 1 "$max"); do
    info "  deploy $name ($tenant_suffix): proxy=$proxy_port api=$api_port (attempt $attempt)" >&2
    if out="$("$GUEPARD_BIN" tenet deploy \
        --tenant-id "$tenant_suffix" \
        --compute-job-id "$COMPUTE_JOB_ID" \
        --upstream-host "$DB_HOST" \
        --upstream-port "$DB_PORT" \
        --masking-salt "$MASKING_SALT" \
        --proxy-config "$proxy_yaml" \
        --client-host "$CLIENT_HOST" \
        --proxy-port "$proxy_port" \
        --api-port "$api_port" \
        --json 2>/tmp/g.err)"; then
      printf '%s' "$out"
      return 0
    fi
    if grep -qE 'already reserved|Static port' /tmp/g.err 2>/dev/null \
      && [[ "$attempt" -lt "$max" ]]; then
      warn "    port $proxy_port or $api_port taken — trying next" >&2
      proxy_port=$((proxy_port + 2))
      api_port=$((api_port + 2))
      continue
    fi
    return 1
  done
  return 1
}

# ── Deploy tenant A (redact-all) ──────────────────────────────────────────────
section "Tenant A — redact all PII"
PROXY_A="$PROXY_DIR/tenant-a-redact-all.yaml"
PROXY_PORT_A="$((BASE))"
API_PORT_A="$((BASE + 1))"

JOB_A_EXPECTED="${TENANT_ID}-a-tenet"
if [[ -n "${KEEP_EXISTING:-}" ]] \
   && "$GUEPARD_BIN" tenet proxy get "$JOB_A_EXPECTED" --json >/dev/null 2>&1; then
  ok "tenant-a already registered (KEEP_EXISTING=1)"
  JOB_A="$JOB_A_EXPECTED"
else
  out_a="$(deploy_tenant "a" "$PROXY_A" "$PROXY_PORT_A" "$API_PORT_A")" \
    || die "tenant-a deploy failed: $(tr '\n' ' ' </tmp/g.err | head -c 300)"
  JOB_A="$(printf '%s' "$out_a" | jq -er '.job_id')"
  PROXY_PORT_A="$(echo "$out_a" | jq -r 'if .proxy_port != null then .proxy_port|tostring else empty end')"
  [[ -z "$PROXY_PORT_A" ]] && PROXY_PORT_A="$((BASE))"
  ok "job_id=$JOB_A  proxy=${CLIENT_HOST}:${PROXY_PORT_A}"
fi

# ── Deploy tenant B (partial masking) ─────────────────────────────────────────
section "Tenant B — partial masking (email + CC only)"
PROXY_B="$PROXY_DIR/tenant-b-partial.yaml"
PROXY_PORT_B="$((BASE + 2))"
API_PORT_B="$((BASE + 3))"

JOB_B_EXPECTED="${TENANT_ID}-b-tenet"
if [[ -n "${KEEP_EXISTING:-}" ]] \
   && "$GUEPARD_BIN" tenet proxy get "$JOB_B_EXPECTED" --json >/dev/null 2>&1; then
  ok "tenant-b already registered (KEEP_EXISTING=1)"
  JOB_B="$JOB_B_EXPECTED"
else
  out_b="$(deploy_tenant "b" "$PROXY_B" "$PROXY_PORT_B" "$API_PORT_B")" \
    || die "tenant-b deploy failed: $(tr '\n' ' ' </tmp/g.err | head -c 300)"
  JOB_B="$(printf '%s' "$out_b" | jq -er '.job_id')"
  PROXY_PORT_B="$(echo "$out_b" | jq -r 'if .proxy_port != null then .proxy_port|tostring else empty end')"
  [[ -z "$PROXY_PORT_B" ]] && PROXY_PORT_B="$((BASE + 2))"
  ok "job_id=$JOB_B  proxy=${CLIENT_HOST}:${PROXY_PORT_B}"
fi

# ── Deploy tenant C (passthrough) ─────────────────────────────────────────────
section "Tenant C — passthrough (masking disabled)"
PROXY_C="$PROXY_DIR/tenant-c-passthrough.yaml"
PROXY_PORT_C="$((BASE + 4))"
API_PORT_C="$((BASE + 5))"

JOB_C_EXPECTED="${TENANT_ID}-c-tenet"
if [[ -n "${KEEP_EXISTING:-}" ]] \
   && "$GUEPARD_BIN" tenet proxy get "$JOB_C_EXPECTED" --json >/dev/null 2>&1; then
  ok "tenant-c already registered (KEEP_EXISTING=1)"
  JOB_C="$JOB_C_EXPECTED"
else
  out_c="$(deploy_tenant "c" "$PROXY_C" "$PROXY_PORT_C" "$API_PORT_C")" \
    || die "tenant-c deploy failed: $(tr '\n' ' ' </tmp/g.err | head -c 300)"
  JOB_C="$(printf '%s' "$out_c" | jq -er '.job_id')"
  PROXY_PORT_C="$(echo "$out_c" | jq -r 'if .proxy_port != null then .proxy_port|tostring else empty end')"
  [[ -z "$PROXY_PORT_C" ]] && PROXY_PORT_C="$((BASE + 4))"
  ok "job_id=$JOB_C  proxy=${CLIENT_HOST}:${PROXY_PORT_C}"
fi

# ── Scheduler collision check ─────────────────────────────────────────────────
section "Job ID check"
EXPECTED_SINGLE_JOB="${TENANT_ID}-tenet"
TENET_MULTI_JOB=1
if [[ "$JOB_A" == "$JOB_B" && "$JOB_B" == "$JOB_C" ]]; then
  TENET_MULTI_JOB=0
  warn "all three deploys returned the same job_id=$JOB_A"
  warn "scheduler names the job ${EXPECTED_SINGLE_JOB} (compute slug + tenet) — B/C overwrote A"
  warn "step-3 will run CLI tests once; multi-tenant masking assertions are skipped until fixed"
  # Last deploy wins for ports on the single allocation
  PROXY_PORT_A="$PROXY_PORT_C"
  PROXY_PORT_B="$PROXY_PORT_C"
fi
[[ "$JOB_A" == "$EXPECTED_SINGLE_JOB" ]] && info "nomad job name matches scheduler pattern: $EXPECTED_SINGLE_JOB" >&2

# ── Save extended state ───────────────────────────────────────────────────────
section "Save state"
cat >> "$STATE_FILE" <<EOF
JOB_A="${JOB_A}"
PROXY_PORT_A="${PROXY_PORT_A}"
JOB_B="${JOB_B}"
PROXY_PORT_B="${PROXY_PORT_B}"
JOB_C="${JOB_C}"
PROXY_PORT_C="${PROXY_PORT_C}"
MASKING_SALT="${MASKING_SALT}"
TENET_MULTI_JOB="${TENET_MULTI_JOB}"
EOF
info "state → $STATE_FILE" >&2

section "Done"
if [[ "$TENET_MULTI_JOB" -eq 1 ]]; then
  ok "3 distinct tenet jobs deployed"
else
  ok "tenet job registered (single shared job — multi-profile test blocked on scheduler)"
fi
info "  A ($JOB_A): proxy ${CLIENT_HOST}:${PROXY_PORT_A}  — redact all"
info "  B ($JOB_B): proxy ${CLIENT_HOST}:${PROXY_PORT_B}  — partial"
info "  C ($JOB_C): proxy ${CLIENT_HOST}:${PROXY_PORT_C}  — passthrough"
info "Next: ./scripts/test/step-3-test.sh"
