# List and Fetch Behavior

## Public Root List

- `GET /` returns the public searchable shell after setup completes.
- Unauthenticated users see only public notes.
- Authenticated admins may still open `/`, but it remains the public index surface.
- The route accepts `q`, `cursor`, and `limit`.
- The rail remains visible while the main pane shows the result list.

## Admin Dashboard List

- `GET /admin` returns the admin searchable index.
- Admin dashboard includes public and private notes.
- Dashboard list is dense and paginated.
- The rail may show recent-note shortcuts, but it does not replace the main list.

## Default Ordering

- Index pages sort by `updated_at DESC, id ASC`.
- Search keeps the same ordering after filtering matches.

## Fetch (`GET /{id}`)

- Returns full note content if accessible.
- Returns `404` if note does not exist.
- Returns `404` if note is private and user is not authenticated.
- Response includes `body`, `is_private`, `created_at`, and `updated_at`.
- Admin note pages choose rich mode or text-mode fallback from the stored Markdown body.

## Note Navigation

- `Prev` and `Next` relationships use `created_at` order, not `updated_at`.
- `Prev` means the nearest older accessible note.
- `Next` means the nearest newer accessible note.
- Guest navigation skips private notes.
- Admin navigation includes private notes.

## History Fetch

- History index returns the current note plus visible revisions.
- Revision snapshots are ordered by `revision_number DESC`.
- Guests can fetch only revisions whose stored state is public.
- Admins can fetch all revisions.
