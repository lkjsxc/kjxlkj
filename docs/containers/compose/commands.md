# Compose Commands

## Preconditions

- Run commands from repository root (same directory as `docker-compose.yml`).
- Docker Engine and Docker Compose v2 must be available.
- Host port `8080` must be free.
- Host bind mounts must remain under `./data/`.

## First-Time Local Startup (Deterministic)

### 1) Prepare local mount directories

```bash
mkdir -p data/postgres data/content
```

Expected result:

- Exit code `0`.
- Both directories exist.

### 2) Validate compose configuration

```bash
docker compose config --quiet
```

Expected result:

- Exit code `0`.
- No output (configuration is valid).

### 3) Build the app image

```bash
docker compose build app
```

Expected result:

- Exit code `0`.
- App image builds from repository `Dockerfile`.

### 4) Start default services only

```bash
docker compose up -d postgres app
docker compose ps
```

Expected result:

- Exit code `0`.
- `postgres` is running (and becomes healthy).
- `app` is running.
- `verify` is not started.

### 5) Confirm database readiness

```bash
docker compose exec postgres pg_isready -U kjxlkj -d kjxlkj
```

Expected result includes `accepting connections`.

### 6) Confirm setup-first readiness

```bash
curl -sS -D - -o /dev/null http://127.0.0.1:8080/ \
  | tr -d '\r' \
  | awk 'NR==1 || tolower($1)=="location:"'

curl -sS http://127.0.0.1:8080/setup \
  | grep -E '<title>Initial setup</title>|<form method="post" action="/setup">'
```

Expected result:

- Home request returns `302 Found` with `location: /setup`.
- Setup response contains both HTML markers.

### 7) Confirm app liveness endpoint

```bash
curl -sS -D - -o /dev/null http://127.0.0.1:8080/healthz \
  | tr -d '\r' \
  | awk 'NR==1'
```

Expected result:

- `HTTP/1.1 200 OK`.
- Used by compose-level health checks to avoid false-positive "running but unreachable" states.

## Functional Verification Handoff

- Continue with [../verification/local-runbook.md](../verification/local-runbook.md) for deterministic setup/login/admin checks and recovery procedures.

## Docker Acceptance Path

1. Complete first-time startup.
2. Complete the local verification runbook.
3. Run acceptance checks:

```bash
docker compose --profile verify run --rm verify
```

4. Optional JSON wrapper:

```bash
cargo run --bin kjxlkj -- compose verify
```

Acceptance completion criteria:

- Functional runbook checks pass.
- Verify profile exits `0`.
- No quality gate reports a failure.

## Stop/Cleanup

```bash
docker compose down
```

## Contract

- Development flow MUST prebuild the app image from `Dockerfile` before `up`.
- `docker compose up` MUST NOT start `verify`.
- Command outputs and exit codes are consumed by automation.
- Non-zero exit code indicates contract failure.
