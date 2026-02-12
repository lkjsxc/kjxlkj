# Workspace Suite UX

Back: [/docs/spec/ui/README.md](/docs/spec/ui/README.md)

## Required Modules

| Module | Required behavior |
|---|---|
| Workspace switcher | quick switching with remembered recent workspaces |
| Project rail | create/open/filter project-scoped content |
| Saved views | persist and share search/filter/sort presets |
| Dashboards | configurable widget surfaces per workspace |
| Command palette | keyboard-first actions for create/open/move/tag/run-rule |
| Graph explorer | backlink-driven neighborhood and scope-aware filtering |

## Dashboard Widget Baseline

- recent changes
- assigned or watched notes
- search result panel
- automation run status panel

## UX Safety Rules

- Feature-rich surfaces MUST NOT break baseline note editing flow.
- Command palette actions MUST show deterministic success/failure feedback.
- Graph navigation MUST preserve return path to previous note/view context.

## Related

- Layout contract: [layout-and-interaction.md](layout-and-interaction.md)
- Workspaces domain: [/docs/spec/domain/workspaces.md](/docs/spec/domain/workspaces.md)
- Automation domain: [/docs/spec/domain/automation.md](/docs/spec/domain/automation.md)
