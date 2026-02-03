# Layout
Canonical UI system: [docs/spec/ui/README.md](/docs/spec/ui/README.md)

## Layout model

spec uses a snapshot-driven UI with a small number of persistent regions plus transient overlays.

- Sidebar: file explorer and other narrow views
- Main region: editor grid (tiled windows)
- Bottom region: command line + statusline + optional list panel
- Overlays: pickers, hovers, completion, confirmations

## Constraints

- Keyboard-only interaction; mouse is ignored.
- Overlays must be instantly cancellable.
- Rendering must be driven by immutable snapshots from core.

## Related

- UX hub: [docs/spec/ux/README.md](/docs/spec/ux/README.md)
- Statusline: [docs/spec/features/ui/statusline/README.md](/docs/spec/features/ui/statusline/README.md)
