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

## Update (`PUT /records/{id}`)

- Requires valid session.
- `id` must exist.
- Request body contains `body`, `alias`, `is_favorite`, and `is_private`.
- Creates a new revision before applying changes.
- Updates `updated_at`.
- Recomputes derived title, summary, and search fields.
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
- Favorite state is current-note state only.
- Favorite changes do not alter revision visibility or history access.

## Editor Rules

- The third-party editor must save through canonical Markdown.
- The normal UI exposes no user-facing mode choice.
- If the editor fails to load, the page still remains editable.

## Delete (`DELETE /records/{id}`)

- Requires valid session.
- `id` must exist.
- Performs soft delete.
- Returns `204` with no body.

## Revision Tracking

- Every update creates a revision entry before changes.
- Revisions store `id`, `body`, `is_private`, `revision_number`, and `created_at`.
- Revision `id` uses the same 26-character opaque format as current notes.
- Revision numbers increment from `1`.
- Revisions are immutable and never deleted.
