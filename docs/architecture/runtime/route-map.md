# Runtime Route Map

- `/setup`, `/login`, `/logout` -> setup and session handlers.
- `/` -> auth-aware homepage handler.
- `/admin` and `/admin/` -> admin dashboard handler.
- `/admin/settings` -> admin settings page and submit handlers.
- `/search` -> auth-aware browse/search handler.
- `/favicon.ico` -> production favicon handler.
- `/assets/icon.svg` -> visible brand icon handler.
- `/admin/markdown-preview` -> admin-only preview renderer.
- `/{ref}` -> note view/edit handler.
- `/{ref}/history` -> history index handler.
- `/{ref}/history/{revision_number}` -> history snapshot handler.
- `/records` -> note create handler.
- `/records/favorites/order` -> favorite reorder handler.
- `/records/{id}` -> note update (PUT), delete (DELETE) handlers.
- `/records/{id}/history` -> revision history handler.
- `/records/{id}/prev` -> previous note handler.
- `/records/{id}/next` -> next note handler.
- `/healthz` -> health handler.

## Access Notes

- Public root is a homepage rather than a library dump.
- Admin dashboard uses session guard.
- Admin settings uses session guard.
- Search widens to private notes only when session is valid.
- Note and history handlers check current-note access and snapshot visibility.
- Root-path note routes resolve alias first and ID second.
