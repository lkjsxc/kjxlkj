# Workspace Suite UX

Back: [/docs/spec/ui/README.md](/docs/spec/ui/README.md)

## Required Modules

| Module | Required behavior |
|---|---|
| Saved views | persist and share search/filter/sort presets |
| Command palette | keyboard-first actions for create/open/move/tag/run-rule |
| Graph explorer | backlink-driven neighborhood and scope-aware filtering |
| Librarian agent | launch autonomous structuring runs and review operation diffs |

## Optional Modules

| Module | Optional behavior |
|---|---|
| Workspace switcher | explicit workspace picker for multi-workspace users |
| Dashboards | configurable widget surfaces when teams need them |

## UX Safety Rules

- Feature-rich surfaces MUST NOT break baseline note editing flow.
- Command palette actions MUST show deterministic success/failure feedback.
- Graph navigation MUST preserve return path to previous note/view context.
- Librarian operation previews MUST allow per-operation accept/reject before apply
  when review mode is enabled.
- Project scoping MAY exist in data model, but a dedicated project-navigation
 pane is out of scope for this baseline and MUST NOT be assumed.

## Related

- Layout contract: [layout-and-interaction.md](layout-and-interaction.md)
- Workspaces domain: [/docs/spec/domain/workspaces.md](/docs/spec/domain/workspaces.md)
- Automation domain: [/docs/spec/domain/automation.md](/docs/spec/domain/automation.md)
