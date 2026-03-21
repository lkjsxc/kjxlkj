# Admin Editor Flow

See [Product Surface Map](../surface-map.md) for endpoint scope.
See [Access Control Contract](../policies/access-control.md) for route/session requirements.

## Access Rules

- `/admin` requires a valid authenticated admin session.
- Unauthorized access is redirected to `/login` after setup completion.

## Core Capabilities

- List Markdown articles.
- Open and edit Markdown content.
- Save content atomically.
- Create, rename, and delete Markdown files.
- Toggle frontmatter `private` visibility.

## Editor Endpoints

- `GET /admin/open/{slug}`
- `POST /admin/create`
- `POST /admin/save`
- `POST /admin/rename`
- `POST /admin/delete/{slug}`
- `POST /admin/toggle-private/{slug}`
