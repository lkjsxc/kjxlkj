# Docker

Back: [/docs/guides/README.md](/docs/guides/README.md)

Deterministic Docker artifact contract for the app runtime.

## Required Root Artifacts

These files MUST exist at repository root:

- `Dockerfile`
- `docker-compose.yml`
- `.dockerignore`

If any file is missing, regenerate it before marking TODO completion.

## Baseline Contract (App Runtime)

- Compose runs exactly one service named `kjxlkj`.
- Container name is `kjxlkj-app`.
- Host port mapping is `8080:8080`.
- Healthcheck probes `http://127.0.0.1:8080/api/readyz`.
- SQLite persistence is mounted at `/data` with named volume `kjxlkj-data`.
- `docker compose config` must pass with no schema errors.

## Regeneration Steps (App Runtime)

1. Regenerate root files to match [/docs/spec/architecture/deployment.md](/docs/spec/architecture/deployment.md).
2. Validate config: `docker compose config`.
3. Build/start: `docker compose up -d --build`.
4. Verify liveness: `curl -fsS http://127.0.0.1:8080/api/healthz`.
5. Verify readiness: `curl -fsS http://127.0.0.1:8080/api/readyz`.
6. Verify health status: `docker compose ps`.
7. Stop/remove: `docker compose down`.

## Implementation Reference

`Dockerfile` MUST:

- use a deterministic base image
- build and run `kjxlkj-server` from workspace sources
- run container process as non-root user
- expose `8080`
- define healthcheck against `/api/readyz`

`docker-compose.yml` MUST:

- define a single `kjxlkj` service
- build from root `Dockerfile`
- include `restart: unless-stopped`
- include persistent `/data` volume for SQLite
- include HTTP healthcheck for `/api/readyz`

## Shutdown and Logs

- Follow logs: `docker compose logs -f`

## Reconstruction Target Scope

After runtime reconstruction, target deployment remains one service named
`kjxlkj` running Rust app components with typed frontend assets.

## Related

- Quickstart: [QUICKSTART.md](QUICKSTART.md)
- Deployment spec: [/docs/spec/architecture/deployment.md](/docs/spec/architecture/deployment.md)
