# Layout and Interaction Contract

Back: [/docs/spec/ui/README.md](/docs/spec/ui/README.md)

## Layout Rules

- The app MUST use one responsive component tree across desktop and mobile
  (`UX-LAYOUT-01`).
- At `>= 1024px`, note list MUST be rendered on the left and editor on the right
  (`UX-LAYOUT-05`).
- Navigation/list and editor/content regions MUST support independent scrolling
  when split-pane is active (`UX-LAYOUT-03`).
- Right pane MUST support inline title/content editing with predictable focus.
- Optional workspace modules MUST NOT crowd baseline note editing surfaces
  (`UX-NAV-01`).

## Responsive Rules

- Separate mobile/desktop implementations are forbidden.
- Small screens MUST preserve independent vertical scrolling where applicable.
- On constrained screens (`< 1024px`), editor MUST remain primary surface and a
  top-left menu button MUST collapse/restore note-list navigation (`UX-LAYOUT-02`,
  `UX-LAYOUT-06`).
- Menu-toggle state changes MUST preserve deterministic focus handoff to either
  note list (opened) or editor (closed) (`UX-A11Y-01`).
- Touch targets MUST be reachable without horizontal scrolling at widths down to
  `320px` (`UX-LAYOUT-04`).
- Typography and editor width MUST adapt fluidly without mode forks.

## Interaction Quality Rules

- Visual hierarchy MUST prioritize note content over chrome.
- Save/sync/conflict/automation state feedback MUST be visible and unobtrusive
  (`UX-FEEDBACK-01`).
- Keyboard-first command palette flows SHOULD remain available in major views
  (`UX-NAV-02`).
- Focus changes MUST be deterministic when panels are toggled (`UX-A11Y-01`).
- Async state changes SHOULD be announced via accessible status channels
  (`UX-A11Y-02`).

## Findings Coverage

| Finding IDs | Required Outcome |
|---|---|
| `USR-003` | editing confidence via autosave-first interaction model |
| `USR-005` | constrained-screen collapse/restore behavior and top-left menu reveal flow |
| `USR-006` | low-noise surface hierarchy with optional secondary modules |

## Related

- UX requirements: [reconstruction-ux-requirements.md](reconstruction-ux-requirements.md)
- Web app shell: [web-app.md](web-app.md)
- Workspace suite: [workspace-suite.md](workspace-suite.md)
- Findings map: [findings-traceability.md](findings-traceability.md)
