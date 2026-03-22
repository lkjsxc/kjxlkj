# Admin Dashboard Flow

## Access Rules

- `/admin` requires authenticated admin session after setup completion.
- Unauthorized access redirects to `/login`.
- Before setup completion, `/admin` redirects to `/setup`.

## Scope

- Admin dashboard is not a dedicated editor page.
- Dashboard manages article lifecycle and links into inline article editing.
- The system has one user identity: fixed username `admin`.

## Core Capabilities

- List all articles including private.
- Create article with datetime-derived placeholder title and slug.
- Private toggle is available at creation and edit.
- Rename, soft-delete, and privacy-toggle actions exist.
- Settings and trash views remain in admin surface.

## Endpoint Surface

- `GET /admin`
- `POST /admin/create`
- `POST /admin/rename`
- `POST /admin/delete/{slug}`
- `POST /admin/toggle-private/{slug}`

## Editing Path

- Direct editing happens on `/article/{slug}` only.
- Inline editor appears on article page for authenticated admin.
