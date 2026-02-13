# Workspace Suite UX

Back: [/docs/spec/ui/README.md](/docs/spec/ui/README.md)

## Required Modules

| Module | Required behavior |
|---|---|
| Saved views | persist/share search-filter-sort presets |
| Command palette | keyboard-first actions for create/open/move/tag/run-rule |
| Graph explorer | backlink neighborhood with scope-aware filters and return context |
| Librarian agent | launch autonomous structuring runs and review operation diffs |

## Optional Modules

| Module | Optional behavior |
|---|---|
| Workspace switcher | explicit workspace picker for multi-workspace users |
| Dashboards | configurable widgets when teams need them |

## UX Safety Rules

- Optional modules MUST be off by default in baseline note-first mode.
- Feature-rich panels MUST NOT break baseline note editing flow.
- Command palette actions MUST return deterministic success/failure feedback.
- Graph navigation MUST preserve return path to prior note/view context.
- Librarian previews MUST support per-operation accept/reject in review mode.
- Project scoping MAY exist in data model, but dedicated project-navigation pane
  is out of baseline scope and MUST NOT be assumed.

## Findings Coverage

| Finding IDs | Required Outcome |
|---|---|
| `USR-006` | optional modules remain opt-in to reduce cognitive load |
| `USR-003` | core editing path remains primary even with advanced modules enabled |

## Related

- Layout contract: [layout-and-interaction.md](layout-and-interaction.md)
- Workspaces domain: [/docs/spec/domain/workspaces.md](/docs/spec/domain/workspaces.md)
- Findings map: [findings-traceability.md](findings-traceability.md)
