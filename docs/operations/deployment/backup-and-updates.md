# Backup and Updates

## Backup Strategy

- PostgreSQL and SeaweedFS are both required runtime backup targets.
- Back up before upgrading the image or schema.
- Keep backups outside the repo checkout.

## PostgreSQL Logical Backup

```bash
mkdir -p backups
docker compose exec -T postgres \
  pg_dump -U "$POSTGRES_USER" "$POSTGRES_DB" \
  > "backups/kjxlkj-db-$(date +%Y%m%d-%H%M%S).sql"
```

## SeaweedFS Backup Rule

- Back up the full SeaweedFS data volume or run an equivalent bucket sync.
- PostgreSQL restore without matching SeaweedFS object state is incomplete for media resources and snapshots.

## Update Flow

```bash
git pull --ff-only
docker compose build app
docker compose up -d postgres seaweedfs app
docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm verify
docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm visual-verify
```

## Failure Response

- If either verification container fails, inspect logs before accepting the update.
- If the runtime stack fails to boot, inspect `docker compose logs postgres seaweedfs app`.
- If the update is rejected, restore both PostgreSQL and SeaweedFS state or redeploy a previously known-good pair.
