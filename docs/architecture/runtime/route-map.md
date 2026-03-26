# Runtime Route Map

- `/setup`, `/login`, `/logout` -> setup/session handlers.
- `/` -> public searchable index handler.
- `/admin` and `/admin/` -> admin searchable index handler.
- `/{id}` -> note view/edit handler.
- `/{id}/history` -> history index handler.
- `/{id}/history/{revision_number}` -> history snapshot handler.
- `/records` -> note create handler.
- `/records/{id}` -> note update (PUT), delete (DELETE) handlers.
- `/records/{id}/history` -> revision history handler.
- `/records/{id}/prev` -> previous note handler.
- `/records/{id}/next` -> next note handler.
- `/healthz` -> health handler.

## Access Notes

- Public root lists public notes only.
- Admin dashboard uses session guard.
- Note/history handlers check current-note access and snapshot visibility.
