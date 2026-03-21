# Admin Editor Behavior

## Access

- `/admin` requires an active admin session.
- Unauthorized requests redirect to `/login`.

## Core Editor Features

- Browse Markdown files.
- Open and edit Markdown content.
- Save file content atomically.
- Create, rename, and delete Markdown files.
- Toggle frontmatter `private` visibility.

## Interaction Model

- Server-rendered pages are primary.
- HTMX script is loaded for lightweight enhancement.
- Small JavaScript drives editor operations.

## API Endpoints Used by Editor

- `GET /admin/open/{slug}`
- `POST /admin/create`
- `POST /admin/save`
- `POST /admin/rename`
- `POST /admin/delete/{slug}`
- `POST /admin/toggle-private/{slug}`
