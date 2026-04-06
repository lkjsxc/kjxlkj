# Admin Dashboard Contract

## Dashboard Intent

- `GET /admin` is the admin dashboard.
- The page is for scan, analytics, favorites, recent activity, settings entry, and entry into note editing.

## Layout

- The persistent shell rail remains visible.
- The rail contains navigation and restrained actions.
- For admins, `New note` sits near the top of the rail rather than below logout or delete actions.
- Main content starts with compact stats, then a `Settings` entry section, then stacked `Popular notes`, `Recently updated`, and `Favorites` sections.
- Dashboard spacing should stay tighter than the early tall-card builds without collapsing section readability.
- The dashboard does not include the canonical settings form.
- The dashboard does not include the full note library.
- The page header does not show `Admin browse`.
- The page does not expose a top-right search button.

## Main Blocks

- Statistics cards for total, public/private split, favorites, and current month or year activity.
- Statistics also expose note-view activity totals and recent popularity context.
- `Settings` is a compact summary block with a link to `/admin/settings`.
- `Settings` summary also shows the configured session timeout.
- Popular notes use an in-place `7d` / `30d` / `90d` switch.
- Popular switching leaves the visible URL at `/admin`.
- Popular switching replaces the full section from server-rendered HTML without a full page reload.
- Popular-note rows show rolling-window and all-time totals.
- Popular notes include a `View more notes` card into the matching `/search` state.
- Recently updated includes a `View more notes` card into `/search`.
- Favorites use a full reorderable admin list of all favorites.
- Favorites include a `View more notes` card into `/search?scope=favorites`.
- The settings summary should describe intro presence and section order rather than a removed home-title field.

## Visual Rules

- Actions are text-first.
- Full browsing belongs on `/search`, not in the dashboard.
- Empty states remain compact and factual.
- Explanatory helper blocks such as `Admin index` are omitted.
- Section wrappers stay lighter than note cards and do not stack repeated nested borders.
- Settings summary cards keep a clearer gap between their labels and values than the tighter early pass.
