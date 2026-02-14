# Web App Shell

Back: [/docs/spec/ui/README.md](/docs/spec/ui/README.md)

## UX Intent

- baseline UI is note-first and low-noise
- editing confidence has priority over secondary modules
- auth transitions MUST be deterministic and explicit

## Hosting and Language Boundary

- frontend runtime MUST be TypeScript (`strict`) and single-page web app
- application and API are served from the same origin
- root application path MUST be `GET /` and return web shell HTML
- frontend assets MUST be served from `/app/main.js` and `/app/styles.css`
- handwritten JavaScript runtime source is forbidden

## Required Shell Views

| View | Purpose |
|---|---|
| Setup | first-run owner registration while setup is available |
| Login | session entry when setup is locked |
| Notes list | searchable index within current scope |
| Note detail | markdown editor with deterministic save/conflict feedback |
| Jobs panel | export/backup/automation/librarian run visibility |

## Site Root Rules

- `GET /` MUST render a usable setup/login/editor shell.
- setup and login forms MUST be actionable without leaving the root web app.
- after auth, note list, title, and editor fields MUST be immediately usable.
- session logout MUST return user to setup/login mode deterministically.

## Session UX Rules

- unauthenticated access follows deterministic setup/login routing
- `GET /api/auth/session` may return `401` pre-auth and MUST be handled as expected state
- setup UI MUST appear only while setup is available
- locked setup state MUST switch to login-only presentation
- setup availability discovery MUST come from `GET /api/setup/status`

## Responsive Note/Editor Rules

- At or above desktop breakpoint (`>= 1024px`), app MUST render split layout:
  - note list region fixed on the left
  - editor region on the right
- At widths below desktop breakpoint, editor MUST be the primary view by default.
- At widths below desktop breakpoint, a menu button MUST be visible in top-left.
- Activating the top-left menu button MUST reveal the note list and allow note selection.
- Closing the menu MUST return focus to editor without losing draft state.

## Editing Surface Rules

- title MUST be editable in detail view
- title edits MUST propagate to list/navigation in the same interaction cycle
- autosave is default authoring path
- default editor chrome SHOULD remain minimal

## Related

- UX requirements: [reconstruction-ux-requirements.md](reconstruction-ux-requirements.md)
- Editor flow: [editor-flow.md](editor-flow.md)
- Layout contract: [layout-and-interaction.md](layout-and-interaction.md)
- Type safety: [/docs/spec/technical/type-safety.md](/docs/spec/technical/type-safety.md)
