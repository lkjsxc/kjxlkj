# Compose Stack Contract

## Compose Files

- `docker-compose.yml` is the canonical runtime stack file.
- `docker-compose.verify.yml` is the canonical verification overlay file.
- `.env.example` is the tracked template.
- `.env` is the active local file loaded by Compose.

## Services

- `postgres`: PostgreSQL database for resources, snapshots, settings, analytics, and sessions
- `minio`: S3-compatible object storage for media binaries
- `app`: Rust runtime service
- `verify`: quality-gate service from the verification overlay
- `visual-verify`: browser verification service from the verification overlay

## Service Dependencies

- `app` depends on healthy `postgres` and healthy `minio`.
- `verify` depends on healthy `app`.
- `visual-verify` depends on healthy `app`.
- Default `docker compose up` starts `postgres`, `minio`, and `app`.

## Runtime Environment

- `.env` owns PostgreSQL credentials, app host exposure, MinIO credentials, and S3 endpoint settings.
- Compose assembles `DATABASE_URL` and the S3-compatible environment for `app`.
- Runtime Compose does not expose MinIO ports on the host.
- Persisted operator settings still own `site_name`, `site_description`, `public_base_url`, search defaults, and session timeout.

## Boot Behavior

1. Parse environment variables.
2. Validate database and object-storage configuration.
3. Connect to PostgreSQL and run migrations.
4. Connect to object storage and ensure the target bucket exists.
5. Start the HTTP server.

## Persistent and Disposable State

- PostgreSQL state is stored in `kjxlkj-postgres-data`.
- MinIO state is stored in `kjxlkj-minio-data`.
- `verify` uses `kjxlkj-verify-cargo` and `kjxlkj-verify-target`.
- Browser verification writes screenshots to `tmp/visual-artifacts/`.
