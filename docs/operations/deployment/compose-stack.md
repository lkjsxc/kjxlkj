# Compose Stack Contract

## Compose Files

- `docker-compose.yml` is the canonical runtime stack file.
- `docker-compose.verify.yml` is the canonical verification overlay file.
- `.env.example` is the tracked template.
- `.env` is the active local file loaded by Compose.

## Services

- `postgres`: PostgreSQL database for resources, snapshots, settings, analytics, and sessions
- `seaweedfs`: SeaweedFS S3 gateway for media binaries
- `app`: Rust runtime service with HTTP and live ICE UDP listeners
- `verify`: quality-gate service from the verification overlay
- `visual-verify`: browser verification service from the verification overlay

## Service Dependencies

- `app` depends on healthy `postgres` and healthy `seaweedfs`.
- `verify` depends on healthy `app`.
- `visual-verify` depends on healthy `app`.
- Default `docker compose up` starts `postgres`, `seaweedfs`, and `app`.

## Runtime Environment

- `.env` owns PostgreSQL credentials, app host exposure, live ICE exposure, SeaweedFS S3 credentials, endpoint settings, and upload limits.
- Compose assembles `DATABASE_URL` and the SeaweedFS S3 environment for `app`.
- Runtime Compose does not expose SeaweedFS ports on the host.
- Runtime Compose exposes the configured live ICE UDP port on the host.
- Runtime Compose passes public, LAN, and trusted-proxy live relay settings to `app`.
- Runtime Compose does not terminate public TLS.
- Runtime Compose does not own public nginx.
- Production edge nginx is external infrastructure.
- Live media uses the app-owned ICE UDP port instead of external TURN.
- Persisted operator settings still own `site_name`, `site_description`, `public_base_url`, search defaults, and session timeout.

## Boot Behavior

1. Parse environment variables.
2. Validate database and object-storage configuration.
3. Connect to PostgreSQL.
4. Run non-destructive PostgreSQL migrations.
5. Connect to SeaweedFS S3 and ensure the target bucket exists.
6. Bind the live ICE UDP listener.
7. Start the HTTP server.

## Migration Rule

- Startup migrations may create missing schema objects and indexes.
- Startup migrations must not drop active runtime tables or truncate runtime data.
- Destructive cleanup belongs only to explicit disposable verification commands such as `docker compose down -v`.

## Persistent and Disposable State

- PostgreSQL state is stored in `kjxlkj-postgres-data`.
- SeaweedFS state is stored in `kjxlkj-seaweedfs-data`.
- `verify` uses `kjxlkj-verify-cargo` and `kjxlkj-verify-target`.
- Browser verification writes screenshots to `tmp/visual-artifacts/`.
