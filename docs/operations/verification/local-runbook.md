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

- `/` returns the homepage after setup
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

- HTML contains the shared shell rail
- HTML contains homepage sections rather than one bare browse list
- homepage note cards do not stretch to the tallest card in a row
- HTML does not expose raw note IDs in normal list rows
- HTML does not contain visible rail section headings such as `CREATE`, `NAVIGATE`, `ACTIONS`, or `SCOPE`
- HTML does not contain helper copy such as `Browse current public notes`
- HTML does not contain top-right `Search` or `Admin sign in` actions

## Verify Search Page

```bash
curl -sS 'http://127.0.0.1:8080/search?q=new'
curl -sS 'http://127.0.0.1:8080/search?q=new' -b cookies.txt
```

Expected:

- guest search returns only public matches
- admin search may include private matches
- HTML contains the search form in the main pane
- results may show contextual snippets rather than only derived summaries
- HTML does not contain a top-right `Browse notes` action

## Verify Admin Dashboard

```bash
curl -sS 'http://127.0.0.1:8080/admin' -b cookies.txt
```

Expected:

- HTML contains stats and settings blocks before the admin library
- HTML contains admin list rows
- `New note` remains in the rail
- HTML does not contain `Admin browse` or `Admin index`
- long previews do not force created/updated metadata into awkward wrapped collisions

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
- HTML does not render a title-adjacent `Public` or `Private` pill above the editor
- HTML hides `Saving` / `Saved`
- HTML references local editor assets rather than an external editor CDN
- preview starts closed
- no repeated save requests occur after the page becomes idle without further edits

## Verify History Page

```bash
curl -sS http://127.0.0.1:8080/<id>/history -b cookies.txt
```

Expected:

- HTML contains one `Current note` card
- HTML contains visible revision cards in newest-first order
- the rail still contains one `All history` card rather than revision links
- HTML does not contain visible rail section headings

## Verify Compact Admin Note

Use browser verification or a real narrow viewport.

Expected:

- the page does not require horizontal scrolling
- the editor toolbar stays inside the viewport and wraps instead of showing a detached scrollbar strip
- opening the note leaves typing focus inside the visible editor
- preview starts closed
- `New note` appears near the top of the rail
- `Prev` and `Next` card footprints remain stable even when one side is unavailable
- typed Markdown stays legible in the editor and renders correctly when preview is opened
- the page owns vertical scrolling instead of the editor body exposing a second normal scroll region
- compact preview opens as a fixed overlay and closes cleanly

## Verify Browser Visual Checks

```bash
docker compose --profile verify run --rm visual-verify
```

Expected:

- desktop screenshots pass home/search/admin/note assertions
- compact screenshots pass closed and open drawer assertions
- the live typing scenario uses keyboard input on the visible Markdown editor surface
- the admin-note idle scenario detects no repeated no-op save churn
- visual verification exits `0`

## Cleanup

```bash
docker compose down -v
rm -f cookies.txt
```
