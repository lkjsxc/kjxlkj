# Admin Dashboard Contract

## Dashboard Intent

- `GET /admin` is the admin dashboard.
- The page is for scan, recent activity, analytics, and entry into note editing.

## Layout

- The persistent shell rail remains visible.
- The rail contains navigation and restrained actions.
- For admins, `New note` sits near the top of the rail rather than below logout or delete actions.
- Main content starts with compact stats, then a concise settings-summary section, then stacked `Popular notes`, `Recently updated`, and `Favorites` sections.
- Dashboard spacing should stay tighter than the early tall-card builds.
- The dashboard does not include the full note library.
- The page header does not show `Admin browse`.
- The page does not expose a top-right search button.

## Main Blocks

- Statistics cards for total, public/private split, favorites, and current month/year activity.
- Statistics also expose note-view activity totals and recent popularity context.
- Settings summary block with a clear entry point to `/settings`.
- Popular notes block with a server-driven `7d` / `30d` / `90d` selector.
- Recent notes block.
- Favorite notes block with summary cards and a `View more notes` entry into `/search`.

## Visual Rules

- Actions are text-first.
- Full browsing belongs on `/search`, not in the dashboard.
- Empty states remain compact and factual.
- Explanatory helper blocks such as `Admin index` are omitted.
- Section wrappers stay lighter than note cards and do not stack repeated nested borders.
