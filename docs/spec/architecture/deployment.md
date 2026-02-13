# Deployment Model

Back: [/docs/spec/architecture/README.md](/docs/spec/architecture/README.md)

## Current Baseline: Documentation Container

When Docker artifacts are generated in All in Docs baseline mode, Compose runs
one `docs` service.

### Baseline Compose Template

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

1. `docker compose up --build` starts exactly one service
2. `http://127.0.0.1:8080` serves docs content
3. `docker compose down` exits cleanly

## Reconstruction Target: Single-Container Runtime

After reconstruction, deployment target remains one compose service running:

- PostgreSQL process
- Rust application process

Typed frontend assets are served by the same app origin.

## Process Supervision Contract (Runtime Target)

Container entrypoint MUST:

1. initialize PostgreSQL data directory if missing
2. start PostgreSQL and wait for readiness
3. run migrations
4. start application server
5. forward termination signals and stop processes cleanly

## Runtime Health Rules

- `/api/healthz` verifies app liveness
- `/api/readyz` verifies DB connectivity and migration compatibility
- compose healthcheck MUST use `/api/readyz`

## Related

- Docker guide: [/docs/guides/DOCKER.md](/docs/guides/DOCKER.md)
- Runtime model: [runtime.md](runtime.md)
- Type safety: [/docs/spec/technical/type-safety.md](/docs/spec/technical/type-safety.md)
