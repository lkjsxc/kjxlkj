# Deployment Model

Back: [/docs/spec/architecture/README.md](/docs/spec/architecture/README.md)

## Single-Container Compose Contract

Deployment MUST use one Docker Compose service that runs both:

- PostgreSQL process
- `kjxlkj` application process

This shape is mandatory for baseline operations and local-first rebuild.

## Process Supervision Contract

Container entrypoint MUST:

1. initialize PostgreSQL data directory if missing
2. start PostgreSQL and wait for readiness
3. run SQL migrations
4. start application server
5. forward termination signals and stop both processes cleanly

## Compose Template (Normative)

```yaml
services:
  kjxlkj:
    build:
      context: .
      dockerfile: Dockerfile
    container_name: kjxlkj
    ports:
      - "8080:8080"
    environment:
      DATABASE_URL: ${DATABASE_URL}
      POSTGRES_DATA_DIR: /var/lib/postgresql/data
      KJXLKJ_CONFIG_PATH: /app/data/config.json
    volumes:
      - kjxlkj_pg:/var/lib/postgresql/data
    healthcheck:
      test: ["CMD-SHELL", "curl -fsS http://127.0.0.1:8080/api/readyz || exit 1"]
      interval: 10s
      timeout: 3s
      retries: 12
      start_period: 30s
    restart: unless-stopped

volumes:
  kjxlkj_pg:
```

## Storage Layout

| Path | Purpose |
|---|---|
| `/var/lib/postgresql/data` | PostgreSQL persistent data |
| `/app/static` | built SPA assets |
| `/app/data/config.json` | non-secret runtime configuration |

## Health Rules

- `/api/healthz` verifies application liveness.
- `/api/readyz` verifies DB connectivity and migration compatibility.
- Compose healthcheck MUST use `/api/readyz`.

## Rebuild Acceptance

Single-container deployment is accepted only when all pass:

1. `docker compose up --build` starts exactly one service
2. service health becomes `healthy`
3. `/api/readyz` returns success
4. shutdown (`docker compose down`) exits cleanly without orphan DB process

## Related

- Docker guide: [/docs/guides/DOCKER.md](/docs/guides/DOCKER.md)
- Runtime model: [runtime.md](runtime.md)
- Configuration contract: [configuration.md](configuration.md)
- Operations: [/docs/spec/technical/operations.md](/docs/spec/technical/operations.md)
