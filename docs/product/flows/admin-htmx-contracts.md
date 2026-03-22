# Admin Interaction Contract

## Global Rules

- Admin authentication is required before mutation handlers execute.
- Setup incomplete redirects admin routes to `/setup`.
- Missing or expired session redirects to `/login`.

## Dashboard Mutations

| Endpoint | Payload | Success |
| --- | --- | --- |
| `POST /admin/create` | `slug`, `title`, `private`, `body` | `303` redirect to `/article/{slug}` |
| `POST /admin/rename` | `slug`, `new_slug` | `303` redirect to `/article/{new_slug}` |
| `POST /admin/delete/{slug}` | path slug | `204` for non-fragment callers |

## Inline Edit Mutations

| Endpoint | Payload | Success |
| --- | --- | --- |
| `POST /article/{slug}/edit` | `title`, `private`, `body`, `last_known_revision` | `200` inline editor fragment with updated revision/status |
| `POST /article/{slug}/history/restore` | `commit_id` | `303` redirect to `/article/{slug}` |

## Conflict Behavior

- Save uses `last_known_revision`.
- Stale save keeps last-write-wins semantics.
- Server still returns updated revision token and timestamp.
