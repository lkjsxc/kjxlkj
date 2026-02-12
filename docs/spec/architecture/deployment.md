# Deployment Model

Back: [/docs/spec/architecture/README.md](/docs/spec/architecture/README.md)

## Single-Service Container Contract

Deployment MUST use one compose service that runs both:

- PostgreSQL process
- `kjxlkj` application process

## Process Supervision

The container image MUST include a supervisor or equivalent launcher that:

1. initializes PostgreSQL data directory
2. starts PostgreSQL and waits for readiness
3. runs SQL migrations
4. starts the Actix application
5. propagates termination signals to both processes

## Storage Layout

| Path | Purpose |
|---|---|
| `/var/lib/postgresql/data` | PostgreSQL persistent data |
| `/app/static` | built SPA assets |
| `/app/config` | runtime configuration (optional mount) |

## Health Rules

- `/healthz` verifies app process liveness.
- `/readyz` verifies DB connectivity and migration compatibility.
- Compose healthcheck MUST use `/readyz`.

## Related

- Docker operator guide: [/docs/guides/DOCKER.md](/docs/guides/DOCKER.md)
- Runtime model: [runtime.md](runtime.md)
