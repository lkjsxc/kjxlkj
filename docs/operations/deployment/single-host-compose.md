# Single-Host Compose Deployment

## Canonical Scope

- The canonical deployment target is one host running Docker Compose.
- The runtime state stores are PostgreSQL plus SeaweedFS-backed object storage.

## Host Prerequisites

- Linux host with Docker Engine and Docker Compose plugin installed.
- Git installed.
- Host app port available unless `.env` changes it.
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
- Set `MEDIA_UPLOAD_MAX_BYTES` when media uploads should allow more or less than `536870912` bytes.
- Set `APP_PORT` if the host should expose something other than `8080`.
- Set `LIVE_ICE_UDP_PORT` if the host should expose live WebRTC on something other than `8189/udp`.
- Set `LIVE_ICE_PUBLIC_IPS` when Docker or NAT hides the public relay address.
- Set `LIVE_ICE_LAN_IPS` when LAN clients need a different relay address.
- Set `LIVE_TRUSTED_PROXY_IPS` when `/live/ws` is behind a trusted reverse proxy.
- Keep `BIND_HOST=0.0.0.0` unless the host should only bind locally.

## Start the Runtime Stack

```bash
docker compose build app
docker compose up -d postgres seaweedfs app
docker compose ps
```

Expected:

- `postgres` is healthy.
- `seaweedfs` is healthy.
- `app` is running.
- The configured live ICE UDP port is published for remote `/live` media.

## Confirm Boot Health

```bash
curl -sS http://127.0.0.1:${APP_PORT:-8080}/healthz
```

Expected: body `ok`.

## Run Full Verification

```bash
docker compose -f docker-compose.yml -f docker-compose.verify.yml build app verify visual-verify
docker compose -f docker-compose.yml -f docker-compose.verify.yml up -d postgres seaweedfs app
docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm verify
docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm visual-verify
docker compose -f docker-compose.yml -f docker-compose.verify.yml down -v
```

## Runtime Data Rules

- PostgreSQL and SeaweedFS are both required persistent state.
- Recreating the app container must not discard either state volume.
- Browser verification artifacts stay under `tmp/visual-artifacts/`.
