# Admin Dashboard Contract

## Dashboard Intent

- `GET /admin` is the admin dashboard.
- The page is for scan, analytics, favorites, recent activity, settings entry, and entry into resource editing.

## Layout

- The persistent shell rail remains visible.
- For admins, `New note` sits near the top of the rail.
- `New media` sits directly below `New note`.
- Admin dashboard rails place `Open GitHub` above `Logout`.
- Main content starts with compact stats, then `Settings`, `Popular`, `Recently updated`, and `Favorites`.

## Main Blocks

- Statistics cards for total resources, public/private split, favorites, and current month or year activity.
- Statistics also expose resource-view activity totals and recent popularity context.
- `Popular` uses an in-place `7d` / `30d` / `90d` switch.
- Recently updated and Favorites may both contain notes and media.
- Favorites use one full reorderable admin list across mixed resources.

## Visual Rules

- Actions are text-first.
- Full browsing belongs on `/search`, not in the dashboard.
- Empty states remain compact and factual.
