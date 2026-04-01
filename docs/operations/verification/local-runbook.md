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
- homepage may include intro Markdown below `Home`
- homepage contains no stats block
- homepage contains a popular-notes block with `7d`, `30d`, and `90d` window controls
- homepage includes a browse-action card that points to `/search`
- homepage section wrappers stay lighter than note cards
- homepage note cards do not stretch to the tallest card in a row
- HTML does not expose raw note IDs in normal list rows
- HTML does not contain visible rail section headings such as `CREATE`, `NAVIGATE`, `ACTIONS`, or `SCOPE`
- HTML does not contain helper copy such as `Browse current public notes`
- HTML does not contain top-right `Search` or `Admin sign in` actions

## Verify Search Page

```bash
curl -sS 'http://127.0.0.1:8080/search'
curl -sS 'http://127.0.0.1:8080/search?q=new'
curl -sS 'http://127.0.0.1:8080/search?q=new' -b cookies.txt
```

Expected:

- empty query returns a paginated note-card browse page
- guest search returns only public matches
- admin search may include private matches
- HTML contains the search form in the main pane
- non-empty queries show a query display near the search input and sort control
- HTML contains sort controls in the main pane
- HTML does not contain a visible `Sort` label
- HTML contains previous/next paging controls rather than `More notes`
- results may show contextual snippets rather than only derived summaries
- HTML does not contain a top-right `Browse notes` action
- empty-query HTML does not contain a `Query` or `All notes` state card

## Verify Admin Dashboard

```bash
curl -sS 'http://127.0.0.1:8080/admin' -b cookies.txt
```

Expected:

- HTML contains stats and settings blocks
- HTML contains a popular-notes block
- HTML contains recent and favorite note rows
- popular, recent, favorite, and settings sections stack vertically
- HTML does not contain a library block
- `New note` remains in the rail
- HTML does not contain `Admin browse` or `Admin index`
- long previews do not force created/updated metadata into awkward wrapped collisions
- settings include homepage intro Markdown, homepage popular count, default Vim mode, and browser-local Vim override controls
- dashboard favorites use persistent favorite order rather than updated time
- dashboard exposes note-view analytics

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
- HTML contains note-view analytics metadata for admins
- preview starts closed
- no repeated save requests occur after the page becomes idle without further edits

## Verify History Page

```bash
curl -sS http://127.0.0.1:8080/<id>/history -b cookies.txt
```

Expected:

- HTML contains one `Current note` card
- HTML contains visible revision cards in newest-first order
- HTML contains previous/next paging controls when the revision set spans multiple pages
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
- compact iPhone-width rendering keeps the same UI font family as other widths

## Verify Browser Visual Checks

```bash
docker compose --profile verify run --rm visual-verify
```

Expected:

- desktop screenshots pass home/search/admin/note assertions
- compact screenshots pass closed and open drawer assertions
- iPhone-width or equivalent compact screenshots confirm font consistency
- homepage popular window switching works at runtime
- the icon asset is linked in HTML and renders in shell branding
- the live typing scenario uses keyboard input on the visible Markdown editor surface
- the admin-note idle scenario detects no repeated no-op save churn
- visual verification exits `0`

## Cleanup

```bash
docker compose down -v
rm -f cookies.txt
```
