# Docker

Back: [/docs/guides/README.md](/docs/guides/README.md)

Single-container Docker Compose workflow.

## Current Baseline Scope

- one compose service named `docs`
- one container serving repository documentation on port `8080`
- one command startup path for operators

## Startup (Current Baseline)

1. Start docs container: `docker compose up --build`
2. Open docs: `http://127.0.0.1:8080`
3. Check container status: `docker compose ps`

## Shutdown and Logs

- Stop/remove: `docker compose down`
- Follow logs: `docker compose logs -f`

## Reconstruction Target Scope

After runtime reconstruction, Compose target returns to one service named
`kjxlkj` running PostgreSQL + app process in one container (see deployment
spec).

## Related

- Quickstart: [QUICKSTART.md](QUICKSTART.md)
- Deployment spec: [/docs/spec/architecture/deployment.md](/docs/spec/architecture/deployment.md)
