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
- Set `PUBLIC_HOST` to `localhost` for local testing or your real domain for deployment.
- Set `TURN_STATIC_AUTH_SECRET` to a random string.
- Leave `MEDIA_UPLOAD_MAX_BYTES=536870912` unless a different upload cap is required.
- Do not look for session timeout or discovery public origin in `.env`; those live in `/admin/settings`.

## Build and Start

```bash
./scripts/generate-local-certs.sh
docker compose build app coturn nginx
docker compose up -d postgres seaweedfs app coturn nginx
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
curl -sS -k https://127.0.0.1/healthz
```

Expected: body `ok`.
