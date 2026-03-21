# Compose Service Map

## Scope

- This map applies to services defined in repository-root `docker-compose.yml`.
- Build and storage rules are canonical in [build-storage-contract.md](build-storage-contract.md).

## App Service

- Hosts web application runtime.
- MUST be built from repository `Dockerfile` with prebuild flow (`docker compose build app` before `docker compose up`).
- Uses host mounts under `./data/` only when host mounts are required.
- SHOULD consume image-baked migrations instead of compose migration mounts.

## PostgreSQL Service

- Persists admin and session relational state.
- Durable host mount path is `./data/postgres`.

## Verify Service

- Runs quality checks as a deterministic verification bundle.
- MUST be opt-in through the `verify` profile and never auto-start in default `docker compose up`.
