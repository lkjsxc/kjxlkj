# Route Map Contract

## Route to Handler Mapping

- `/healthz` -> health handler.
- `/` -> landing page handler.
- `/setup` -> setup GET/POST handlers.
- `/login` -> login GET/POST handlers.
- `/logout` -> logout POST handler.
- `/admin` and `/admin/` -> admin dashboard handler.
- `/{slug}` -> note view/edit handler.
- `/records` -> note create handler (POST).
- `/records/{slug}` -> note update (PUT), delete (DELETE) handlers.
- `/records/{slug}/history` -> revision history handler.
- `/records/{slug}/prev` -> previous note handler.
- `/records/{slug}/next` -> next note handler.

## Guarding Model

- Setup/login/landing handlers use setup guards.
- Admin dashboard uses session guard.
- Note view handler checks `is_private` flag against session.
- `/records/*` handlers require session guard.

## Response Model

- `/healthz` returns plain text.
- Setup/login/admin/note pages return HTML.
- `/records/*` endpoints return JSON.
