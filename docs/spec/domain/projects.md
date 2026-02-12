# Projects Domain

Back: [/docs/spec/domain/README.md](/docs/spec/domain/README.md)

## Project Model

- A project MUST belong to exactly one workspace.
- Project name MUST be unique within its workspace.
- Projects MAY be archived; archived projects are excluded by default list filters.

## Note Scoping Rules

- Notes MAY reference `project_id`.
- If `project_id` is set, note workspace MUST equal project workspace.
- Project deletion MUST NOT hard-delete note streams.
- On project deletion, project-scoped notes MUST remain reachable through workspace
 views until reassigned or archived.

## Related

- Workspaces: [workspaces.md](workspaces.md)
- Notes: [notes.md](notes.md)
- Permissions: [permissions.md](permissions.md)
