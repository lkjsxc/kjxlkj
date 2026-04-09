# Route Surface Contract

## HTML Setup + Session Endpoints

- `GET /`, `GET /setup`, `POST /setup`, `GET /login`, `POST /login`, and `POST /logout` keep the same setup and session behavior.

## HTML Resource Pages

- `GET /`: auth-aware homepage shell for mixed resources.
- `GET /admin` and `GET /admin/`: admin dashboard.
- `GET /admin/settings`: admin settings page.
- `GET /admin/media/new`: admin upload-first media creation page.
- `GET /search`: auth-aware browse/search page using `q`, `kind`, `direction`, `sort`, `scope`, `popular_window`, `cursor`, and `limit`.
- `GET /{ref}`: live note page, live media page, or saved-snapshot page.
- `GET /{ref}/history`: history index for one live resource.
- `GET /{ref}/file`: current media binary or `404` for note resources.
- `GET /{snapshot_id}/file`: saved-snapshot media binary or `404` for note snapshots.

## HTML Fragment Endpoints

- `GET /_/popular-resources/home/{window}` returns the homepage Popular section.
- `GET /_/popular-resources/admin/{window}` returns the dashboard Popular section.
- `{window}` is `7d`, `30d`, or `90d`.

## Asset Delivery

- `GET /favicon.ico` returns the canonical favicon.
- `GET /assets/icon.svg` returns the authored icon source.
- `GET /robots.txt` and `GET /sitemap.xml` still depend on persisted `public_base_url`.
- `POST /admin/markdown-preview` renders sanitized Markdown preview HTML for admins only.

## Resource Management

- `POST /resources/notes`: admin-only JSON note create.
- `POST /resources/media`: admin-only multipart media create.
- `PUT /resources/{id}`: admin-only JSON metadata and Markdown update for both resource kinds.
- `PUT /resources/media/{id}/file`: admin-only multipart file replacement for live media.
- `DELETE /resources/{id}`: admin-only soft delete.
- `PUT /resources/favorites/order`: admin-only favorite reorder across mixed resources.

## Resource History + Navigation JSON

- `GET /resources/{id}/history`: admin-only JSON history listing.
- `GET /resources/{id}/prev`: previous accessible resource `id`.
- `GET /resources/{id}/next`: next accessible resource `id`.

## Health Check

- `GET /healthz` returns `200` with body `ok`.
