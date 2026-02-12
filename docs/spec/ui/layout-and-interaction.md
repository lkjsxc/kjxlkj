# Layout and Interaction Contract

Back: [/docs/spec/ui/README.md](/docs/spec/ui/README.md)

## Two-Pane Layout Rules

- The app MUST use one responsive layout model across desktop and mobile.
- Left pane (navigation/list) and right pane (editor/detail) MUST scroll independently.
- The right pane MUST allow inline editing of note title and note content.
- Bottom action chrome (save button/version bar) MAY exist for diagnostics but MUST NOT be required for normal authoring.

## Responsive Rules

- The same component tree MUST adapt by container size; separate mobile/desktop implementations are forbidden.
- Small screens MUST preserve independent vertical scrolling behavior for navigation and content regions.
- Touch targets MUST remain reachable without horizontal scrolling at widths down to 320px.
- Editor width and typography MUST adapt fluidly by viewport without mode switching.

## Visual Design Rules

- UI MUST follow modern flat design principles: low-noise surfaces, clear spacing rhythm, and restrained shadow use.
- Visual hierarchy MUST prioritize readability of notes and title editing over chrome.
- Interaction feedback (saving, sync, conflict) MUST be visible but unobtrusive.

## Related

- Web app shell: [web-app.md](web-app.md)
- Editor flow: [editor-flow.md](editor-flow.md)
