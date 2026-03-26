# Create, Update, and Delete Behavior

## Create (`POST /records`)

- Requires valid session.
- Auto-generates a 22-character opaque `id`.
- Request body may include `body` and `is_private`.
- Defaults: `body = "# New Note\n"`, `is_private = true`.
- Returns `201` with created note JSON.

## Update (`PUT /records/{id}`)

- Requires valid session.
- `id` must exist.
- Request body contains `body` and `is_private`.
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
- Revisions store `body`, `is_private`, `revision_number`, and `created_at`.
- Revision numbers increment from `1`.
- Revisions are immutable and never deleted in this pass.
