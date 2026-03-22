# Admin HTMX Fragment Contracts

This document defines HTMX request and response contracts for admin interactions.

## Global HTMX Rules

- Contract applies when request header `HX-Request: true` is present.
- Responses return HTML fragments targeting stable admin shell regions.
- Fragment IDs are stable and reused across all flows:
  - `#admin-article-list`
  - `#admin-trash-list`
  - `#admin-editor-pane`
  - `#admin-preview-pane`
  - `#admin-status-banner`
  - `#admin-conflict-banner`
- Admin auth is enforced before handler logic:
  - setup incomplete: `401` with `HX-Redirect: /setup`
  - missing or expired session: `401` with `HX-Redirect: /login`

## Fragment Target Contract

| Fragment ID | Required content |
| --- | --- |
| `admin-article-list` | Ordered list of slugs with active selection and private badges |
| `admin-trash-list` | Ordered list of trashed slugs with restore/permanent-delete actions |
| `admin-editor-pane` | Editor form for current slug including `last_known_revision` |
| `admin-preview-pane` | Sanitized server-rendered markdown preview |
| `admin-status-banner` | Save/create/rename/delete outcome messages |
| `admin-conflict-banner` | Visible warning when stale revision was overwritten |

## Endpoint Contracts

| Endpoint | Request payload | Success response | Error response |
| --- | --- | --- | --- |
| `GET /admin/open/{slug}` | path `slug` | `200` editor fragment + preview fragment | `404` missing slug, `401` auth failure |
| `POST /admin/preview` | `slug`, `title`, `body`, `private` | `200` preview fragment for `#admin-preview-pane` | `400` validation banner |
| `POST /admin/save` | `slug`, `title`, `body`, `private`, `last_known_revision` | `200` status banner; returns updated revision token | `400` validation banner |
| `POST /admin/create` | `slug`, `title`, `body` | `201` list + editor fragments with new active slug | `400` validation banner |
| `POST /admin/rename` | `slug`, `new_slug` | `200` refreshed list + editor fragments | `400` validation banner |
| `POST /admin/delete/{slug}` | path `slug` | `200` active list fragment + status banner (soft-delete to trash) | `404` missing slug |
| `POST /admin/toggle-private/{slug}` | path `slug` | `200` list row/editor metadata fragment with new privacy value | `404` missing slug |
| `POST /admin/settings/save` | `site_title`, `session_timeout_minutes` | `200` settings status fragment | `400` validation banner |
| `POST /admin/settings/reindex` | none | `200` settings status fragment with reindex completion marker | `500` deterministic error output |
| `POST /admin/trash/restore/{slug}` | path `slug` | `200` trash + active list fragments with status banner | `404` missing slug |
| `POST /admin/trash/delete-permanent/{slug}` | path `slug` | `200` trash list fragment with status banner | `404` missing slug |

## Server-Side Preview Contract

- Preview generation is always server-rendered from submitted markdown.
- `POST /admin/preview` is the canonical preview endpoint for HTMX swaps.
- Response HTML is sanitized before returning the preview fragment.
- Preview refresh is available from both explicit user action and keyboard shortcut.

## Conflict Signaling Contract

- Save requests include `last_known_revision`.
- If server detects stale revision, it still applies the incoming write (last-write-wins).
- Conflict save response MUST include:
  - updated `#admin-status-banner`
  - visible `#admin-conflict-banner`
- Conflict details are defined canonically in [admin-conflict-warning.md](admin-conflict-warning.md).
