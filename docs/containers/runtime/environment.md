# Runtime Environment Contract

## Required Variables

- `BIND_HOST` default: `0.0.0.0`
- `BIND_PORT` default: `8080`
- `DATA_ROOT` default: `/app/data`
- `DATABASE_URL` required PostgreSQL DSN for setup/login/session persistence
- `ADMIN_TOKEN` required for write operations
- `SESSION_TIMEOUT_MINUTES` optional; clamped to `5..10080` (default `1440`)

## Boot Rule

App process fails fast when required configuration is invalid or when PostgreSQL is unavailable.
