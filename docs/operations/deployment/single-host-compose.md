# Single-Host Compose Deployment

## Canonical Scope

- The canonical deployment target is one host running Docker Compose.
- Use [compose-stack.md](compose-stack.md) for the detailed runtime contract.
- Use this page for the exact deployment sequence.

## Host Prerequisites

- Linux host with Docker Engine and Docker Compose plugin installed.
- Git installed.
- Port `8080` available on the host unless `.env` changes `APP_PORT`.
- Enough disk for the named PostgreSQL volume `kjxlkj-postgres-data`.

## Prepare the Checkout

```bash
git clone <repo-url> kjxlkj
cd kjxlkj
cp .env.example .env
```

## Configure `.env`

- Set `POSTGRES_DB`, `POSTGRES_USER`, and `POSTGRES_PASSWORD` to deployment-specific values.
- Set `APP_PORT` if the host should expose something other than `8080`.
- Keep `BIND_HOST=0.0.0.0` unless the host should only bind locally.
- Do not look for `SESSION_TIMEOUT_MINUTES` in compose; that setting lives in `/admin/settings`.
- `DATABASE_URL` is constructed by Compose for the `app` service and should not be hand-authored in `.env`.

## Build the Runtime Stack

```bash
docker compose build app
```

## Start the Runtime Stack

```bash
docker compose up -d postgres app
docker compose ps
```

Expected:

- `postgres` is healthy.
- `app` is running.
- Compose readiness continues to rely on health checks rather than shell sleeps.
- PostgreSQL state is stored in the named volume `kjxlkj-postgres-data`.

## Confirm Boot Health

```bash
curl -sS http://127.0.0.1:${APP_PORT:-8080}/healthz
```

Expected: body `ok`.

## Run Full Verification

```bash
docker compose -f docker-compose.yml -f docker-compose.verify.yml build app verify visual-verify
docker compose -f docker-compose.yml -f docker-compose.verify.yml up -d postgres app
docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm verify
docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm visual-verify
docker compose -f docker-compose.yml -f docker-compose.verify.yml down -v
```

Expected:

- `verify` exits `0` only when Rust, docs, and line-limit checks pass.
- `visual-verify` exits `0` only when browser-rendered desktop and compact screenshots pass.
- The verification bundle removes PostgreSQL state because it ends with `down -v`.

## Resume Runtime After Verification

```bash
docker compose up -d postgres app
```

## Runtime Data Rules

- PostgreSQL is the only persistent runtime state store.
- The app container does not require a writable notes filesystem.
- Browser verification artifacts stay under `tmp/visual-artifacts/`.
- Recreating the app container must not discard PostgreSQL state as long as the named volume is preserved.

## First Operator Handoff

- Continue with [first-login-and-live-use.md](first-login-and-live-use.md).
- The first live admin configures the session timeout from `/admin/settings`.
- Use [../verification/compose-pipeline.md](../verification/compose-pipeline.md) for the deeper acceptance contract.
