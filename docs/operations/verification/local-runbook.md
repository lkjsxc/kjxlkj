# Local Verification Runbook

## Start Runtime

```bash
docker compose up -d app
docker compose ps
```

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
curl -sS -X PUT http://127.0.0.1:8080/v1/records/demo-note   -H 'Content-Type: application/json'   -H 'x-admin-token: local-dev-token'   -d '{"title":"Demo","body":"hello","tags":["demo"]}'
```

Expected: `201` for first create.
