# Create, Update, and Delete Behavior

## Create (`POST /records`)

- Requires valid session.
- Auto-generates a 26-character opaque `id`.
- Request body must include `body`.
- Request body may include `alias`, `is_favorite`, and `is_private`.
- The browser create flow seeds `body` with a local-time heading in `YYYY-MM-DD HH:mm`.
- Server create behavior does not invent fallback note content when `body` is missing.
- Alias may be `null`.
- Default `is_favorite = false`.
- Default `is_private = true`.
- Returns `201` with created note JSON.
- Creates saved snapshot `1` from the persisted note state.

## Update (`PUT /records/{id}`)

- Requires valid session.
- `id` must exist.
- Request body contains `body`, `alias`, `is_favorite`, and `is_private`.
- Updates `updated_at`.
- Recomputes derived title, summary, and search fields.
- Creates one new saved snapshot from the persisted post-update note state.
- Returns `200` with updated note.
- The stored body remains canonical raw Markdown regardless of editor presentation.

## Public Visibility Control

- The canonical UI control is a checkbox labeled `Public`.
- Checked maps to `is_private = false`.
- Unchecked maps to `is_private = true`.
- Toggling visibility triggers immediate save and immediate chrome refresh.

## Alias and Favorite Rules

- Clearing the alias returns the note to an ID-only canonical route.
- Alias validity and uniqueness are checked before write.
- Direct typing and paste must preserve internal `-`, `_`, and `.` separators in the editor field.
- Alias input normalization should not delete allowed separators while the user is typing.
- Favorite state is current-note state only.
- Favorite changes do not alter revision visibility or history access.

## Editor Rules

- The normal UI exposes no user-facing mode choice.
- The note body owns the visible document heading on both admin and guest note pages.
- The visible textarea label `Markdown body` is absent.
- If the editor helper JS fails to load, the page still remains editable.

## Delete (`DELETE /records/{id}`)

- Requires valid session.
- `id` must exist.
- Performs soft delete.
- Returns `204` with no body.

## Saved Snapshot Tracking

- Creating a note creates saved snapshot `1`.
- Every update creates one additional immutable saved snapshot after the live-note write succeeds.
- Saved snapshots store `id`, `alias`, `title`, `summary`, `body`, `is_private`, `snapshot_number`, and `created_at`.
- Saved snapshot `id` uses the same 26-character opaque format as current notes.
- Saved snapshot numbers increment from `1`.
- Saved snapshots are immutable and never deleted.
