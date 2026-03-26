# Local Verification Runbook

## Start Services

```bash
docker compose up -d
docker compose ps
```

Expected: `postgres` and `app` are running.

## Verify Setup + Login

```bash
curl -sS -D - -o /dev/null http://127.0.0.1:8080/
curl -sS -X POST http://127.0.0.1:8080/setup \
  -H 'Content-Type: application/x-www-form-urlencoded' \
  -d 'username=admin&password=adminpass&confirm_password=adminpass'
curl -sS -X POST http://127.0.0.1:8080/login \
  -H 'Content-Type: application/x-www-form-urlencoded' \
  -d 'username=admin&password=adminpass' \
  -c cookies.txt
```

Expected:

- `/` returns the public index after setup
- setup redirects to `/login`
- login redirects to `/admin`

## Create a Note

```bash
curl -sS -X POST http://127.0.0.1:8080/records \
  -H 'Content-Type: application/json' \
  -b cookies.txt \
  -d '{}'
```

Expected: `201` with JSON body containing `id`.

## Verify Public Root Search

```bash
curl -sS 'http://127.0.0.1:8080/?q=new'
```

Expected:

- HTML contains a dense note list
- HTML does not contain the public rail shell
- HTML does not expose raw note IDs in normal list rows

## Verify Admin Dashboard Search

```bash
curl -sS 'http://127.0.0.1:8080/admin?q=new' -b cookies.txt
```

Expected:

- HTML contains admin list rows
- HTML contains search controls
- HTML uses text-style actions

## Verify Admin Note Shell

Use the returned `id` from create.

```bash
curl -sS http://127.0.0.1:8080/<id> -b cookies.txt
```

Expected:

- HTML contains `Prev` / `Next` labels
- HTML does not contain toolbar controls
- HTML does not contain helper text next to `Public`
- HTML does not render a footer history button below the note body

## Verify Browser Visual Checks

```bash
docker compose --profile verify run --rm visual-verify
```

Expected:

- desktop screenshots pass list/note assertions
- compact screenshots pass drawer assertions
- visual verification exits `0`

## Cleanup

```bash
docker compose down -v
rm -f cookies.txt
```
