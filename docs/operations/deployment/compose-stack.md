# Compose Stack Contract

## Compose Files

- `docker-compose.yml` is the canonical runtime stack file.
- `docker-compose.verify.yml` is the canonical verification overlay file.
- `.env.example` is the tracked template.
- `.env` is the active local file loaded by Compose.

## Services

- `postgres`: PostgreSQL database for notes, revisions, search, settings, analytics, and sessions
- `app`: Rust runtime service exposing `${APP_PORT:-8080}` on the host and `${BIND_PORT:-8080}` in-container
- `verify`: quality-gate service from the verification overlay
- `visual-verify`: browser verification service from the verification overlay

## Service Dependencies

- `app` depends on `postgres` being healthy.
- `verify` depends on `app` being healthy.
- `visual-verify` depends on `app` being healthy.
- Default `docker compose up` starts only `postgres` and `app`.

## Runtime Environment

- `BIND_HOST` default: `0.0.0.0`
- `BIND_PORT` default: `8080`
- `DATABASE_URL` format: `postgres://user:password@host:port/database`
- `DATABASE_URL` is required by the runtime and is assembled by Compose for the `app` service.
- `PUBLIC_BASE_URL` is optional and enables canonical URLs, `robots.txt`, `sitemap.xml`, and indexable HTML mode when valid.
- Session timeout is not an environment variable; it is loaded from `app_settings.session_timeout_minutes`.

## Boot Behavior

1. Parse environment variables.
2. Validate `DATABASE_URL` and `BIND_PORT`.
3. Connect to PostgreSQL.
4. Run database migrations.
5. Start the HTTP server.

## Persistent and Disposable State

- PostgreSQL state is stored in `kjxlkj-postgres-data`.
- `verify` uses `kjxlkj-verify-cargo` for Cargo registry cache.
- `verify` uses `kjxlkj-verify-target` for the Rust target directory.
- Browser verification writes screenshots to `tmp/visual-artifacts/`.
- The app container does not require a writable notes filesystem.
