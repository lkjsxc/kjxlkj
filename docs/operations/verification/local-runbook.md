# Local Verification Runbook

## Start Services

```bash
docker compose up -d
docker compose ps
```

Expected: `postgres` and `app` are running.

## Verify Postgres Health

```bash
docker compose exec postgres pg_isready
```

Expected: `/var/run/postgresql:5432 - accepting connections`

## Verify Health

```bash
curl -sS -D - -o /dev/null http://127.0.0.1:8080/healthz
```

Expected: `HTTP/1.1 200 OK` and body `ok`.

## Verify Setup Flow (Fresh Instance)

```bash
curl -sS -D - -o /dev/null http://127.0.0.1:8080/
curl -sS -D - -o /dev/null http://127.0.0.1:8080/setup
curl -sS -D - -o /dev/null http://127.0.0.1:8080/login
```

Expected before setup:

- `/` redirects to `/setup`
- `/setup` returns `200` HTML
- `/login` redirects to `/setup`

## Complete Setup

```bash
curl -sS -X POST http://127.0.0.1:8080/setup \
  -H 'Content-Type: application/x-www-form-urlencoded' \
  -d 'username=admin&password=adminpass&confirm_password=adminpass'
```

Expected: `303` redirect to `/login`.

## Login

```bash
curl -sS -X POST http://127.0.0.1:8080/login \
  -H 'Content-Type: application/x-www-form-urlencoded' \
  -d 'username=admin&password=adminpass' \
  -c cookies.txt
```

Expected: `303` redirect to `/admin`, sets `session_id` cookie.

## Create a Note

```bash
curl -sS -X POST http://127.0.0.1:8080/records \
  -H 'Content-Type: application/json' \
  -b cookies.txt \
  -d '{}'
```

Expected: `201` with JSON body containing `slug`.

## Verify Guest Note Shell

Use the returned slug from the previous step.

```bash
curl -sS http://127.0.0.1:8080/<slug>
```

Expected after making the note public:

- HTML contains previous/next shell slots
- HTML contains a history link
- HTML does not contain editor controls for guests

## Verify Admin Note Shell

```bash
curl -sS http://127.0.0.1:8080/<slug> -b cookies.txt
```

Expected:

- HTML contains dashboard navigation
- HTML contains revision history navigation
- HTML contains the `Public` checkbox
- HTML contains editor markup

## Verify JSON Navigation

```bash
curl -sS http://127.0.0.1:8080/records/<slug>/prev -b cookies.txt
curl -sS http://127.0.0.1:8080/records/<slug>/next -b cookies.txt
```

Expected: JSON object with `slug` or `null`.

## Cleanup

```bash
docker compose down -v
rm -f cookies.txt
```
