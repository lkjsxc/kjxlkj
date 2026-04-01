# Data Consistency Rules

## Current Note State

- `id` is immutable once assigned.
- `alias`, `is_favorite`, `favorite_position`, and `is_private` belong to the current note state.
- `created_at` never changes.
- `updated_at` changes on every successful write.
- `title`, `summary`, and search document are derived from the current body and updated on every write.

## Revision State

- Every update snapshots the prior current body and privacy state.
- Revision visibility is evaluated per stored snapshot.
- Revisions never become searchable current-note records.

## Navigation State

- `Prev` / `Next` relationships are computed from `created_at`.
- Browsing indexes are computed from `updated_at`.

## Settings State

- Homepage hero content belongs to singleton app settings.
- Homepage section visibility and order belong to singleton app settings.
- Default new-note visibility belongs to singleton app settings.
- Search default page size belongs to singleton app settings.
