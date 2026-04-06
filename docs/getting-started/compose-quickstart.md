# Compose Quickstart

## Goal

Boot a working local `kjxlkj` instance with the smallest canonical command set.

## Prerequisites

- Linux host
- Docker Engine
- Docker Compose plugin
- Git

## Prepare the Checkout

```bash
git clone <repo-url> kjxlkj
cd kjxlkj
cp .env.example .env
```

## Configure `.env`

- Set `POSTGRES_DB`, `POSTGRES_USER`, and `POSTGRES_PASSWORD`.
- Set `APP_PORT` if the host should expose something other than `8080`.
- Keep `BIND_HOST=0.0.0.0` unless the service must bind locally only.
- Do not look for session timeout in `.env`; it is stored in `app_settings`.
- Do not look for public-origin SEO configuration in `.env`; `public_base_url` is stored in `app_settings` and edited from `/admin/settings`.

## Build and Start

```bash
docker compose build app
docker compose up -d postgres app
docker compose ps
```

Expected:

- `postgres` becomes healthy.
- `app` becomes running.
- The named PostgreSQL volume is exposed as `kjxlkj-postgres-data`.

## Confirm Health

```bash
curl -sS http://127.0.0.1:${APP_PORT:-8080}/healthz
```

Expected: body `ok`.

## Next Step

- Continue with [first-session.md](first-session.md) for setup and initial live-use checks.
- Continue with [verification.md](verification.md) for the full compose acceptance flow.
- Use [../operations/deployment/single-host-compose.md](../operations/deployment/single-host-compose.md) for the deeper deployment reference.
- Use [../operations/deployment/runtime-configuration.md](../operations/deployment/runtime-configuration.md) for the split between compose env and persisted operator settings.
