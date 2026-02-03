# Windows
Windows are viewports over buffers. They are core-owned layout state.

## Requirements

- Core owns the window tree / split layout.
- Rendering uses window snapshots; renderer never mutates layout.
- Cursor and viewport semantics are deterministic and mode-aware.

## Window model

A window is defined by:

- `WindowId` (stable)
- `BufferId` (what it shows)
- cursor state (per-window)
- viewport state (top line, dimensions)
- per-window options (wrap, numbers) as core state

## Layout

- Layout is represented as a tree (splits + leaf windows).
- UI regions (sidebar/bottom panels) are separate from the editor grid.

## Related

- UI views/components: [docs/spec/ui/README.md](/docs/spec/ui/README.md)
- Layout UX: [docs/spec/ux/layout.md](/docs/spec/ux/layout.md)
