# Product Surface Map

## Public Surface

- `GET /` lists visible articles after setup completion; before any admin exists, it redirects to `/setup`.
- `GET /article/{slug}` renders an article with last-updated metadata and previous/next navigation, without author attribution display.
- `GET /article/{slug}/history` renders article history for authenticated admin only.
- `GET /search` renders role-aware result filtering.

## Authentication Surface

- `GET /setup` renders setup page when no admin exists.
- `POST /setup` creates fixed username `admin` with password-only credential setup.
- `GET /login` renders password-only login form for fixed identity `admin`.
- `POST /login` authenticates fixed username `admin` by password-only payload.
- `POST /logout` destroys admin session.

## Admin Surface

- `GET /admin` renders admin dashboard with article list and create form.
- `POST /admin/create` creates new article; articles are private by default.
- `POST /admin/rename` renames article slug.
- `POST /admin/delete/{slug}` moves article to trash.
- `GET /admin/settings`, `POST /admin/settings/save`, `POST /admin/settings/reindex`.
- `GET /admin/trash`, `POST /admin/trash/restore/{slug}`, `POST /admin/trash/delete-permanent/{slug}`.

## Inline Editing Surface

- `POST /article/{slug}/edit` persists inline edits.
- `POST /article/{slug}/history/restore` restores a historical revision.
- Editing is admin-only and preserves canonical Markdown.
- Save and preview buttons are removed; autosave is deterministic.

## Contract Priority

- Setup-first rules override normal auth flow until setup completion.
