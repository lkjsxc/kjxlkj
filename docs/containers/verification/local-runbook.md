# Local Verification Runbook

## Scope

- Validates local Docker behavior for `app` + `postgres`.
- Run from repository root.
- Start with [../compose/commands.md](../compose/commands.md).

## Setup/Login/Admin Functional Verification Flow

### 1) Confirm setup-first redirect before admin exists

```bash
curl -sS -D - -o /dev/null http://127.0.0.1:8080/ \
  | tr -d '\r' \
  | awk 'NR==1 || tolower($1)=="location:"'
```

Expected output includes:

- `HTTP/1.1 302 Found`
- `location: /setup`

### 2) Confirm `/setup` is full HTML (not placeholder-only)

```bash
curl -sS http://127.0.0.1:8080/setup \
  | grep -E '<title>Initial setup</title>|<h1>Set up first admin account</h1>|<form method="post" action="/setup">'
```

Expected output includes all three markers.

### 3) Create first admin user

```bash
curl -sS -D - -o /dev/null \
  -X POST http://127.0.0.1:8080/setup \
  -H 'Content-Type: application/x-www-form-urlencoded' \
  --data 'username=admin&password=s3cret' \
  | tr -d '\r' \
  | awk 'NR==1 || tolower($1)=="location:"'
```

Expected output includes:

- `HTTP/1.1 303 See Other`
- `location: /login`

### 4) Confirm setup lock is active after admin creation

```bash
curl -sS -o /dev/null -w 'code=%{http_code}\n' http://127.0.0.1:8080/setup
```

Expected output:

- `code=404`

### 5) Login and capture a reusable session id

```bash
curl -sS -D /tmp/kjxlkj.login.headers -o /tmp/kjxlkj.login.body -w 'code=%{http_code}\n' \
  -X POST http://127.0.0.1:8080/login \
  -H 'Content-Type: application/x-www-form-urlencoded' \
  --data 'username=admin&password=s3cret'
cat /tmp/kjxlkj.login.body
grep -i '^set-cookie: session_id=' /tmp/kjxlkj.login.headers
export KJXLKJ_SESSION_ID="$(awk -F 'session_id=|;' 'tolower($1) ~ /^set-cookie: / {print $2}' /tmp/kjxlkj.login.headers | head -n 1)"
test -n "$KJXLKJ_SESSION_ID"
```

Expected output:

- `code=200`
- Body is `login-ok`.
- A `set-cookie: session_id=...` header is present.
- `KJXLKJ_SESSION_ID` is non-empty.

### 6) Verify admin guard behavior

Authenticated request:

```bash
curl -sS -H "Cookie: session_id=$KJXLKJ_SESSION_ID" -o /dev/null -w 'code=%{http_code}\n' http://127.0.0.1:8080/admin
```

Expected output:

- `code=200`

Unauthenticated request:

```bash
curl -sS -D - -o /dev/null http://127.0.0.1:8080/admin \
  | tr -d '\r' \
  | awk 'NR==1 || tolower($1)=="location:"'
```

Expected output includes:

- `HTTP/1.1 302 Found`
- `location: /login`

### 7) Verify admin content mutation path

```bash
curl -sS -H "Cookie: session_id=$KJXLKJ_SESSION_ID" -o /dev/null -w 'code=%{http_code}\n' \
  -X POST http://127.0.0.1:8080/admin/create \
  -H 'Content-Type: application/x-www-form-urlencoded' \
  --data-urlencode 'slug=smoke-post' \
  --data-urlencode 'title=Smoke Post' \
  --data-urlencode 'body=# Smoke'
```

Expected output:

- `code=201`

### 8) Verify public visibility and admin read-back

```bash
curl -sS http://127.0.0.1:8080/ | grep -F 'smoke-post'
curl -sS -H "Cookie: session_id=$KJXLKJ_SESSION_ID" http://127.0.0.1:8080/admin/open/smoke-post | head -n 1
```

Expected output:

- Public home output includes `smoke-post`.
- Admin open output starts with `# Smoke`.

## Diagnosing Blank or Placeholder Pages

### `/setup` appears blank or missing form

Run:

```bash
curl -sS http://127.0.0.1:8080/setup \
  | grep -E '<title>Initial setup</title>|<form method="post" action="/setup">'
```

Interpretation:

- Missing markers indicates setup rendering/startup issue.

Recovery action:

```bash
docker compose build --no-cache app
docker compose up -d --force-recreate app
```

### `/login` only returns `login`

Run:

```bash
curl -sS http://127.0.0.1:8080/login
```

Interpretation:

- `login` is expected for current contract, not a placeholder failure.

### `/admin` returns `200` with empty body

Run:

```bash
curl -sS -H "Cookie: session_id=$KJXLKJ_SESSION_ID" -w '\ncode=%{http_code} bytes=%{size_download}\n' http://127.0.0.1:8080/admin
```

Interpretation:

- `code=200` with `bytes=0` is expected when no articles exist yet.

### `/` appears blank after setup/login

Run:

```bash
curl -sS -D - -o /tmp/kjxlkj.home http://127.0.0.1:8080/ | tr -d '\r' | head -n 1
wc -c /tmp/kjxlkj.home
```

Interpretation:

- `HTTP/1.1 200 OK` with `0` bytes means no visible content exists yet.
- `302` with `location: /setup` means admin state is missing; follow recovery workflow.

## PostgreSQL and Data Recovery Workflow

### 1) Inspect health before deleting data

```bash
docker compose ps
docker compose logs --no-color --tail=120 postgres app
docker compose exec postgres pg_isready -U kjxlkj -d kjxlkj
docker compose exec postgres psql -U kjxlkj -d kjxlkj -c 'SELECT COUNT(*) AS admin_count FROM admin_users;'
```

Expected result:

- `pg_isready` reports accepting connections.
- `admin_count` is `0` before setup and `>=1` after setup.

### 2) Reset PostgreSQL only (preserve content files)

```bash
docker compose down
rm -rf data/postgres
mkdir -p data/postgres
docker compose up -d postgres app
```

Expected result:

- `/` redirects to `/setup` again.
- `data/content` remains intact.

### 3) Full local reset (destructive)

Backup current data first:

```bash
cp -a data "data.backup.$(date +%Y%m%d%H%M%S)"
```

Then reset all runtime state:

```bash
docker compose down
rm -rf data/postgres data/content
mkdir -p data/postgres data/content
docker compose build app
docker compose up -d postgres app
```

Expected result:

- Fresh setup-first state (`/` redirects to `/setup`).
- No existing content slugs remain.

## Docker Acceptance Path

```bash
docker compose --profile verify run --rm verify
```

Expected result:

- Exit code `0`.
- Formatting, lint, tests, build, topology, and line-limit checks all pass.

Optional JSON wrapper:

```bash
cargo run --bin kjxlkj -- compose verify
```

## Cleanup

```bash
unset KJXLKJ_SESSION_ID
rm -f /tmp/kjxlkj.login.headers /tmp/kjxlkj.login.body /tmp/kjxlkj.home
docker compose down
```
