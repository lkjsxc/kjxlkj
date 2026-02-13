# Web App Shell

Back: [/docs/spec/ui/README.md](/docs/spec/ui/README.md)

## UX Intent

- baseline UI is note-first and low-noise
- editing confidence has priority over secondary modules
- auth transitions MUST be deterministic and explicit

## Hosting and Language Boundary

- frontend runtime MUST be TypeScript (`strict`) and single-page web app
- application and API are served from the same origin
- direct JavaScript runtime source is forbidden

## Required Shell Views

| View | Purpose |
|---|---|
| Setup | first-run owner registration while setup is available |
| Login | session entry when setup is locked |
| Notes list | searchable index within current scope |
| Note detail | markdown editor with deterministic save/conflict feedback |
| Jobs panel | export/backup/automation/librarian run visibility |

## Session UX Rules

- unauthenticated access follows deterministic setup/login routing
- `GET /api/auth/session` may return `401` pre-auth and MUST be handled as expected state
- setup UI MUST appear only while setup is available
- locked setup state MUST switch to login-only presentation

## Editing Surface Rules

- title MUST be editable in detail view
- title edits MUST propagate to list/navigation in the same interaction cycle
- autosave is default authoring path
- default editor chrome SHOULD remain minimal

## Related

- UX requirements: [reconstruction-ux-requirements.md](reconstruction-ux-requirements.md)
- Editor flow: [editor-flow.md](editor-flow.md)
- Type safety: [/docs/spec/technical/type-safety.md](/docs/spec/technical/type-safety.md)
