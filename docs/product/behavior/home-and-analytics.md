# Home and Analytics Behavior

## Home

- Homepage Popular, Recently updated, and Favorites are mixed-resource sections.
- Fresh installs order Home sections as `Recently updated`, `Favorites`, then `Popular`.
- Section cards remain shared across notes and media.
- Image media may render a fixed-height cropped thumbnail inside list cards.
- Image and video cards both prefer `variant=card` when that derivative exists.
- File-family cards use text metadata rather than generated thumbnails.
- Media card thumbnails use `128px` height.
- Media card badges and metadata render below the thumbnail without overlapping it.
- Video media cards use a card-style still image rather than the player poster treatment.

## Dashboard Stats

- Dashboard totals count live resources rather than only notes.
- Public/private split, favorites, and recent-update counts also count live resources.
- View totals expose counted opens from [view-counting.md](view-counting.md).
- Dashboard analytics include `Views 1d`, `Views 7d`, `Views 30d`, `Views 90d`, and `Views total`.

## Popularity

- Popularity windows are `1d`, `7d`, `30d`, `90d`, and `all`.
- Popularity ranking applies to live note pages and live media pages.
- Popularity ranking uses counted opens from [view-counting.md](view-counting.md).
- Guest cards do not expose popularity totals.
- Admin Popular cards may expose the active popularity total only.
- When the active window is `all`, the visible card metric label is `Views`.
