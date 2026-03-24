# Route Map Contract

## Route to Handler Mapping

- `/healthz` -> health handler.
- `/` -> setup-first home handler.
- `/setup` -> setup GET/POST handlers.
- `/login` -> login GET/POST handlers.
- `/logout` -> logout POST handler.
- `/admin` and `/admin/` -> admin shell guard handler.
- `/v1/records` -> list handler.
- `/v1/records/{id}` -> fetch, upsert, delete handlers.

## Guarding Model

- Setup/login/home/admin handlers use setup and session guards.
- HTML routes use cookie session guard (`session_id`) for admin transitions.
- Record API read handlers are unguarded.
- Write handlers call token guard before mutation.

## Response Model

Handlers return deterministic status codes and payload shapes (HTML for setup/session pages, JSON for record APIs).
