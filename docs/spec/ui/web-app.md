# Web App Shell

Back: [/docs/spec/ui/README.md](/docs/spec/ui/README.md)

## UX Intent

- Baseline UI is note-first and low-noise.
- Authoring confidence outranks feature density.
- Auth transitions MUST be deterministic.

## Required Shell Views

| View | Purpose |
|---|---|
| Setup | first-run owner registration only while setup is available |
| Login | authenticated session entry after setup lock |
| Notes list | searchable note index |
| Note detail | Obsidian-like markdown editor + title + backlinks |
| Agent runs | `kjxlkj-agent` run status and review surface |

## Session UX Rules

- Pre-auth `GET /api/auth/session` `401` is expected and non-fatal.
- Setup-locked state MUST render login-only presentation.
- Session expiry MUST redirect to login without losing local draft recovery path.

## Note Creation Rules

- `Create New Note` MUST create and select the new note immediately.
- If no title is provided, default title MUST be current datetime at creation.
- Title display MUST stay independent from immutable `note_id`.

## Editing Surface Rules

- Editor MUST be markdown-first and autosave-first.
- Default chrome SHOULD hide inline save/version/delete controls.
- Optional modules MUST NOT displace baseline note editing surfaces.

## Related

- Editor flow: [editor-flow.md](editor-flow.md)
- Workspace suite: [workspace-suite.md](workspace-suite.md)
- Layout contract: [layout-and-interaction.md](layout-and-interaction.md)
