# List and Fetch Behavior

## Home Page

- `GET /` returns the homepage shell after setup completes.
- Guests see public recent notes and public favorites only.
- Signed-in admins see the same structure with private-capable data and admin actions.
- Homepage is intentionally short and does not act as the full browse surface.

## Admin Dashboard

- `GET /admin` returns the admin dashboard.
- Dashboard includes statistics, settings, recent notes, and favorites.
- Dashboard does not include the full note library.
- Dashboard data includes public and private notes.

## Search and Browse

- `GET /search` is the canonical browse and search workspace.
- Search accepts `q`, `sort`, `cursor`, and `limit`.
- Empty `q` returns the first paginated page of all viewable notes.
- Non-empty `q` returns paginated matches only.

## Default Ordering

- Homepage recent and favorite blocks sort by `updated_at DESC, id ASC`.
- Empty-query `/search` defaults to `updated_desc`.
- Non-empty-query `/search` defaults to `relevance`.
- Note-to-note `Prev` and `Next` continue to use `created_at`.

## Fetch (`GET /{ref}`)

- Returns full note content if accessible.
- Returns `404` if note does not exist.
- Returns `404` if note is private and user is not authenticated.
- Resolves `ref` by alias first and by `id` second.
- Redirects to the alias URL when a note has an alias and the request used its raw `id`.
- Response includes `body`, `is_private`, `created_at`, and `updated_at`.
- Admin note pages edit the stored Markdown body through one rendered workspace.
- Guest note rendering reflects common structured Markdown such as headings, tables, task lists, and strikethrough when present in stored `body`.

## Note Navigation

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
