# Route Surface Contract

## Route Model

- `/` is the global public feed.
- `/{user}` is the public feed for one personal space.
- `/{user}/{ref}` is the canonical resource route.
- `{user}` is the personal-space slug and normally equals the owner username.
- Resource aliases are unique inside one personal space.
- Root `/{ref}` resource routing does not exist.
- Legacy `/resources/*` and `/api/resources/*` routes do not exist.

## Auth Endpoints

- `GET /setup` renders first-user setup when no user exists.
- `POST /setup` creates the first active user and personal space.
- `GET /login` accepts optional same-origin `return_to`.
- `POST /login` creates an opaque server-side user session.
- `POST /logout` revokes the current user session.
- `GET /reset-password`, `POST /reset-password/request`, and `POST /reset-password` own local password recovery.

## HTML Resource Pages

- `GET /`: public feed across all public resources.
- `GET /{user}`: public feed for one personal space.
- `GET /{user}/admin`: member dashboard for one personal space.
- `GET /{user}/settings`: owner/admin settings page.
- `GET /{user}/search`: browse/search page using `q`, `kind`, `direction`, `sort`, `scope`, `popular_window`, `cursor`, and `limit`.
- `GET /{user}/live`: live broadcast page for one personal space.
- `GET /{user}/{ref}`: live note page, live media page, or saved-snapshot page.
- `GET /{user}/{ref}/history`: history index for one live resource.
- `GET /{user}/{ref}/file`: current media binary or `404` for note resources.
- `GET /{user}/{snapshot_id}/file`: saved-snapshot media binary or `404` for note snapshots.
- File routes accept optional `variant=card|display|poster`.
- `variant=card` is the canonical card still-image route for both image and video media.
- `variant=poster` remains the canonical video player poster route.

## HTML Fragment Endpoints

- `GET /{user}/_/popular-resources/home/{window}` returns the user-space homepage Popular section.
- `GET /{user}/_/popular-resources/admin/{window}` returns the user-space dashboard Popular section.
- `{window}` is `1d`, `7d`, `30d`, `90d`, or `all`.

## Asset Delivery

- `GET /favicon.ico` returns the canonical favicon.
- `GET /assets/icon.svg` returns the authored icon source.
- `GET /{user}/assets/site-icon` returns the personal-space icon or the fallback icon.
- `POST /{user}/settings/site-icon` stores one uploaded image as the personal-space icon.
- `POST /{user}/settings/site-icon/reset` clears the personal-space icon.
- `GET /robots.txt` and `GET /sitemap.xml` expose only public resources.
- `GET /.well-known/nostr.json` returns configured public Nostr names and relays.
- `POST /{user}/markdown-preview` renders sanitized Markdown preview HTML for authorized members.
- `POST /account/password` changes the signed-in user's password.

## Resource Management

- `POST /{user}/resources/notes`: authorized JSON note create.
- `POST /{user}/resources/media`: authorized multipart media create.
- `POST /{user}/resources/{id}/media-attachments`: authorized multipart note attachment upload.
- `PUT /{user}/resources/{id}`: authorized metadata and Markdown update.
- `DELETE /{user}/resources/{id}`: authorized soft delete.
- `PUT /{user}/favorites/order`: authorized favorite reorder across mixed resources.

## Live Signaling

- `GET /{user}/live/ws`: public WebSocket endpoint for one personal space.
- A member with `BroadcastLive` may connect as the active broadcaster.
- Guests and members may connect as viewers when the space is publicly visible.
- Viewer-count signaling is sent only to the active broadcaster.
- Broadcaster disconnect or page leave ends that personal-space stream.
- One broadcaster may be active per personal space.

## Resource History + Navigation JSON

- `GET /{user}/resources/{id}/history`: authorized JSON history listing.
- `GET /{user}/resources/{id}/prev`: previous accessible resource `id`.
- `GET /{user}/resources/{id}/next`: next accessible resource `id`.

## Health Check

- `GET /healthz` returns `200` with body `ok`.
