# Settings Page Contract

## Route Intent

- `GET /settings` is the dedicated admin-only settings page.
- The page is linked from the dashboard and from the persistent rail.
- The page is the canonical place for global app configuration.

## Sections

- `Homepage intro` edits the Markdown shown below `Home`.
- `Homepage layout` controls homepage visibility, order, and count for `Popular notes`, `Recently updated`, and `Favorites`.
- `New note defaults` controls whether newly created notes start private or public.
- `Search defaults` controls default search page size.
- `Favorite order` exposes the full reorderable favorite list.

## Interaction Rules

- Saving settings applies immediately to subsequent HTML requests.
- Homepage layout settings affect `/` only.
- Reordering favorites operates on the full current favorite set, not the homepage-sized subset.
- The settings page must not expose editor-only local browser preferences.

## Visual Rules

- The page uses the same shell and section language as Home, Search, and Dashboard.
- The page favors short factual helper text over narrative explanations.
- The page keeps the reorderable favorites tool near other global configuration rather than on the dashboard.
