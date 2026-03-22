# Product Surface Map

## Public Surface

- `GET /` lists visible articles after setup completion; before any admin exists, it redirects to `/setup`.
- `GET /article/{slug}` renders a single visible article.

## Authentication Surface

- `GET /setup` renders the complete first-admin setup page when none exists.
- `GET /login` renders admin login after setup completion.
- `POST /logout` destroys the admin session.

## Admin Surface

- `GET /admin` renders the full editor shell page for authenticated admins.
- `GET /admin/open/{slug}` loads editor content for a slug.
- `POST /admin/preview` returns server-rendered preview fragments for HTMX swaps.
- `POST /admin/save` persists edits with last-write-wins conflict signaling.
- `POST /admin/create`, `POST /admin/rename`, `POST /admin/delete/{slug}`, and `POST /admin/toggle-private/{slug}` mutate content state.

## UX Contract Layers

- Server-rendered page contracts: [flows/page-contracts.md](flows/page-contracts.md)
- HTMX admin contracts: [flows/admin-htmx-contracts.md](flows/admin-htmx-contracts.md)
- JavaScript UX contracts: [flows/admin-js-ux-contract.md](flows/admin-js-ux-contract.md)
- Conflict warning contract: [flows/admin-conflict-warning.md](flows/admin-conflict-warning.md)

## Contract Priority

- Setup-first rules override normal auth entry rules until setup is complete.
