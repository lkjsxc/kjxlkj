# Compose Service Contract

## Compose Files

- `docker-compose.yml`: runtime services only.
- `docker-compose.verify.yml`: verification services only.

## Services

- `postgres`: PostgreSQL database for auth/sessions.
- `app`: runtime service exposing port `8080`.
- `verify`: quality verification service from the verification overlay.
- `visual-verify`: browser screenshot verification service from the verification overlay.

## Service Dependencies

- `app` depends on `postgres` being healthy
- `verify` depends on `app` being healthy
- `visual-verify` depends on `app` being healthy

## Postgres Service

- Image: `postgres:16-alpine`
- Port: `5432` (internal only by default)
- Volume: `kjxlkj-postgres-data:/var/lib/postgresql/data`
- Healthcheck: `pg_isready`

## Verification Volumes

- `verify` uses `kjxlkj-verify-cargo` for the Cargo registry cache.
- `verify` uses `kjxlkj-verify-target` for the Rust target dir.

## Start Rule

- Default `docker compose up` starts only `postgres` and `app`.
- Verification services start only when the overlay file is included.
- Health checks remain part of the canonical compose contract because the app runs migrations at boot and the verification overlay reuses the same readiness model.
