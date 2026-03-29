# Admin Dashboard Contract

## Dashboard Intent

- `GET /admin` is the hybrid admin dashboard.
- The page is for scan, settings, statistics, favorites, recent activity, and entry into note editing.

## Layout

- The persistent shell rail remains visible.
- The rail contains navigation and restrained actions.
- For admins, `New note` sits near the top of the rail rather than below logout or delete actions.
- Main content starts with stats and settings blocks, then recent/favorite note blocks, then the dense paginated admin library.
- The admin library remains the authoritative browse surface for thousands of notes.
- The page header does not show `Admin browse`.
- The page does not expose a top-right search button.

## Main Blocks

- Statistics cards for total, public/private split, favorites, and current month/year activity.
- Global settings form.
- Local editor-preference form.
- Recent notes block.
- Favorite notes block.
- Dense admin library with cursor pagination.

## Visual Rules

- Actions are text-first.
- Search entry belongs on `/search`, not in the rail.
- Empty states remain compact and factual.
- Explanatory helper blocks such as `Admin index` are omitted.
