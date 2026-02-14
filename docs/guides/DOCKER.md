# Docker

Back: [/docs/guides/README.md](/docs/guides/README.md)

Single-container Docker Compose workflow.

## Scope

- one compose service named `kjxlkj`
- one container running both PostgreSQL and app process
- one command startup path for operators

## Files To Rebuild

The following runtime artifacts are required for executable startup:

- `Dockerfile`
- `docker-compose.yml`
- supervisor/entrypoint script used by container startup

Use canonical template and rules from:

- [/docs/spec/architecture/deployment.md](/docs/spec/architecture/deployment.md)

## Startup

1. Reconstruct runtime artifacts from specs/TODO waves.
2. Build and start: `docker compose up --build`
3. Confirm health: `docker compose ps`
4. Confirm readiness: `curl -fsS http://127.0.0.1:8080/api/readyz`

## Shutdown and Logs

- Stop/remove: `docker compose down`
- Follow logs: `docker compose logs -f`

## Acceptance Checklist

- exactly one compose service exists
- container transitions to `healthy`
- `/api/readyz` succeeds
- graceful stop leaves no orphan DB process

## Related

- Quickstart: [QUICKSTART.md](QUICKSTART.md)
- Deployment spec: [/docs/spec/architecture/deployment.md](/docs/spec/architecture/deployment.md)
