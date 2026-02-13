# Web App Shell

Back: [/docs/spec/ui/README.md](/docs/spec/ui/README.md)

## UX Intent

- The baseline UI is note-first and low-noise.
- Core editing confidence is prioritized over secondary panels.
- Auth transitions MUST be deterministic and never ambiguous.

## Hosting Boundary

- SPA assets are built with React + Vite.
- Application and API are served from the same origin.

## Required Shell Views

| View | Purpose |
|---|---|
| Setup | first-run owner registration only when setup is available |
| Login | authenticated session entry when setup is locked |
| Notes list | searchable note index within scope |
| Note detail | markdown-native editor with title, metadata, backlinks |
| Jobs panel | export/backup/automation progress including librarian runs |

## Session UX Rules

- Unauthenticated access follows deterministic setup/login routing.
- `GET /api/auth/session` MAY return `401` before login and MUST be treated as
  expected unauthenticated state, not fatal error.
- Setup UI MUST be shown only while setup is actually available.
- If setup is locked (for example deterministic `409`), UI MUST switch to
  login-only presentation with no setup-like visuals.
- Session expiry MUST redirect to re-auth flow without draft loss.

## Editing Surface Rules

- Note title MUST be editable in detail view.
- Title edits MUST propagate to lists and related navigation surfaces in the same
  interaction cycle.
- Autosave is the default authoring path.
- Manual `Save Now`, inline `Delete`, and inline version badges are optional and
  SHOULD remain hidden in default layout.
- Secondary modules (dashboard/workspace switcher) MUST NOT displace baseline
  note editing surfaces.

## Findings Coverage

| Finding IDs | Required Outcome |
|---|---|
| `USR-001`, `USR-004` | deterministic pre-auth/session and setup-lock presentation behavior |
| `USR-006` | optional modules remain opt-in and low-noise |
| `USR-007`, `USR-008` | immediate title propagation and minimal default editor chrome |

## Related

- Workspace suite: [workspace-suite.md](workspace-suite.md)
- Editor flow: [editor-flow.md](editor-flow.md)
- Findings map: [findings-traceability.md](findings-traceability.md)
