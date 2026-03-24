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

## Verify Read API

```bash
curl -sS http://127.0.0.1:8080/v1/records
```

Expected: valid JSON array.

## Verify Write API

```bash
curl -sS -X PUT http://127.0.0.1:8080/v1/records/demo-note \
  -H 'Content-Type: application/json' \
  -H 'x-admin-token: local-dev-token' \
  -d '{"title":"Demo","body":"hello","tags":["demo"]}'
```

Expected: `201` for first create.

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

## Cleanup

```bash
docker compose down -v
rm -f cookies.txt
```
