# Cursor Customization

Cursor appearance and visibility rules.

## Scope

- The focused cursor MUST be visible at all times while the editor has focus.
- Cursor styling MUST be deterministic and derived from snapshots.
- Mouse-driven cursor interactions are out of scope; mouse input is ignored.

## Cursor shapes

| Shape | Meaning | Notes |
|-------|---------|-------|
| `block` | Cursor covers the cell at the cursor position. | Default for Normal. |
| `bar` | Cursor is a thin vertical bar at the cursor position. | Default for Insert. |
| `underline` | Cursor is a thin underline at the cursor position. | Default for Replace. |
| `hollow` | Cursor is an outlined block. | Default for Visual; MAY be emulated via styling if terminal lacks support. |

## Defaults per mode

| Mode | Default shape | Requirement |
|------|---------------|-------------|
| Normal | `block` | Distinct from selection and search highlights. |
| Insert | `bar` | Must remain visible at insertion points (end-inclusive model). |
| Visual | `hollow` | Selection highlight MUST remain visible under cursor styling. |
| Replace | `underline` | Must remain visible while overwriting. |
| Command | `bar` | Cursor tracks the command-line caret position. |

## Rendering contract (normative)

The cursor is a render-layer with the highest priority:

1. Content rendering (text, folds, conceal)
2. Highlight layers (selection, search, diagnostics)
3. Cursor rendering (primary cursor + secondary cursors)

Cursor rendering MUST NOT be overwritten by any later drawing operation.

## Terminal cursor vs drawn cursor

| Technique | Requirement |
|----------|-------------|
| Terminal cursor | The implementation SHOULD set the terminal cursor position and shape when supported. |
| Drawn cursor | The implementation MUST be able to render a visible cursor style in the cell grid. |

If the terminal cursor is unavailable or visually ambiguous under a theme, the drawn cursor MUST still ensure the cursor remains visible.

## Colors and contrast

| Rule | Requirement |
|------|-------------|
| Contrast | Cursor styling MUST remain visible under all built-in themes. |
| Fallback | If a configured cursor color is unreadable, the implementation MUST fall back to an invert or high-contrast style. |

## Blinking

| Setting | Default | Requirement |
|---------|---------|-------------|
| `ui.cursor.blink` | `true` | If disabled, cursor MUST be steady. |
| `ui.cursor.blink_rate_ms` | `500` | Must clamp to a safe range to avoid flicker. |

## Cursor line / column

| Feature | Setting | Requirement |
|---------|---------|-------------|
| Cursor line | `ui.cursorline` | When enabled, the focused line MUST be highlighted without obscuring cursor visibility. |
| Cursor column | `ui.cursorcolumn` | When enabled, the focused column MUST be highlighted without obscuring cursor visibility. |
| Crosshair | `ui.crosshair` | Equivalent to cursorline + cursorcolumn. |

## Virtual cursor (past end-of-line)

Cursor column semantics are defined in `/docs/spec/editing/cursor/README.md`.

Rendering requirements:

- In Insert mode (end-inclusive), an insertion point at column `N` MUST render as a cursor positioned immediately after the last grapheme on the line.
- If that position would fall outside the visible text area, the viewport MUST be adjusted per `/docs/spec/features/ui/viewport.md`.

## Multi-cursor

| Cursor | Requirement |
|--------|-------------|
| Primary | Uses the mode-specific cursor shape and participates in viewport follow. |
| Secondary | MUST be visible via a distinct highlight style and MUST NOT change the viewport. |

## Visibility invariants

- The cursor MUST remain visible across redraws, mode transitions, and overlays.
- If the UI focus moves to a non-editor view, the cursor MUST move to that view’s focus caret location.

## Related

- Cursor semantics: [/docs/spec/editing/cursor/README.md](/docs/spec/editing/cursor/README.md)
- Viewport follow and clamping: [/docs/spec/features/ui/viewport.md](/docs/spec/features/ui/viewport.md)
- Keybinding cursor indicators: [/docs/spec/ux/keybindings.md](/docs/spec/ux/keybindings.md)
