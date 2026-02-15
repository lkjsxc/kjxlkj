# Layout and Interaction Contract

Back: [/docs/spec/ui/README.md](/docs/spec/ui/README.md)

## Layout Rules

- One responsive component tree MUST serve desktop and mobile.
- Navigation/list and editor/content regions MUST support independent scroll.
- Editor/content region MUST remain the visual priority.

## Menu Toggle Threshold Rule

The compact-menu behavior threshold MUST move earlier than the old 1024px split.

- Menu button mode MUST activate at approximately the final two-thirds width zone
  of common desktop ranges.
- Canonical activation breakpoint: `max-width: 1280px`.
- Above `1280px`, persistent split navigation MAY stay visible.
- At or below `1280px`, top-right menu toggle MUST control navigation visibility.

## Responsive Rules

- On compact widths, selecting a note MUST auto-close the menu.
- Touch targets MUST remain reachable at `320px` width without horizontal scroll.
- Focus transitions MUST remain deterministic when opening/closing panels.

## Interaction Quality Rules

- Visual hierarchy MUST prioritize markdown content over chrome.
- Save/sync/conflict/offline feedback MUST be visible and unobtrusive.
- Keyboard-first command palette flows SHOULD remain available.

## Related

- Editor flow: [editor-flow.md](editor-flow.md)
- UX requirements: [reconstruction-ux-requirements.md](reconstruction-ux-requirements.md)
- Findings map: [findings-traceability.md](findings-traceability.md)
