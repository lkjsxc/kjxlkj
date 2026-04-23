# Compose Quickstart

## Goal

Boot a working local `kjxlkj` instance with the smallest canonical command set.

## Prepare the Checkout

```bash
git clone <repo-url> kjxlkj
cd kjxlkj
cp .env.example .env
```

## Configure `.env`

- Set PostgreSQL credentials.
- Set SeaweedFS S3 credentials.
- Set `APP_PORT` if the host should expose something other than `8080`.
- Leave `MEDIA_UPLOAD_MAX_BYTES=536870912` unless a different upload cap is required.
- Do not look for session timeout or discovery public origin in `.env`; those live in `/admin/settings`.

## Build and Start

```bash
docker compose build app
docker compose up -d postgres seaweedfs app
docker compose ps
```

Expected:

- `postgres` becomes healthy.
- `seaweedfs` becomes healthy.
- `app` becomes running.

## Verification Bootstrap

```bash
cp .env.example .env
docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm verify
```

Expected: the verify container exits `0` after the Rust, docs, and line-limit
gates pass.

## Confirm Health

```bash
curl -sS http://127.0.0.1:${APP_PORT:-8080}/healthz
```

Expected: body `ok`.
