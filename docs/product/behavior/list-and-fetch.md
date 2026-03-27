# List and Fetch Behavior

## Public Root List

- `GET /` returns the public browse shell after setup completes.
- Unauthenticated users see only public notes.
- Authenticated admins may still open `/`, but it remains the public browse surface.
- The route accepts `cursor` and `limit`.
- The rail remains visible while the main pane shows the result list.

## Admin Dashboard List

- `GET /admin` returns the admin browse index.
- Admin dashboard includes public and private notes.
- Dashboard list is dense and paginated.
- The rail provides navigation and actions, not recent-note shortcuts.

## Search Page

- `GET /search` is the only canonical search surface.
- Search accepts `q`, `cursor`, and `limit`.
- Empty query renders guidance instead of a full browse dump.

## Default Ordering

- Browse and search pages sort by `updated_at DESC, id ASC`.

## Fetch (`GET /{id}`)

- Returns full note content if accessible.
- Returns `404` if note does not exist.
- Returns `404` if note is private and user is not authenticated.
- Response includes `body`, `is_private`, `created_at`, and `updated_at`.
- Admin note pages edit the stored Markdown body through one rendered workspace.

## Note Navigation

- `Prev` and `Next` relationships use `created_at` order, not `updated_at`.
- `Prev` means the nearest older accessible note.
- `Next` means the nearest newer accessible note.
- Guest navigation skips private notes.
- Admin navigation includes private notes.
- HTML note rails always render both timeline slots even when one side resolves to `null`.

## History Fetch

- History index returns the current note plus visible revisions.
- Revision snapshots are ordered by `revision_number DESC`.
- Guests can fetch only revisions whose stored state is public.
- Admins can fetch all revisions.
- The history rail never expands into per-revision links.
