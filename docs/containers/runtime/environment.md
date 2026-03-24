# Runtime Environment Contract

## Required Variables

| Variable | Required | Default | Description |
|----------|----------|---------|-------------|
| `BIND_HOST` | No | `0.0.0.0` | Listen address |
| `BIND_PORT` | No | `8080` | Listen port |
| `DATA_ROOT` | No | `/app/data` | Record storage root |
| `DATABASE_URL` | Yes | - | PostgreSQL connection DSN |
| `ADMIN_TOKEN` | Yes | - | API write token |
| `SESSION_TIMEOUT_MINUTES` | No | `1440` | Session duration (clamped 5-10080) |

## Variable Formats

### DATABASE_URL

```
postgres://user:password@host:port/database
```

Example: `postgres://kjxlkj:secret@postgres:5432/kjxlkj`

### ADMIN_TOKEN

- Minimum length: 16 characters
- Recommended: random 32+ character string

## Boot Behavior

1. Parse and validate all environment variables
2. Fail fast if required variables missing
3. Fail fast if DATABASE_URL format invalid
4. Fail fast if PostgreSQL connection fails
5. Run database migrations
6. Start HTTP server

## Validation Rules

- `SESSION_TIMEOUT_MINUTES`: clamped to range [5, 10080]
- `BIND_PORT`: must be valid port (1-65535)
- `DATA_ROOT`: created if not exists
