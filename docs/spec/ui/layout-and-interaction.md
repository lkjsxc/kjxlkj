# Layout and Interaction Contract

Back: [/docs/spec/ui/README.md](/docs/spec/ui/README.md)

## Layout Rules

- The app MUST use one responsive component tree across desktop and mobile.
- Navigation/list and editor/content regions MUST support independent scrolling.
- Right pane MUST support inline title/content editing with predictable focus.
- Optional workspace modules MUST NOT crowd baseline note editing surfaces.

## Responsive Rules

- Separate mobile/desktop implementations are forbidden.
- Small screens MUST preserve independent vertical scrolling.
- On constrained screens, a menu toggle MUST collapse/restore navigation regions
  so editor can take primary space.
- On constrained screens, the menu toggle MUST be rendered in the top-right.
- Selecting a note from constrained-screen navigation MUST close the menu.
- Touch targets MUST be reachable without horizontal scrolling at widths down to
  `320px`.
- Typography and editor width MUST adapt fluidly without mode forks.

## Interaction Quality Rules

- Visual hierarchy MUST prioritize note content over chrome.
- Save/sync/conflict/automation state feedback MUST be visible and unobtrusive.
- Keyboard-first command palette flows SHOULD remain available in major views.
- Focus changes MUST be deterministic when panels are toggled.

## Findings Coverage

| Finding IDs | Required Outcome |
|---|---|
| `USR-003` | editing confidence via autosave-first interaction model |
| `USR-005` | constrained-screen collapse/restore behavior |
| `USR-006` | low-noise surface hierarchy with optional secondary modules |

## Related

- Web app shell: [web-app.md](web-app.md)
- Workspace suite: [workspace-suite.md](workspace-suite.md)
- Findings map: [findings-traceability.md](findings-traceability.md)
