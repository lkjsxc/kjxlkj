# List and Fetch Behavior

## Home Page

- `GET /` returns the homepage shell after setup completes.
- Guests see public recent notes and public favorites only.
- Signed-in admins see the same structure with private-capable data and admin actions.
- Homepage is intentionally short and does not act as the full browse surface.
- `Quick search` stays fixed near the top of the page.
- `Popular notes`, `Recently updated`, and `Favorites` follow the configured homepage visibility, order, and count settings.
- Each homepage summary section ends with one `View more notes` card linking into `/search`.

## Admin Dashboard

- `GET /admin` returns the admin dashboard.
- Dashboard includes statistics, a concise settings summary, and summary sections for popular notes, recent notes, and favorites.
- Dashboard does not include the full note library or the full settings form.
- Dashboard data includes public and private notes.

## Search and Browse

- `GET /search` is the canonical browse and search workspace.
- Search accepts `q`, `scope`, `popular_window`, `sort`, `cursor`, and `limit`.
- Empty `q` returns the first paginated page of all viewable notes.
- Non-empty `q` returns paginated matches only.
- `scope=favorites` filters to favorites only.
- `scope=popular` orders by popularity and honors `popular_window`.

## Default Ordering

- Homepage recent blocks sort by `updated_at DESC, id ASC`.
- Favorite blocks use persistent `favorite_position ASC`.
- Empty-query `/search` defaults to `updated_desc`.
- Non-empty-query `/search` defaults to `relevance`.
- `/search?scope=favorites` defaults to `favorite_order`.
- `/search?scope=popular` defaults to `popular`.
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

- History index returns the current note plus one visible revision page.
- Revision snapshots are ordered by `revision_number DESC`.
- Guests can fetch only revisions whose stored state is public.
- Admins can fetch all revisions.
- The history rail never expands into per-revision links.
- HTML and JSON history fetches share cursor pagination rules.
