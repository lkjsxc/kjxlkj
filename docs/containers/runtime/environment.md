# Runtime Environment Contract

## Required Variables

| Variable | Required | Default | Description |
|----------|----------|---------|-------------|
| `BIND_HOST` | No | `0.0.0.0` | Listen address |
| `BIND_PORT` | No | `8080` | Listen port |
| `DATABASE_URL` | Yes | - | PostgreSQL connection DSN |

## Variable Formats

### DATABASE_URL

```
postgres://user:password@host:port/database
```

Example: `postgres://kjxlkj:secret@postgres:5432/kjxlkj`

## Boot Behavior

1. Parse and validate all environment variables
2. Fail fast if required variables missing
3. Fail fast if DATABASE_URL format invalid
4. Fail fast if PostgreSQL connection fails
5. Run database migrations
6. Start HTTP server

## Settings Rule

- Session timeout is not an environment variable.
- Session timeout is loaded from `app_settings.session_timeout_minutes` during login.
- The untouched runtime default is `1440` minutes.

## Validation Rules

- `BIND_PORT`: must be valid port (1-65535)
