# Local Verification Runbook

## Start Services

```bash
docker compose up -d postgres minio app
docker compose ps
```

Expected: `postgres`, `minio`, and `app` are running.

## Verify Setup + Login

```bash
curl -sS -D - -o /dev/null http://127.0.0.1:8080/
curl -sS -X POST http://127.0.0.1:8080/setup \
  -H 'Content-Type: application/x-www-form-urlencoded' \
  -d 'username=admin&password=adminpass&confirm_password=adminpass&setup_code=<console-code>'
curl -sS -X POST http://127.0.0.1:8080/login \
  -H 'Content-Type: application/x-www-form-urlencoded' \
  -d 'username=admin&password=adminpass' \
  -c cookies.txt
```

## Create a Note

```bash
curl -sS -X POST http://127.0.0.1:8080/resources/notes \
  -H 'Content-Type: application/json' \
  -b cookies.txt \
  -d '{"body":"# Launch Notes\n\n![](/demo-image/file)\n","is_private":false}'
```

## Create Media

```bash
curl -sS -X POST http://127.0.0.1:8080/resources/media \
  -b cookies.txt \
  -F 'file=@./tmp/demo-image.png' \
  -F 'alias=demo-image' \
  -F 'is_private=false'
```

## Verify Resource Pages

- `/search` can filter `kind=all|note|media`.
- Admin rails show `New note`, then `Open GitHub`.
- Guest image media pages show the image plus rendered Markdown body.
- Image media pages may serve WebP display variants while preserving the original file URL.
- Guest note pages render inline images from `![](/demo-image/file)`.
- Guest or admin video media pages expose a playable `<video>` element.
- `/demo-image/file` returns the binary and respects visibility.
- `/demo-image/file?variant=card` returns WebP when a card variant exists.

## Verify History + Immutability

- Updating note or media Markdown creates a new saved snapshot.
- `/{snapshot_id}/file` serves the older binary.
- Existing media pages do not expose a file replacement control.
- Admin history pages distinguish the live resource from immutable saved snapshots.
- Guest readers may open known public saved-snapshot URLs but not history index pages.

## Verify Browser Visual Checks

```bash
docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm visual-verify
```

## Cleanup

```bash
docker compose -f docker-compose.yml -f docker-compose.verify.yml down -v
rm -f cookies.txt
```
