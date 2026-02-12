# Layout and Interaction Contract

Back: [/docs/spec/ui/README.md](/docs/spec/ui/README.md)

## Layout Rules

- The app MUST use one responsive layout model across desktop and mobile.
- Left pane and right pane MUST scroll independently.
- The right pane MUST allow inline editing of note title and note content.
- Workspace-level surfaces (command palette, dashboards, graph explorer)
 MUST preserve the same component tree across breakpoints.

## Responsive Rules

- Separate mobile/desktop implementations are forbidden.
- Small screens MUST preserve independent vertical scrolling for navigation and
 content regions.
- Touch targets MUST remain reachable without horizontal scrolling at widths down
 to 320px.
- Editor width and typography MUST adapt fluidly by viewport without mode switching.

## Visual and Interaction Rules

- UI MUST follow modern flat design principles with low-noise surfaces.
- Visual hierarchy MUST prioritize content readability over chrome.
- Interaction feedback for save, sync, conflict, and automation status MUST be
 visible and unobtrusive.
- Keyboard-first flows via command palette SHOULD be available in all major views.

## Related

- Web app shell: [web-app.md](web-app.md)
- Workspace suite: [workspace-suite.md](workspace-suite.md)
- Editor flow: [editor-flow.md](editor-flow.md)
