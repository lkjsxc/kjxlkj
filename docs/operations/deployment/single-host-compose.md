# Single-Host Compose Deployment

## Canonical Scope

- The canonical deployment target is one host running Docker Compose.
- The runtime state stores are PostgreSQL plus SeaweedFS-backed object storage.

## Host Prerequisites

- Linux host with Docker Engine and Docker Compose plugin installed.
- Git installed.
- Host ports 80 and 443 available for nginx.
- Host UDP 3478 available unless `.env` changes it.
- Enough disk for `kjxlkj-postgres-data` and `kjxlkj-seaweedfs-data`.

## Prepare the Checkout

```bash
git clone <repo-url> kjxlkj
cd kjxlkj
cp .env.example .env
```

## Configure `.env`

- Set PostgreSQL credentials to deployment-specific values.
- Set SeaweedFS S3 credentials.
- Set `PUBLIC_HOST` to the final domain or public IP (used for ICE server URLs and TLS realm).
- Set `TURN_STATIC_AUTH_SECRET` to a strong random secret.
- Set `MEDIA_UPLOAD_MAX_BYTES` when media uploads should allow more or less than `536870912` bytes.
- Keep `BIND_HOST=0.0.0.0` and `BIND_PORT=8080`; the app is no longer exposed directly on the host.
- Keep `APP_PORT=8080` for internal binding only.

## Start the Runtime Stack

```bash
./scripts/generate-local-certs.sh
docker compose build app coturn nginx
docker compose up -d postgres seaweedfs app coturn nginx
docker compose ps
```

Expected:

- `postgres` is healthy.
- `seaweedfs` is healthy.
- `app` is running.

## Confirm Boot Health

```bash
curl -sS -k https://127.0.0.1/healthz
```

Expected: body `ok`.

## Run Full Verification

```bash
docker compose -f docker-compose.yml -f docker-compose.verify.yml build app coturn nginx verify visual-verify
docker compose -f docker-compose.yml -f docker-compose.verify.yml up -d postgres seaweedfs app coturn nginx
docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm verify
docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm visual-verify
docker compose -f docker-compose.yml -f docker-compose.verify.yml down -v
```

## Runtime Data Rules

- PostgreSQL and SeaweedFS are both required persistent state.
- Recreating the app container must not discard either state volume.
- Browser verification artifacts stay under `tmp/visual-artifacts/`.
