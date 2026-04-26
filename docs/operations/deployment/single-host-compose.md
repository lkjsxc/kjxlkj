# Single-Host Compose Deployment

## Canonical Scope

- The canonical repo-owned runtime target is one local host running Docker Compose.
- The runtime state stores are PostgreSQL plus SeaweedFS-backed object storage.
- External proxies, TLS, domains, and STUN/TURN services are outside the repo-owned compose stack.

## Host Prerequisites

- Linux host with Docker Engine and Docker Compose plugin installed.
- Git installed.
- Host TCP port `8080` available.
- Enough disk for `kjxlkj-postgres-data` and `kjxlkj-seaweedfs-data`.

## Prepare the Checkout

```bash
git clone <repo-url> kjxlkj
cd kjxlkj
```

## Start the Runtime Stack

```bash
docker compose build app
docker compose up -d postgres seaweedfs app
docker compose ps
```

Expected:

- `postgres` is healthy.
- `seaweedfs` is healthy.
- `app` is healthy.
- The app answers on `http://localhost:8080`.

## Confirm Boot Health

```bash
curl -sS http://127.0.0.1:8080/healthz
```

Expected: body `ok`.

## Run Full Verification

```bash
docker compose -f docker-compose.yml -f docker-compose.verify.yml build app verify visual-verify
docker compose -f docker-compose.yml -f docker-compose.verify.yml up -d postgres seaweedfs app
docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm verify
docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm visual-verify
docker compose -f docker-compose.yml -f docker-compose.verify.yml down -v
```

## Runtime Data Rules

- PostgreSQL and SeaweedFS are both required persistent state.
- Recreating the app container must not discard either state volume.
- Browser verification artifacts stay under `tmp/visual-artifacts/`.
