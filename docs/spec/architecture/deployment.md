# Deployment Model

Back: [/docs/spec/architecture/README.md](/docs/spec/architecture/README.md)

## Current Baseline: App Runtime Container

When Docker artifacts are generated in current baseline mode, Compose runs one
app service.

### Baseline Artifact Set

Required root artifacts:

- `Dockerfile`
- `docker-compose.yml`
- `.dockerignore`

### Baseline Compose Template

```yaml
services:
  kjxlkj:
    build:
      context: .
      dockerfile: Dockerfile
    container_name: kjxlkj-app
    environment:
      BIND_ADDRESS: 0.0.0.0
      PORT: "8080"
      DATABASE_URL: sqlite:/data/kjxlkj.db?mode=rwc
      JWT_SECRET: ${JWT_SECRET:-dev-secret-change-in-production}
    ports:
      - "8080:8080"
    volumes:
      - kjxlkj-data:/data
    healthcheck:
      test: ["CMD", "curl", "-fsS", "http://127.0.0.1:8080/api/readyz"]
      interval: 10s
      timeout: 3s
      retries: 5
      start_period: 5s
    restart: unless-stopped

volumes:
  kjxlkj-data:
```

### Baseline Acceptance

1. `docker compose up -d --build` starts exactly one service (`kjxlkj`)
2. `GET /api/healthz` returns `200`
3. `GET /api/readyz` returns `200` after migrations complete
4. `docker compose down` exits cleanly

## Runtime Health Rules

- `/api/healthz` verifies app liveness
- `/api/readyz` verifies DB connectivity and migration compatibility
- compose healthcheck MUST use `/api/readyz`

## Related

- Docker guide: [/docs/guides/DOCKER.md](/docs/guides/DOCKER.md)
- Runtime model: [runtime.md](runtime.md)
- Type safety: [/docs/spec/technical/type-safety.md](/docs/spec/technical/type-safety.md)
