# Runtime Route Map

- `/setup`, `/login`, `/logout` -> setup and session handlers.
- `/` -> auth-aware homepage handler.
- `/admin` and `/admin/` -> admin dashboard handler.
- `/admin/settings` -> admin settings page and submit handlers.
- `/search` -> auth-aware browse/search handler.
- `/favicon.ico` -> production favicon handler.
- `/assets/icon.svg` -> visible brand icon handler.
- `/robots.txt` -> conditional robots policy handler.
- `/sitemap.xml` -> conditional sitemap handler.
- `/admin/markdown-preview` -> admin-only preview renderer.
- `/{ref}` -> root resource handler for live notes and saved snapshots.
- `/{ref}/history` -> history index handler.
- `/records` -> note create handler.
- `/records/favorites/order` -> favorite reorder handler.
- `/records/{id}` -> note update (PUT), delete (DELETE) handlers.
- `/records/{id}/history` -> saved-snapshot history handler.
- `/records/{id}/prev` -> previous note handler.
- `/records/{id}/next` -> next note handler.
- `/healthz` -> health handler.

## Access Notes

- Public root is a homepage rather than a library dump.
- Admin dashboard uses session guard.
- Admin settings uses session guard.
- Search widens to private notes only when session is valid.
- Note and history handlers check live-note access and saved-snapshot visibility.
- Root-path current-note routes resolve alias first and opaque ID second.
