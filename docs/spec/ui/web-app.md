# Web App Shell

Back: [/docs/spec/ui/README.md](/docs/spec/ui/README.md)

## Hosting Boundary

- SPA assets are built with React + Vite.
- Actix MUST serve static assets and API from same server origin.

## Required Shell Views

| View | Purpose |
|---|---|
| Setup | first-run account registration |
| Login | authenticated session entry |
| Notes list | searchable note index |
| Note detail | markdown-native editor + editable title + metadata + backlinks |
| Media note detail | standalone image/video note presentation and metadata |
| Jobs panel | export/backup progress |

## Session UX Rules

- Unauthenticated access redirects to login/setup flow.
- `GET /api/v1/auth/session` MAY return `401` before login; UI MUST treat this as unauthenticated state, not a fatal error.
- Session expiry triggers deterministic re-authentication path.

## Editing UX Rules

- Note title MUST be editable in the right-pane detail surface.
- Note deletion MUST be accessible from detail view with deterministic confirmation UX.
- Autosave MUST be the default authoring path.
- Manual save controls MAY exist, but core flow MUST NOT depend on them.

## Related

- Auth spec: [/docs/spec/security/auth.md](/docs/spec/security/auth.md)
- HTTP contract: [/docs/spec/api/http.md](/docs/spec/api/http.md)
