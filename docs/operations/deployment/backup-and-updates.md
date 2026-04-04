# Backup and Updates

## Backup Strategy

- PostgreSQL is the only required runtime backup target.
- Back up before upgrading the image or the schema.
- Keep backups outside the repo checkout.

## Logical Backup

```bash
mkdir -p backups
docker compose exec -T postgres \
  pg_dump -U "$POSTGRES_USER" "$POSTGRES_DB" \
  > "backups/kjxlkj-$(date +%Y%m%d-%H%M%S).sql"
```

## Restore Rule

- Stop the runtime stack before destructive restore work.
- Restore into the same database named in `.env`.
- Restart the app and re-run verification after restore.

## Update Flow

```bash
git pull --ff-only
docker compose build app
docker compose up -d postgres app
docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm verify
docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm visual-verify
```

## Failure Response

- If either verification container fails, inspect logs before accepting the update.
- If the runtime stack fails to boot, inspect `docker compose logs postgres app`.
- If the update is rejected, restore from backup or redeploy the previously known-good image.

## Shutdown

```bash
docker compose down
```

Use `docker compose down -v` only when intentionally removing PostgreSQL state.
