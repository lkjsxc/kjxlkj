# List and Fetch Behavior

## Home Page

- `GET /` returns the homepage shell after setup completes.
- The homepage hero still uses only editable global `home_intro_markdown`.
- Home sections list mixed resources rather than note-only rows.
- Guests see public-only resources.
- Signed-in admins see the same structure with private-capable data and admin actions.
- The homepage remains intentionally short and does not become the full browse surface.

## Admin Dashboard

- `GET /admin` returns the admin dashboard.
- The dashboard includes resource statistics, popularity, recent activity, favorites, and a settings entry point.
- Dashboard data includes both notes and media.

## Search and Browse

- `GET /search` is the canonical browse and search workspace.
- Search accepts `q`, `kind`, `sort`, `cursor`, `limit`, `direction`, `scope`, and `popular_window`.
- `kind=all` is the default.
- `scope=favorites` narrows results to favorite resources only.
- Empty `q` returns the first paginated page of viewable resources inside the current kind, scope, and sort.
- Non-empty `q` returns paginated matches inside the current kind and scope only.

## Default Ordering

- Homepage recent blocks sort by `updated_at DESC, id ASC`.
- Homepage favorite blocks use persistent `favorite_position ASC`.
- Homepage popular blocks sort by the selected rolling window, then lifetime views, then `updated_at DESC, id ASC`.
- Note-to-note `Prev` and `Next` semantics now apply to the mixed resource timeline.

## Fetch (`GET /{ref}`)

- Returns the live note page, live media page, or one saved-snapshot page if accessible.
- Returns `404` if the target does not exist.
- Returns `404` if the target is private and the user is not authenticated.
- Resolves `ref` by alias first and then by globally unique opaque ID.
- Current live-resource IDs redirect to the alias URL when an alias exists.
- Saved-snapshot IDs never redirect.

## Current File Fetch (`GET /{ref}/file`)

- Returns the current media binary when `/{ref}` is live media.
- Returns `404` when `/{ref}` resolves to a note.
- Returns `404` for private live media when no valid session exists.

## Snapshot File Fetch (`GET /{snapshot_id}/file`)

- Returns the immutable media binary stored on that snapshot.
- Returns `404` when the snapshot belongs to a note.
- Uses the snapshot’s stored visibility.

## History Fetch

- History index returns the live resource plus one visible saved-snapshot page.
- Saved snapshots are ordered by `snapshot_number DESC`.
- Guests can fetch only saved snapshots whose stored state is public.
- Admins can fetch all saved snapshots.
- The history rail never expands into per-snapshot links.
