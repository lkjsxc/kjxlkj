# Docker

Back: [/docs/guides/README.md](/docs/guides/README.md)

How to regenerate Docker artifacts from canonical docs.

## Baseline Scope

- Docker files are intentionally absent in current baseline.
- Docker artifacts are derived outputs and may be regenerated when needed.

## Regeneration Steps (Docs Container)

1. Generate root `Dockerfile` and `docker-compose.yml` from deployment/docs specs.
2. Build/start: `docker compose up --build`
3. Verify docs endpoint and health.

## Shutdown and Logs

- Stop/remove: `docker compose down`
- Follow logs: `docker compose logs -f`

## Reconstruction Target Scope

After runtime reconstruction, target deployment remains one service named
`kjxlkj` running PostgreSQL + Rust app with typed frontend assets.

## Related

- Quickstart: [QUICKSTART.md](QUICKSTART.md)
- Deployment spec: [/docs/spec/architecture/deployment.md](/docs/spec/architecture/deployment.md)
