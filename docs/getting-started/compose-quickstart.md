# Compose Quickstart

## Goal

Boot a working local `kjxlkj` instance with the smallest canonical command set.

## Prepare the Checkout

```bash
git clone <repo-url> kjxlkj
cd kjxlkj
```

## Build and Start

```bash
docker compose build app
docker compose up -d postgres seaweedfs app
docker compose ps
```

Expected:

- `postgres` becomes healthy.
- `seaweedfs` becomes healthy.
- `app` becomes healthy.
- The app is reachable at `http://localhost:8080`.

## Verification Bootstrap

```bash
docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm verify
```

Expected: the verify container exits `0` after Rust, docs, and line-limit gates pass.

## Confirm Health

```bash
curl -sS http://127.0.0.1:8080/healthz
```

Expected: body `ok`.

## Configuration Boundary

- Compose local defaults are already encoded in `docker-compose.yml`.
- Do not create `.env` for the canonical compose flow.
- Use `/admin/settings` for site identity, public origin, discovery, and live ICE settings.
- Use direct app environment variables only for non-compose runs.
