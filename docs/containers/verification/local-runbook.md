# Local Verification Runbook

## Scope

- Validate Docker flow for `postgres` + `app`.
- Run from repository root.

## Flow

1. Confirm setup redirect:

```bash
curl -sS -D - -o /dev/null http://127.0.0.1:8080/ | tr -d '\r' | awk 'NR==1 || tolower($1)=="location:"'
```

2. Complete setup with fixed admin:

```bash
curl -sS -D - -o /dev/null -X POST http://127.0.0.1:8080/setup -H 'Content-Type: application/x-www-form-urlencoded' --data 'password=s3cret'
```

3. Confirm setup lock page (no blank response):

```bash
curl -sS http://127.0.0.1:8080/setup | grep -E 'setup-locked-page|Go to login'
```

4. Login (password-only semantics):

```bash
curl -sS -D /tmp/kjxlkj.login.headers -o /tmp/kjxlkj.login.body -X POST http://127.0.0.1:8080/login -H 'Content-Type: application/x-www-form-urlencoded' --data 'password=s3cret'
export KJXLKJ_SESSION_ID="$(awk -F 'session_id=|;' 'tolower($1) ~ /^set-cookie: / {print $2}' /tmp/kjxlkj.login.headers | head -n 1)"
```

5. Create article without explicit privacy flag (must default private):

```bash
curl -sS -o /dev/null -w 'code=%{http_code}\n' -H "Cookie: session_id=$KJXLKJ_SESSION_ID" -X POST http://127.0.0.1:8080/admin/create -H 'Content-Type: application/x-www-form-urlencoded' --data-urlencode 'slug=smoke-post' --data-urlencode 'title=Smoke Post' --data-urlencode 'body=# Smoke'
```

6. Verify inline editor and history pages:

```bash
curl -sS -H "Cookie: session_id=$KJXLKJ_SESSION_ID" http://127.0.0.1:8080/article/smoke-post | grep -E 'article-inline-editor|article-updated|article-nav'
curl -sS -H "Cookie: session_id=$KJXLKJ_SESSION_ID" http://127.0.0.1:8080/article/smoke-post/history | grep -F 'article-history-page'
```

7. Verify guest cannot see private article:

```bash
curl -sS -o /dev/null -w 'code=%{http_code}\n' http://127.0.0.1:8080/article/smoke-post
```

## Acceptance

```bash
docker compose --profile verify run --rm verify
```

Expected: exit code `0`.
