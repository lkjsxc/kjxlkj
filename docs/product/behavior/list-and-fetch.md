# List and Fetch Behavior

## Home Page

- `GET /` returns the homepage shell after setup completes.
- The homepage hero uses only the editable global `home_intro_markdown`.
- The homepage always contains `Quick search`.
- The homepage may contain `Popular notes`, `Recently updated`, and `Favorites` in the configured order and visibility.
- Guests see public-only home data.
- Signed-in admins see the same structure with private-capable data and admin actions.
- The homepage is intentionally short and does not act as the full browse surface.

## Admin Dashboard

- `GET /admin` returns the admin dashboard.
- The dashboard includes statistics, popularity, recent activity, favorites, and a settings entry point.
- The dashboard does not own the canonical settings form.
- The dashboard does not include the full note library.
- Dashboard data includes public and private notes.

## Admin Settings

- `GET /admin/settings` returns the dedicated settings workspace.
- The settings page owns the canonical global settings form.
- Saving settings affects rendered HTML routes and new-note defaults immediately.
- Saving settings affects future login session lifetime immediately.

## Search and Browse

- `GET /search` is the canonical browse and search workspace.
- Search accepts `q`, `sort`, `cursor`, `limit`, `direction`, `scope`, and `popular_window`.
- `scope=all` is the default.
- `scope=favorites` narrows search and browse results to favorite notes only.
- Empty `q` returns the first paginated page of viewable notes inside the current scope and sort.
- Non-empty `q` returns paginated matches inside the current scope only.

## Default Ordering

- Homepage recent blocks sort by `updated_at DESC, id ASC`.
- Homepage favorite blocks use persistent `favorite_position ASC`.
- Homepage popular blocks sort by the selected rolling window, then lifetime views, then `updated_at DESC, id ASC`.
- Empty-query `/search` defaults to `updated_desc` for `scope=all`.
- Empty-query `/search` defaults to `favorite_position_asc` for `scope=favorites`.
- Non-empty-query `/search` defaults to `relevance`.
- Note-to-note `Prev` and `Next` continue to use `created_at`.

## Fetch (`GET /{ref}`)

- Returns the current note page or a revision snapshot page if accessible.
- Returns `404` if the target does not exist.
- Returns `404` if the target is private and user is not authenticated.
- Resolves `ref` by alias first, then by globally unique opaque ID.
- Aliases resolve only to current notes.
- Current-note IDs redirect to the alias URL when the note has an alias.
- Revision IDs never redirect.
- Current-note responses include the editable current body for admins.
- Revision responses include the immutable historical snapshot body.

## Note Navigation

- `Prev` means the nearest older accessible note.
- `Next` means the nearest newer accessible note.
- Guest navigation skips private notes.
- Admin navigation includes private notes.
- HTML note rails always render both timeline slots even when one side resolves to `null`.

## History Fetch

- History index returns the current note plus one visible revision page.
- Revision snapshots are ordered by `revision_number DESC`.
- Revision snapshots also expose stable opaque revision IDs.
- Guests can fetch only revisions whose stored state is public.
- Admins can fetch all revisions.
- The history rail never expands into per-revision links.
- HTML and JSON history fetches share cursor pagination rules.
