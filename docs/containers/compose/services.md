# Compose Service Contract

## Services

- `postgres`: PostgreSQL database for auth/sessions.
- `app`: runtime service exposing port `8080`.
- `verify`: profile-gated quality verification service.
- `visual-verify`: profile-gated browser screenshot verification service.

## Service Dependencies

- `app` depends on `postgres` being healthy
- `verify` depends on `app` being healthy

## Postgres Service

- Image: `postgres:16-alpine`
- Port: `5432` (internal only by default)
- Volume: `pgdata:/var/lib/postgresql/data`
- Healthcheck: `pg_isready`

## Profile Rule

- Default `docker compose up` starts `postgres` and `app`.
- `verify` and `visual-verify` run only with `--profile verify`.
