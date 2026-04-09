# Single-Host Compose Deployment

## Canonical Scope

- The canonical deployment target is one host running Docker Compose.
- The runtime state stores are PostgreSQL plus MinIO-backed object storage.

## Host Prerequisites

- Linux host with Docker Engine and Docker Compose plugin installed.
- Git installed.
- Host ports for app and MinIO available unless `.env` changes them.
- Enough disk for `kjxlkj-postgres-data` and `kjxlkj-minio-data`.

## Prepare the Checkout

```bash
git clone <repo-url> kjxlkj
cd kjxlkj
cp .env.example .env
```

## Configure `.env`

- Set PostgreSQL credentials to deployment-specific values.
- Set MinIO root credentials and app-facing S3 credentials.
- Set `APP_PORT` if the host should expose something other than `8080`.
- Keep `BIND_HOST=0.0.0.0` unless the host should only bind locally.

## Start the Runtime Stack

```bash
docker compose build app
docker compose up -d postgres minio app
docker compose ps
```

Expected:

- `postgres` is healthy.
- `minio` is healthy.
- `app` is running.

## Confirm Boot Health

```bash
curl -sS http://127.0.0.1:${APP_PORT:-8080}/healthz
```

Expected: body `ok`.

## Run Full Verification

```bash
docker compose -f docker-compose.yml -f docker-compose.verify.yml build app verify visual-verify
docker compose -f docker-compose.yml -f docker-compose.verify.yml up -d postgres minio app
docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm verify
docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm visual-verify
docker compose -f docker-compose.yml -f docker-compose.verify.yml down -v
```

## Runtime Data Rules

- PostgreSQL and MinIO are both required persistent state.
- Recreating the app container must not discard either state volume.
- Browser verification artifacts stay under `tmp/visual-artifacts/`.
