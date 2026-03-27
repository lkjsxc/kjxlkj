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

- `/` returns the public browse page after setup
- setup redirects to `/login`
- login redirects to `/admin`

## Create a Note

```bash
curl -sS -X POST http://127.0.0.1:8080/records \
  -H 'Content-Type: application/json' \
  -b cookies.txt \
  -d '{"body":"# 2026-03-27 21:04\n","is_private":true}'
```

Expected: `201` with JSON body containing `id`.

## Verify Public Root Browse

```bash
curl -sS 'http://127.0.0.1:8080/'
```

Expected:

- HTML contains a dense note list
- HTML contains the shell rail
- HTML does not expose raw note IDs in normal list rows
- HTML does not contain `RECENT`
- HTML does not contain a rail search form

## Verify Search Page

```bash
curl -sS 'http://127.0.0.1:8080/search?q=new'
curl -sS 'http://127.0.0.1:8080/search?q=new' -b cookies.txt
```

Expected:

- guest search returns only public matches
- admin search may include private matches
- HTML contains the search form in the main pane

## Verify Admin Dashboard

```bash
curl -sS 'http://127.0.0.1:8080/admin' -b cookies.txt
```

Expected:

- HTML contains admin list rows
- HTML links to `/search`
- HTML uses restrained actions

## Verify Admin Note Shell

Use the returned `id` from create.

```bash
curl -sS http://127.0.0.1:8080/<id> -b cookies.txt
```

Expected:

- HTML contains `Prev` / `Next` labels
- HTML contains one `All history` card and no inline revision links
- HTML does not contain `Rich mode` or `Text mode`
- HTML does not contain helper text next to `Public`
- HTML hides `Saving` / `Saved`
- HTML references local editor assets rather than an external editor CDN

## Verify History Page

```bash
curl -sS http://127.0.0.1:8080/<id>/history -b cookies.txt
```

Expected:

- HTML contains one `Current note` card
- HTML contains visible revision cards in newest-first order
- the rail still contains one `All history` card rather than revision links

## Verify Compact Admin Note

Use browser verification or a real narrow viewport.

Expected:

- the page does not require horizontal scrolling
- the editor toolbar stays inside the viewport and wraps instead of showing a detached scrollbar strip
- opening the note leaves typing focus inside the visible editor
- `New note` appears near the top of the rail
- `Prev` and `Next` card footprints remain stable even when one side is unavailable
- newly typed headings, lists, blockquotes, fenced code, and tables render with normal Toast UI styling
- the page owns vertical scrolling instead of the editor body exposing a second normal scroll region

## Verify Browser Visual Checks

```bash
docker compose --profile verify run --rm visual-verify
```

Expected:

- desktop screenshots pass browse/search/note assertions
- compact screenshots pass closed and open drawer assertions
- the live typing scenario uses keyboard input on the visible WYSIWYG editor surface
- visual verification exits `0`

## Cleanup

```bash
docker compose down -v
rm -f cookies.txt
```
