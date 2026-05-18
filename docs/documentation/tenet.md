# Tenet (transparent DB proxy)

This page describes **Tenet from the Guepard CLI perspective**. Tenet itself is a separate project: transparent PostgreSQL/MySQL proxy that **masks PII in result sets** before data reaches clients (AI tools, apps, BI).

**Upstream Tenet project:** [guepard-tenet](https://github.com/Guepard-Corp/guepard-tenet) — README, `proxy.example.yaml`, and full docs (`docs/04-configuration.md`, `docs/05-transformers.md`, Nomad packaging, etc.).

---

## What the CLI does

The CLI **does not run** the Tenet binary locally for production traffic. It calls the **Guepard API** (`config.api_url`), which schedules Tenet on **Nomad** next to your compute job.

| CLI responsibility | Tenet server responsibility |
|---------------------|-----------------------------|
| `tenet deploy` — upload `proxy.yaml`, upstream, salt, optional ports | Speak wire protocol, forward queries, apply rules to **results** |
| `tenet start` / `stop` / `purge` — lifecycle | Health, metrics, optional management API |
| `tenet proxy get` / `set` — fetch or replace YAML on a running job | Reload / apply rules per API |

Examples below use the **`guepard`** binary (Homebrew, direct download, or `cargo install`). **From a clone of this repo** you can run the same commands with `cargo run -q -- …` instead of `guepard` (no install step).

---

## Quick usage

### 1. Deploy Tenet (after compute exists)

You need a **running compute** job for the deployment (Nomad job id is usually `<deployment-name>-compute`). Get **host** and **port** from `guepard deploy -x <id> --json` → `connection` / `compute`, or from Nomad.

```bash
guepard tenet deploy \
  --tenant-id <deployment-name> \
  --compute-job-id <deployment-name>-compute \
  --upstream-host <postgres-host> \
  --upstream-port <postgres-port> \
  --masking-salt <unique-secret-per-tenant> \
  --proxy-config ./proxy.yaml \
  --client-host <address-clients-use-to-reach-tenet> \
  --json
```

- **`--tenant-id`:** Stable id for this Tenet (often the deployment **name** slug).
- **`--upstream-host` / `--upstream-port`:** Where Tenet connects **as a client** to Postgres (private IP, FQDN, or EIP — whatever routing allows from the Tenet task).
- **`--masking-salt`:** Passed into the scheduled job (e.g. `TENET_MASKING_SALT`). Same value + same cell → deterministic masking; change salt → different masked output. See [guepard-tenet masking salt](https://github.com/Guepard-Corp/guepard-tenet/blob/main/docs/04-configuration.md#masking-salt-per-tenant).
- **`--client-host`:** Used for **printed hints** and your own notes: the host **your users** put in `psql`/drivers (VPC IP, EIP, or LB). It does **not** open AWS security groups; you must allow **Tenet’s proxy port** inbound the same way you allow the DB port.
- **`--proxy-config`:** UTF-8 file path; CLI sends contents as `config_yaml` to the API (CLI does not validate YAML).

Omit **`--proxy-port`** / **`--api-port`** for **dynamic** Nomad ports (typical). Pass them only when you need **fixed** host ports (debugging, firewall rules). Read **`proxy_port`** / **`api_port`** from deploy **`--json`** or from Nomad allocation **ReservedPorts**.

### 2. Lifecycle

```bash
guepard tenet start <job_id> --json
guepard tenet stop <job_id> --json
guepard tenet purge <job_id> --json
```

`job_id` is returned by **`tenet deploy --json`** as `job_id` (e.g. `<tenant>-tenet`). If **`start`** returns **404** while Nomad already shows the task **running**, you can often ignore it and connect with `psql` anyway.

### 3. Change rules without full redeploy

```bash
guepard tenet proxy set <job_id> --proxy-config ./proxy-new.yaml
guepard tenet proxy get <job_id> -o ./proxy-backup.yaml
```

---

## `proxy.yaml` (rules)

Full reference: **guepard-tenet** [Configuration](https://github.com/Guepard-Corp/guepard-tenet/blob/main/docs/04-configuration.md) and [Transformers](https://github.com/Guepard-Corp/guepard-tenet/blob/main/docs/05-transformers.md).

Minimal PostgreSQL example (legacy **`strategy`** style, from [proxy.example.yaml](https://github.com/Guepard-Corp/guepard-tenet/blob/main/proxy.example.yaml)):

```yaml
masking_enabled: true
protocol: postgres
rules:
  - table: users
    column: email
    strategy: email
  - table: users
    column: phone
    strategy: phone
```

Modern rule shape uses **`transformer`** + optional **`options`** (see guepard-tenet docs). Example:

```yaml
rules:
  - table: users
    column: notes
    transformer: redacted
    options:
      character: "#"
      width: 12
```

**Strategies / transformers** include `email`, `phone`, `hash`, `redacted`, `partial_mask`, `json`, and many more — see guepard-tenet **05-transformers**.

Optional sections in the same file: **`safety.blocked_patterns`**, **`limits`**, **`upstream_tls`**, **`tls`** (client-facing), etc.

This repo ships small samples under **`scripts/`**:

| File | Purpose |
|------|---------|
| `scripts/tenet-proxy-rules.min.yaml` | Simple redact rules for `tenet_masking_test` |
| `scripts/tenet-proxy-rules-alt-*.yaml` | Variations for experiments / `proxy set` |
| `scripts/sql/tenet_masking_seed.sql` | Seed table + row for demos |
| `scripts/tenet-e2e-from-deploy.sh` | Scripted flow: deploy → compute → seed → tenet (`cargo run` by default; set `USE_INSTALLED_GUEPARD=1` for `guepard` on `PATH`) |

---

## Connecting with `psql`

- **Direct to Postgres:** use `connection_uri` / host+port from **`guepard deploy -x … --json`**, usually **`sslmode=require`** if the DB uses TLS.
- **Through Tenet:** use **`host` = client-reachable address** (EIP, LB, or VPN into VPC) and **`port` = Tenet proxy port** (from API JSON or Nomad). If Tenet has no TLS on the proxy port, use **`sslmode=disable`**.

Example:

```bash
export PGPASSWORD='<guepard-db-password>'
psql "host=<client-host> port=<tenet-proxy-port> user=guepard dbname=postgres sslmode=disable" \
  -c "SELECT id, email FROM sensitive_table LIMIT 5;"
```

---

## Troubleshooting

| Symptom | Likely cause |
|---------|----------------|
| **`502`** on `deploy` / `compute` / `tenet` | Guepard API / gateway; retry later, escalate with request id if any. |
| **`compute start` 502** but **`deploy -x` OK** | Partial API outage on compute routes. |
| **`tenet start` 404** | Scheduler idempotency / job already running — verify Nomad alloc. |
| **Hang on `psql` to EIP:tenet-port** | Security group / NACL / no DNAT to host where Tenet listens. Open same path as for Postgres host port. |
| **Plaintext through Tenet** | Rules don’t match **table/column** names; wrong DB in connection string; or `masking_enabled: false`. |

---

## See also

- [Complete command reference](commands.md) — `tenet` flags table and copy-paste examples
- [Deployments](deployments.md) — creating deployments and connection info
- [Compute](compute-commands.md) — starting compute, status, ports
