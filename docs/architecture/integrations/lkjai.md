# lkjai Integration Contract

## Goal

Expose a stable machine-facing API surface for the future `lkjai` server-side
assistant.

## Canonical Machine Routes

- `GET /api/resources/search`
- `GET /api/resources/{id}`
- `GET /api/resources/{id}/history`
- `POST /api/resources/notes`
- `POST /api/resources/media`
- `PUT /api/resources/{id}`

## Rules

- `/api/resources/...` is the canonical assistant-facing namespace.
- Existing browser-oriented `/resources/...` routes may remain as compatibility
  aliases during transition.
- Search, fetch, and history routes return JSON only.
- Note and media mutation routes keep the existing resource payload shape.
- `lkjai` should confirm create and update operations before issuing them.
