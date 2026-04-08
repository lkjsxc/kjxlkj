# List and Fetch Behavior

## Home Page

- `GET /` returns the homepage shell after setup completes.
- The homepage hero uses only the editable global `home_intro_markdown`.
- The homepage always contains `Quick search`.
- The homepage may contain `Popular`, `Recently updated`, and `Favorites` in the configured order and visibility.
- The homepage popular-window switch is in-place and does not alter the visible URL.
- Guests see public-only home data.
- Signed-in admins see the same structure with private-capable data and admin actions.
- The homepage is intentionally short and does not act as the full browse surface.
- The homepage is the only library-style page that remains search-indexable.

## Admin Dashboard

- `GET /admin` returns the admin dashboard.
- The dashboard includes statistics, popularity, recent activity, favorites, and a settings entry point.
- The dashboard popular-window switch is in-place and does not alter the visible URL.
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
- `/search` remains guest-readable but is not search-indexable.

## Default Ordering

- Homepage recent blocks sort by `updated_at DESC, id ASC`.
- Homepage favorite blocks use persistent `favorite_position ASC`.
- Homepage popular blocks sort by the selected rolling window, then lifetime views, then `updated_at DESC, id ASC`.
- Empty-query `/search` defaults to `updated_desc` for `scope=all`.
- Empty-query `/search` defaults to `favorite_position_asc` for `scope=favorites`.
- Non-empty-query `/search` defaults to `relevance`.
- Note-to-note `Prev` and `Next` follow [../navigation/timeline/semantics-and-order.md](../navigation/timeline/semantics-and-order.md).

## Fetch (`GET /{ref}`)

- Returns the live note page or a saved-snapshot page if accessible.
- Returns `404` if the target does not exist.
- Returns `404` if the target is private and user is not authenticated.
- Resolves `ref` by alias first, then by globally unique opaque ID.
- Aliases resolve only to current notes.
- Current-note IDs redirect to the alias URL when the note has an alias.
- Saved-snapshot IDs never redirect.
- Current-note responses include the editable current body for admins.
- Saved-snapshot responses include the immutable saved body.
- Public current-note routes are search-indexable only when `public_base_url` is non-blank and valid.
- Saved snapshots remain guest-readable when allowed, but are not search-indexable.

## Note Navigation

- Note and history rails use the shared timeline canon from [../navigation/timeline/README.md](../navigation/timeline/README.md).
- HTML note rails always render both timeline slots even when one side resolves to `null`.
- Timeline cards show relation label, note title, short summary preview, and created time.
- Note and history rails order note-level sections as live context, timeline, `History`, `Open GitHub`, then the trailing action block.

## History Fetch

- History index returns the live note plus one visible saved-snapshot page.
- Saved snapshots are ordered by `snapshot_number DESC`.
- Saved snapshots also expose stable opaque snapshot IDs.
- Guests can fetch only saved snapshots whose stored state is public.
- Admins can fetch all saved snapshots.
- The history rail never expands into per-snapshot links.
- HTML and JSON history fetches share the pager contract from [../navigation/paging/README.md](../navigation/paging/README.md).
- History indexes are not search-indexable.
- The history-page `Live note` card uses the same Created/Updated metadata language as Home and Search cards.
