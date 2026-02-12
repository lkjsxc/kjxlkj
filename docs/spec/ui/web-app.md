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
| Note detail | markdown editor + metadata + backlinks |
| Jobs panel | export/backup progress |

## Session UX Rules

- Unauthenticated access redirects to login/setup flow.
- Session expiry triggers deterministic re-authentication path.

## Related

- Auth spec: [/docs/spec/security/auth.md](/docs/spec/security/auth.md)
- HTTP contract: [/docs/spec/api/http.md](/docs/spec/api/http.md)
