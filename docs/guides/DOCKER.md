# Docker

Back: [/docs/guides/README.md](/docs/guides/README.md)

Single-service container workflow.

## Scope

`Dockerfile` and `docker-compose.yml` are derived artifacts.

When present, the expected model is one compose service with:

- PostgreSQL process
- `kjxlkj` app process

## Runtime Commands

- Build/start: `docker compose up --build`
- Stop: `docker compose down`
- Tail logs: `docker compose logs -f`

## Health

Container healthcheck should pass via `/api/v1/readyz`.

## Related

- Deployment spec: [/docs/spec/architecture/deployment.md](/docs/spec/architecture/deployment.md)
- Operations spec: [/docs/spec/technical/operations.md](/docs/spec/technical/operations.md)
