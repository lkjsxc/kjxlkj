# Deployment Model

Back: [/docs/spec/architecture/README.md](/docs/spec/architecture/README.md)

## Current Baseline: Documentation Container

In the docs-only reconstruction baseline, Docker Compose MUST run exactly one
service that serves repository documentation.

### Baseline Compose Template (Normative for current repo state)

```yaml
services:
  docs:
    build:
      context: .
      dockerfile: Dockerfile
    container_name: kjxlkj-docs
    ports:
      - "8080:8080"
    restart: unless-stopped
```

### Baseline Acceptance

1. `docker compose up --build` starts exactly one service.
2. `http://127.0.0.1:8080` serves repository docs content.
3. `docker compose down` exits cleanly.

## Reconstruction Target: Single-Container Runtime

Deployment target after reconstruction remains one Docker Compose service that
runs both:

- PostgreSQL process
- `kjxlkj` application process

This shape is mandatory for baseline operations and local-first rebuild.

## Process Supervision Contract (Runtime Target)

Container entrypoint MUST:

1. initialize PostgreSQL data directory if missing
2. start PostgreSQL and wait for readiness
3. run SQL migrations
4. start application server
5. forward termination signals and stop both processes cleanly

## Runtime Compose Template (Target)

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
      KJXLKJ_BIND_ADDR: 0.0.0.0:8080
      POSTGRES_DATA_DIR: /var/lib/postgresql/data
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

## Storage Layout (Runtime Target)

| Path | Purpose |
|---|---|
| `/var/lib/postgresql/data` | PostgreSQL persistent data |
| `/app/static` | built SPA assets |
| `/app/config` | runtime configuration (optional mount) |

## Health Rules (Runtime Target)

- `/api/healthz` verifies application liveness.
- `/api/readyz` verifies DB connectivity and migration compatibility.
- Compose healthcheck MUST use `/api/readyz`.

## Related

- Docker guide: [/docs/guides/DOCKER.md](/docs/guides/DOCKER.md)
- Runtime model: [runtime.md](runtime.md)
- Operations: [/docs/spec/technical/operations.md](/docs/spec/technical/operations.md)
