# Compose Service Map

## App Service

- Hosts web application runtime.
- MUST be built from repository `Dockerfile` as documented in [build-storage-contract.md](build-storage-contract.md).
- Uses data mounts under `./data/` only when host mounts are required.

## PostgreSQL Service

- Persists admin and session relational state.
- Durable host mount path is `./data/postgres`.

## Verify Service

- Runs quality checks as a deterministic verification bundle.
- MUST be opt-in through the `verify` profile and never auto-start in default `docker compose up`.
