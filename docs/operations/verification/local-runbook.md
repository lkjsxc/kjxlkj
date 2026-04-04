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
- homepage hero is driven only by intro Markdown
- homepage contains no stats block
- homepage contains a popular-notes block with `7d`, `30d`, and `90d` window controls
- guest popular cards do not show rolling-window or all-time totals
- homepage includes `View more notes` cards that point into `/search`
- homepage section wrappers stay lighter than note cards
- homepage note cards do not stretch to the tallest card in a row
- rectangular cards and controls keep tight corners instead of soft rounded shells
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
- sort and search-submit controls align vertically
- HTML contains previous/next paging controls rather than `More notes`
- results may show contextual snippets rather than only derived summaries
- HTML does not contain a top-right `Browse notes` action
- empty-query HTML does not contain a `Query` or `All notes` state card
- `/search?scope=favorites` returns favorite-only browse results
- `/search?sort=popular_desc&popular_window=30d` returns popularity-ordered browse results

## Verify Admin Dashboard

```bash
curl -sS 'http://127.0.0.1:8080/admin' -b cookies.txt
```

Expected:

- HTML contains stats and a settings entry block
- HTML contains a popular-notes block
- HTML contains recent and favorite note rows
- popular, recent, favorite, and settings sections stack vertically
- HTML does not contain a library block
- `New note` remains in the rail
- HTML does not contain `Admin browse` or `Admin index`
- long previews do not force created/updated metadata into awkward wrapped collisions
- dashboard settings block links to `/admin/settings`
- dashboard favorites use persistent favorite order rather than updated time
- dashboard exposes note-view analytics

## Verify Admin Settings

```bash
curl -sS 'http://127.0.0.1:8080/admin/settings' -b cookies.txt
```

Expected:

- HTML contains the canonical settings form
- settings include intro Markdown
- settings include visibility and order controls for popular, recent, and favorites
- settings include item-count controls defaulting to `5`
- settings include session timeout with default `1440`
- settings include default new-note visibility
- settings include default search page size
- HTML does not contain Vim-mode controls

## Verify Admin Note Shell

Use the returned `id` from create.

```bash
curl -sS http://127.0.0.1:8080/<id> -b cookies.txt
```

Expected:

- HTML contains `Prev` / `Next` labels
- HTML contains one `All history` card and no inline saved-snapshot links
- HTML does not contain `Rich mode` or `Text mode`
- HTML does not contain helper text next to `Public`
- HTML does not render a title-adjacent `Public` or `Private` pill above the editor
- rectangular buttons, cards, and editor shells keep tight `2px` through `4px` corners
- HTML hides `Saving` / `Saved`
- HTML contains note-view analytics metadata for admins
- preview starts closed
- no repeated save requests occur after the page becomes idle without further edits
- HTML does not contain Vim-mode status text or controls

## Verify History Page

```bash
curl -sS http://127.0.0.1:8080/<id>/history -b cookies.txt
```

Expected:

- HTML contains one `Live note` card
- HTML contains visible saved-snapshot cards in newest-first order
- the first snapshot card on page one is labeled `Latest saved snapshot`
- later snapshot cards use `Saved snapshot N`
- snapshot cards link to root-path opaque snapshot IDs rather than numbered note-scoped URLs
- HTML contains previous/next paging controls when the snapshot set spans multiple pages
- the rail still contains one `All history` card rather than snapshot links
- HTML does not contain visible rail section headings

## Verify Compact Admin Note

Use browser verification or a real narrow viewport.

Expected:

- the page does not require horizontal scrolling
- opening the note leaves typing focus inside the visible editor
- preview starts closed
- `New note` appears near the top of the rail
- `Prev` and `Next` card footprints remain stable even when one side is unavailable
- typed Markdown stays legible in the editor and renders correctly when preview is opened
- the page owns vertical scrolling instead of the editor body exposing a second normal scroll region
- compact preview opens as a fixed overlay and closes cleanly
- compact preview still works when the rail is hidden behind the drawer
- compact preview content is lighter than the surrounding shell
- compact iPhone-width rendering keeps the same UI font family as other widths
- compact rectangular controls keep the same tight-corner radius family as desktop

## Verify Browser Visual Checks

```bash
docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm visual-verify
```

Expected:

- desktop screenshots pass home/search/admin/admin-settings/note assertions
- compact screenshots pass closed and open drawer assertions
- iPhone-width or equivalent compact screenshots confirm font consistency
- homepage popular window switching works at runtime
- the icon asset is linked through `/favicon.ico` and renders in shell branding
- the icon verification checks transparent corners and centered text mass
- the live typing scenario uses keyboard input on the visible Markdown editor surface
- the admin-note idle scenario detects no repeated no-op save churn
- visual verification exits `0`

## Cleanup

```bash
docker compose -f docker-compose.yml -f docker-compose.verify.yml down -v
rm -f cookies.txt
```
