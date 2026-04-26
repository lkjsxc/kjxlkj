# Compose Stack Contract

## Compose Files

- `docker-compose.yml` is the canonical local runtime stack file.
- `docker-compose.verify.yml` is the canonical verification overlay file.
- Compose uses literal local defaults.
- Compose does not require a local environment file.

## Services

- `postgres`: PostgreSQL database for resources, saved snapshots, settings, analytics, and sessions.
- `seaweedfs`: SeaweedFS S3 gateway for media binaries.
- `app`: Rust Axum application published as `localhost:8080`.
- `verify`: quality-gate service from the verification overlay.
- `visual-verify`: browser verification service from the verification overlay.

## Service Dependencies

- `postgres` exposes a health check through `pg_isready`.
- `seaweedfs` exposes a health check through `weed shell`.
- `app` depends on healthy `postgres` and healthy `seaweedfs`.
- `verify` depends on healthy `app`.
- `visual-verify` depends on healthy `app`.

## Runtime Environment

- Compose assembles `DATABASE_URL` for `app`.
- Compose passes literal SeaweedFS S3 settings to `app`.
- Compose passes upload byte limits and deterministic verification setup code.
- `app` listens on `0.0.0.0:8080` inside the compose network.
- Host traffic reaches the app directly on `localhost:8080`.
- External reverse proxies and external STUN/TURN services are operator infrastructure.

## Boot Behavior

1. Parse environment variables.
2. Validate database and object-storage configuration.
3. Connect to PostgreSQL.
4. Run non-destructive PostgreSQL migrations.
5. Connect to SeaweedFS S3 and ensure the target bucket exists.
6. Start the HTTP server.

## Persistent and Disposable State

- PostgreSQL state is stored in `kjxlkj-postgres-data`.
- SeaweedFS state is stored in `kjxlkj-seaweedfs-data`.
- `verify` uses `kjxlkj-verify-cargo` and `kjxlkj-verify-target`.
- Browser verification writes screenshots to `tmp/visual-artifacts/`.
- Destructive cleanup belongs only to explicit disposable verification commands such as `docker compose down -v`.
