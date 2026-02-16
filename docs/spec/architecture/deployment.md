# Deployment Model

Back: [/docs/spec/architecture/README.md](/docs/spec/architecture/README.md)

## Compose Contract

Baseline deployment uses a single compose service containing:

- PostgreSQL process
- application process

## Entrypoint Contract

1. init DB directory if missing
2. start DB and wait for readiness
3. run migrations
4. start app server
5. forward shutdown signals

## Required Artifacts

- `Dockerfile`
- `docker-compose.yml`
- `.dockerignore`
- `scripts/entrypoint.sh`
- `scripts/backup-restore-drill.sh`

## Related

- Runtime model: [runtime.md](runtime.md)
- Operations: [/docs/spec/technical/operations.md](/docs/spec/technical/operations.md)
