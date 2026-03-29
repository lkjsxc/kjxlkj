# List and Fetch Behavior

## Home Page

- `GET /` returns the homepage shell after setup completes.
- Guests see public recent notes, public favorites, and public statistics only.
- Signed-in admins see the same homepage structure plus private-capable data and quick admin actions.
- The route does not expose cursor pagination as its primary purpose.

## Admin Dashboard List

- `GET /admin` returns the hybrid admin dashboard.
- The page includes dashboard sections first and the full admin library below.
- Admin dashboard includes public and private notes.
- The admin library remains dense and paginated.

## Search Page

- `GET /search` is the only canonical search surface.
- Search accepts `q`, `cursor`, and `limit`.
- Empty query renders guidance instead of a full browse dump.

## Default Ordering

- Recent and library lists sort by `updated_at DESC, id ASC`.
- Search sorts by search rank first and `updated_at DESC, id ASC` second.

## Fetch (`GET /{ref}`)

- Returns full note content if accessible.
- Returns `404` if note does not exist.
- Returns `404` if note is private and user is not authenticated.
- Resolves `ref` by alias first and by `id` second.
- Redirects to the alias URL when a note has an alias and the request used its raw `id`.
- Response includes `body`, `is_private`, `created_at`, and `updated_at`.
- Admin note pages edit the stored Markdown body through one rendered workspace.
- Guest note rendering must reflect common structured Markdown such as headings, tables, task lists, and strikethrough when present in stored `body`.

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
