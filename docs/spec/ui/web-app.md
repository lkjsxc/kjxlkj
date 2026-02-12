# Web App Shell

Back: [/docs/spec/ui/README.md](/docs/spec/ui/README.md)

## Hosting Boundary

- SPA assets are built with React + Vite.
- Actix MUST serve static assets and API from same server origin.

## Required Shell Views

| View | Purpose |
|---|---|
| Setup | first-run owner account registration |
| Login | authenticated session entry when setup is locked |
| Notes list | searchable note index within scope |
| Note detail | markdown-native editor + title + metadata + backlinks |
| Jobs panel | export/backup/automation progress including librarian runs |

## Session UX Rules

- Unauthenticated access redirects to login/setup flow.
- `GET /api/auth/session` MAY return `401` before login; UI MUST treat this
 as unauthenticated state, not a fatal error.
- Setup UI MUST be shown only when setup is actually available.
- If setup is locked (for example deterministic `409` on setup attempt), the UI
 MUST switch to login-only presentation and MUST NOT reuse setup-like visuals.
- Redundant authenticated-state banners/panels are optional and SHOULD be
 omitted unless they provide actionable value.
- Session expiry triggers deterministic re-authentication path.

## Editing UX Rules

- Note title MUST be editable in the detail surface.
- Title edits MUST propagate to note lists and related navigation surfaces
 without requiring full-page refresh.
- Autosave MUST be the default authoring path.
- Manual save controls are optional and SHOULD remain hidden by default.
- Inline delete controls in the editor are optional and SHOULD remain hidden by
 default.

## Related

- Workspace suite: [workspace-suite.md](workspace-suite.md)
- Auth spec: [/docs/spec/security/auth.md](/docs/spec/security/auth.md)
- HTTP contract: [/docs/spec/api/http.md](/docs/spec/api/http.md)
