# Admin Dashboard Contract

## Dashboard Intent

- `GET /admin` is the admin dashboard.
- The page is for scan, settings, favorites, recent activity, and entry into note editing.

## Layout

- The persistent shell rail remains visible.
- The rail contains navigation and restrained actions.
- For admins, `New note` sits near the top of the rail rather than below logout or delete actions.
- Main content starts with compact stats and settings blocks, then recent and favorite note blocks.
- Dashboard spacing should stay tighter than the early tall-card builds.
- The dashboard does not include the full note library.
- The page header does not show `Admin browse`.
- The page does not expose a top-right search button.

## Main Blocks

- Statistics cards for total, public/private split, favorites, and current month/year activity.
- Settings form for global defaults, including default Vim mode.
- Browser-local Vim override controls.
- Recent notes block.
- Favorite notes block.

## Visual Rules

- Actions are text-first.
- Full browsing belongs on `/search`, not in the dashboard.
- Empty states remain compact and factual.
- Explanatory helper blocks such as `Admin index` are omitted.
