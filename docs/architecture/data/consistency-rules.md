# Data Consistency Rules

## Current Resource State

- `id` is immutable once assigned.
- `alias`, `is_favorite`, `favorite_position`, and `visibility` belong to current resource state.
- `created_at` never changes.
- `updated_at` changes on every successful write.
- `title`, `summary`, and search document are derived from the current body and updated on every write.

## Saved Snapshot State

- Every create or update writes one saved snapshot from the post-save resource state.
- Saved snapshot `id` is immutable once assigned.
- Saved snapshot visibility is evaluated per stored snapshot.
- Saved snapshots never become searchable current resources.
- Saved snapshots copy media derivative metadata from the post-save resource state.

## Navigation State

- `Prev` / `Next` relationships are computed from `created_at`.
- Browsing indexes are computed from `updated_at`.

## Settings State

- Homepage hero content belongs to personal-space settings.
- Homepage section visibility and order belong to personal-space settings.
- Default new-resource visibility belongs to personal-space settings.
- Search default page size belongs to personal-space settings.
- Session timeout belongs to platform configuration.
- Site identity fields belong to personal-space settings.
- Site icon state belongs to personal-space settings.
- Media WebP quality belongs to personal-space settings.
- Discovery public-origin state belongs to personal-space settings.
