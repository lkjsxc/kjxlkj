# Product Surface Map

## Public Surface

- `GET /` lists visible articles after setup completion; before any admin exists, it redirects to `/setup`.
- `GET /article/{slug}` renders an article with last-updated metadata and previous/next navigation.
- `GET /article/{slug}/history` renders article history for authenticated admin only.
- `GET /search` renders role-aware result filtering.

## Authentication Surface

- `GET /setup` renders setup page when no admin exists.
- `POST /setup` creates fixed username `admin` with password.
- `GET /login` renders login form.
- `POST /login` authenticates `admin` by password.
- `POST /logout` destroys admin session.

## Admin Surface

- `GET /admin` renders admin dashboard with article list and create form.
- `POST /admin/create` creates new article; private defaults to true.
- `POST /admin/rename` renames article slug.
- `POST /admin/delete/{slug}` moves article to trash.
- `POST /admin/toggle-private/{slug}` toggles privacy.
- `GET /admin/settings`, `POST /admin/settings/save`, `POST /admin/settings/reindex`.
- `GET /admin/trash`, `POST /admin/trash/restore/{slug}`, `POST /admin/trash/delete-permanent/{slug}`.

## Inline Editing Surface

- `POST /article/{slug}/edit` persists inline edits.
- `POST /article/{slug}/history/restore` restores a historical revision.
- Editing is admin-only and preserves canonical Markdown.
- Save and preview buttons are removed; autosave is deterministic.

## Contract Priority

- Setup-first rules override normal auth flow until setup completion.
