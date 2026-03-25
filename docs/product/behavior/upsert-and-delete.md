# Create, Update, and Delete Behavior

## Create (`POST /records`)

- Requires valid session (401 if not authenticated).
- Auto-generates slug from current datetime: `YYYY-MM-DD-HHmm`.
- If slug collision, append counter: `2026-03-25-0134-2`.
- Request body may include `body` and `is_private`.
- Defaults: `body = "# New Note"`, `is_private = true`.
- Returns `201` with created note.

## Update (`PUT /records/{slug}`)

- Requires valid session (401 if not authenticated).
- Slug must exist (404 if not found).
- Request body may include `body` and/or `is_private`.
- Creates a new revision before applying changes.
- Updates `updated_at` timestamp.
- Returns `200` with updated note.

## Delete (`DELETE /records/{slug}`)

- Requires valid session (401 if not authenticated).
- Slug must exist (404 if not found).
- Performs soft delete: sets `deleted_at` timestamp.
- Note remains in database but hidden from views.
- Returns `204` with no body.

## Revision Tracking

- Every update creates a revision entry before changes.
- Revisions store: `body`, `is_private`, `revision_number`, `created_at`.
- Revision numbers increment from 1.
- Revisions are immutable and never deleted.

## Update Timestamp

Each successful write (create, update) sets `updated_at` to current UTC time.
