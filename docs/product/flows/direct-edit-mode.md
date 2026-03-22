# Direct Edit Mode Contract

This contract defines v1 direct-edit behavior for markdown authoring.

## Mode Model

- v1 direct editing MUST use deterministic split-view mode.
- Editor pane remains markdown-authoritative input.
- Preview pane remains server-rendered authoritative output.

## Split-View Requirements

- Editor pane ID: `#admin-editor-pane`.
- Preview pane ID: `#admin-preview-pane`.
- Optional synchronized navigation between panes MAY be provided.
- Mode toggle control MAY exist but must not break base form behavior.

## Interaction Rules

- Preview updates are triggered by:
  - explicit preview action
  - save response refresh
- Autosave, conflict handling, and unsaved-change guards remain active.
- Direct-edit enhancements MUST preserve keyboard shortcut contracts.

## Non-Admin Rule

- Non-admin users are read-only.
- No editing or suggestion submission surface is provided in v1.

## Cross-References

- Admin editor flow: [admin-editor.md](admin-editor.md)
- JavaScript UX contract: [admin-js-ux-contract.md](admin-js-ux-contract.md)
- Runtime interaction invariants: [../../architecture/runtime/ui-interaction-contract.md](../../architecture/runtime/ui-interaction-contract.md)
