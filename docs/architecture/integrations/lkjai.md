# lkjai Integration Contract

## Goal

Expose a stable machine-facing API surface for `lkjai` without making `lkjai`
impersonate a browser user.

## Canonical Machine Routes

- `GET /api/users/{user}/resources/search`
- `GET /api/users/{user}/resources/{ref}`
- `GET /api/users/{user}/resources/{ref}/history`
- `POST /api/users/{user}/resources/notes`
- `POST /api/users/{user}/resources/media`
- `PUT /api/users/{user}/resources/{ref}`
- `DELETE /api/users/{user}/resources/{ref}`

## Rules

- `/api/users/{user}/resources/...` is the assistant-facing namespace.
- The `{user}` segment selects one personal space.
- `lkjai` uses a service-account bearer token.
- `lkjai` does not use browser session cookies.
- Search, fetch, and history routes return JSON only.
- Note and media mutation routes keep the existing resource payload shape.
- `lkjai` should confirm create and update operations before issuing them.
